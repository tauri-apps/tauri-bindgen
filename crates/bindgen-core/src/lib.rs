pub mod hash;
mod postprocess;

use miette::Diagnostic;
pub use postprocess::postprocess;

use std::{
    collections::{btree_map::Entry, BTreeMap, HashMap},
    fmt::{self, Write},
    ops::Deref,
    sync::Arc,
};
use wit_parser::*;

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(code(my_lib::io_error))]
    IoError(#[from] std::io::Error),
}

pub const VERSION_MISMATCH_MSG: &str = "The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.";

pub trait WorldGenerator {
    fn generate(&mut self, name: &str, interface: &Interface, files: &mut Files, world_hash: &str) {
        self.import(name, interface, files, world_hash);
        self.finish(name, files, world_hash);
    }

    fn import(&mut self, name: &str, iface: &Interface, files: &mut Files, world_hash: &str);

    fn finish(&mut self, name: &str, files: &mut Files, world_hash: &str);
}

pub trait InterfaceGenerator<'a> {
    fn iface(&self) -> &'a Interface;

    fn type_record(&mut self, ty: &Arc<TypeDef>, name: &str, record: &Record, docs: &Docs);
    fn type_flags(&mut self, ty: &Arc<TypeDef>, name: &str, flags: &Flags, docs: &Docs);
    fn type_variant(&mut self, ty: &Arc<TypeDef>, name: &str, variant: &Variant, docs: &Docs);
    fn type_union(&mut self, ty: &Arc<TypeDef>, name: &str, union: &Union, docs: &Docs);
    fn type_enum(&mut self, ty: &Arc<TypeDef>, name: &str, enum_: &Enum, docs: &Docs);
    fn type_alias(&mut self, ty: &Arc<TypeDef>, name: &str, ty: &Type, docs: &Docs);
    fn type_resource(&mut self, ty: &Arc<TypeDef>, name: &str, ty: &Resource, docs: &Docs);

    fn types(&mut self) {
        for ty in self.iface().types.values() {
            dbg!("in types", &ty.kind);
            match &ty.kind {
                TypeDefKind::Record(record) => self.type_record(ty, &ty.name, record, &ty.docs),
                TypeDefKind::Flags(flags) => self.type_flags(ty, &ty.name, flags, &ty.docs),
                TypeDefKind::Enum(enum_) => self.type_enum(ty, &ty.name, enum_, &ty.docs),
                TypeDefKind::Variant(variant) => self.type_variant(ty, &ty.name, variant, &ty.docs),
                TypeDefKind::Union(u) => self.type_union(ty, &ty.name, u, &ty.docs),
                TypeDefKind::Type(t) => self.type_alias(ty, &ty.name, t, &ty.docs),
                TypeDefKind::Resource(resource) => {
                    self.type_resource(ty, &ty.name, resource, &ty.docs)
                }
            }
        }
    }
}

// impl Types {
pub fn analyze(iface: &mut Interface) {
    for (_, ty) in iface.types.iter_mut() {
        type_id_info(ty);
    }
    for f in iface.functions.iter_mut() {
        for (_, ty) in f.params.iter_mut() {
            set_param_result_ty(ty, TypeInfo::PARAM);
        }
        for ty in f.results.types_mut() {
            set_param_result_ty(ty, TypeInfo::RESULT);
        }
    }

    for (_, ty) in iface.types.iter_mut() {
        let ty = Arc::make_mut(ty);
        if let TypeDefKind::Resource(r) = &mut ty.kind {
            for m in r.methods.iter_mut() {
                for (_, ty) in m.params.iter_mut() {
                    set_param_result_ty(ty, TypeInfo::PARAM);
                }
                for ty in m.results.types_mut() {
                    set_param_result_ty(ty, TypeInfo::RESULT);
                }
            }
        }
    }
}

fn type_id_info(ty: &mut Arc<TypeDef>) -> TypeInfo {
    let ty = Arc::make_mut(ty);

    let mut info = TypeInfo::empty();
    match &mut ty.kind {
        TypeDefKind::Record(r) => {
            for field in r.fields.iter_mut() {
                info |= type_info(&mut field.ty);
            }
        }
        TypeDefKind::Variant(v) => {
            for case in v.cases.iter_mut() {
                info |= optional_type_info(case.ty.as_mut());
            }
        }
        TypeDefKind::Union(u) => {
            for case in u.cases.iter_mut() {
                info |= type_info(&mut case.ty);
            }
        }
        TypeDefKind::Type(ty) => {
            info = type_info(ty);
        }
        TypeDefKind::Flags(_) => {}
        TypeDefKind::Enum(_) => {}
        TypeDefKind::Resource(r) => {
            let has_statics = r.methods.iter().any(|m| m.static_);

            if has_statics {
                info |= TypeInfo::HAS_STATICS;
            }
        }
    };

    ty.info |= info;
    info
}

fn type_info(ty: &mut Type) -> TypeInfo {
    let mut info = TypeInfo::empty();
    match ty {
        Type::String => info |= TypeInfo::HAS_LIST,
        Type::Tuple(ty) => {
            for ty in ty.types.iter_mut() {
                info |= type_info(ty);
            }
        }
        Type::List(ty) => {
            info = type_info(ty);
            info |= TypeInfo::HAS_LIST;
        }
        Type::Option(ty) => {
            info = type_info(ty);
        }
        Type::Result(res) => {
            info = optional_type_info(res.ok.as_mut());
            info |= optional_type_info(res.err.as_mut());
        }
        Type::Id(id) => return type_id_info(id),
        _ => {}
    }
    info
}

fn optional_type_info(ty: Option<&mut Type>) -> TypeInfo {
    match ty {
        Some(ty) => type_info(ty),
        None => TypeInfo::empty(),
    }
}

fn set_param_result_id(ty: &mut Arc<TypeDef>, new_info: TypeInfo) {
    let ty = Arc::make_mut(ty);
    match &mut ty.kind {
        TypeDefKind::Record(r) => {
            for field in r.fields.iter_mut() {
                set_param_result_ty(&mut field.ty, new_info)
            }
        }
        TypeDefKind::Flags(_) => {}
        TypeDefKind::Enum(_) => {}
        TypeDefKind::Variant(v) => {
            for case in v.cases.iter_mut() {
                set_optional_param_result_ty(case.ty.as_mut(), new_info)
            }
        }
        TypeDefKind::Union(u) => {
            for case in u.cases.iter_mut() {
                set_param_result_ty(&mut case.ty, new_info)
            }
        }
        TypeDefKind::Type(ty) => set_param_result_ty(ty, new_info),
        TypeDefKind::Resource(_) => {}
    }
}

pub fn set_param_result_ty(ty: &mut Type, new_info: TypeInfo) {
    match ty {
        Type::Tuple(ty) => {
            for ty in ty.types.iter_mut() {
                set_param_result_ty(ty, new_info)
            }
        }
        Type::List(ty) | Type::Option(ty) => set_param_result_ty(ty, new_info),
        Type::Result(res) => {
            set_optional_param_result_ty(res.ok.as_mut(), new_info);
            set_optional_param_result_ty(res.err.as_mut(), new_info);
        }
        Type::Id(id) => {
            let info = &mut Arc::make_mut(id).info;
            // type_id_info(iface, *id);
            // let info = type_info.get_mut(id).unwrap();

            if !info.symmetric_difference(new_info).is_empty() {
                *info |= new_info;
                set_param_result_id(id, new_info);
            }
        }
        _ => {}
    }
}

fn set_optional_param_result_ty(ty: Option<&mut Type>, new_info: TypeInfo) {
    if let Some(ty) = ty {
        set_param_result_ty(ty, new_info)
    }
}

#[derive(Debug, Default)]
pub struct Files {
    files: BTreeMap<String, Vec<u8>>,
}

impl Files {
    pub fn push(&mut self, name: &str, contents: &[u8]) {
        match self.files.entry(name.to_owned()) {
            Entry::Vacant(entry) => {
                entry.insert(contents.to_owned());
            }
            Entry::Occupied(ref mut entry) => {
                entry.get_mut().extend_from_slice(contents);
            }
        }
    }

    pub fn get_size(&mut self, name: &str) -> Option<usize> {
        self.files.get(name).map(|data| data.len())
    }

    pub fn remove(&mut self, name: &str) -> Option<Vec<u8>> {
        self.files.remove(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'_ str, &'_ [u8])> {
        self.files.iter().map(|p| (p.0.as_str(), p.1.as_slice()))
    }
}

impl IntoIterator for Files {
    type Item = (String, Vec<u8>);

    type IntoIter = std::collections::btree_map::IntoIter<String, Vec<u8>>;

    fn into_iter(self) -> Self::IntoIter {
        self.files.into_iter()
    }
}

#[derive(Debug, Default)]
pub struct Source {
    s: String,
    indent: usize,
}

impl Source {
    pub fn push_str(&mut self, src: &str) {
        let lines = src.lines().collect::<Vec<_>>();
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with('}') && self.s.ends_with("  ") {
                self.s.pop();
                self.s.pop();
            }
            self.s.push_str(if lines.len() == 1 {
                line
            } else {
                line.trim_start()
            });
            if trimmed.ends_with('{') {
                self.indent += 1;
            }
            if trimmed.starts_with('}') {
                // Note that a `saturating_sub` is used here to prevent a panic
                // here in the case of invalid code being generated in debug
                // mode. It's typically easier to debug those issues through
                // looking at the source code rather than getting a panic.
                self.indent = self.indent.saturating_sub(1);
            }
            if i != lines.len() - 1 || src.ends_with('\n') {
                self.newline();
            }
        }
    }

    pub fn indent(&mut self, amt: usize) {
        self.indent += amt;
    }

    pub fn deindent(&mut self, amt: usize) {
        self.indent -= amt;
    }

    fn newline(&mut self) {
        self.s.push('\n');
        for _ in 0..self.indent {
            self.s.push_str("  ");
        }
    }

    pub fn as_mut_string(&mut self) -> &mut String {
        &mut self.s
    }
}

impl Write for Source {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }
}

impl Deref for Source {
    type Target = str;
    fn deref(&self) -> &str {
        &self.s
    }
}

impl From<Source> for String {
    fn from(s: Source) -> String {
        s.s
    }
}

/// Calls [`write!`] with the passed arguments and unwraps the result.
///
/// Useful for writing to things with infallible `Write` implementations like
/// `Source` and `String`.
///
/// [`write!`]: std::write
#[macro_export]
macro_rules! uwrite {
    ($dst:expr, $($arg:tt)*) => {
        write!($dst, $($arg)*).unwrap()
    };
}

/// Calls [`writeln!`] with the passed arguments and unwraps the result.
///
/// Useful for writing to things with infallible `Write` implementations like
/// `Source` and `String`.
///
/// [`writeln!`]: std::writeln
#[macro_export]
macro_rules! uwriteln {
    ($dst:expr, $($arg:tt)*) => {
        writeln!($dst, $($arg)*).unwrap()
    };
}
