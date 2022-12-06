pub mod hash;
mod postprocess;

use miette::Diagnostic;
pub use postprocess::postprocess;

use std::{
    collections::{btree_map::Entry, BTreeMap, HashMap},
    fmt::{self, Write},
    ops::Deref,
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

    fn type_record(&mut self, id: TypeId, name: &str, record: &Record, docs: &Docs);
    fn type_flags(&mut self, id: TypeId, name: &str, flags: &Flags, docs: &Docs);
    fn type_variant(&mut self, id: TypeId, name: &str, variant: &Variant, docs: &Docs);
    fn type_union(&mut self, id: TypeId, name: &str, union: &Union, docs: &Docs);
    fn type_enum(&mut self, id: TypeId, name: &str, enum_: &Enum, docs: &Docs);
    fn type_alias(&mut self, id: TypeId, name: &str, ty: &Type, docs: &Docs);

    fn types(&mut self) {
        for (id, ty) in self.iface().types.iter() {
            match &ty.kind {
                TypeDefKind::Record(record) => self.type_record(id, &ty.name, record, &ty.docs),
                TypeDefKind::Flags(flags) => self.type_flags(id, &ty.name, flags, &ty.docs),
                TypeDefKind::Enum(enum_) => self.type_enum(id, &ty.name, enum_, &ty.docs),
                TypeDefKind::Variant(variant) => self.type_variant(id, &ty.name, variant, &ty.docs),
                TypeDefKind::Union(u) => self.type_union(id, &ty.name, u, &ty.docs),
                TypeDefKind::Type(t) => self.type_alias(id, &ty.name, t, &ty.docs),
            }
        }
    }
}

bitflags::bitflags! {
    pub struct TypeInfo: u32 {
        /// Whether or not this type is ever used (transitively) within the
        /// parameter of a function.
        const PARAM = 0b00000001;
        /// Whether or not this type is ever used (transitively) within the
        /// result of a function.
        const RESULT = 0b00000010;
        /// Whether or not this type is ever used (transitively) within the
        /// error case in the result of a function.
        const ERROR = 0b00000100;
        /// Whether or not this type (transitively) has a list.
        const HAS_LIST = 0b00001000;

        const PARAM_AND_RESULT = Self::PARAM.bits | Self::RESULT.bits;
    }
}

impl TypeInfo {
    pub fn owns_data(&self) -> bool {
        self.contains(TypeInfo::HAS_LIST)
    }
}

#[derive(Debug, Default)]
pub struct Types {
    type_info: HashMap<TypeId, TypeInfo>,
}

impl Types {
    pub fn analyze(&mut self, iface: &Interface) {
        for (t, _) in iface.types.iter() {
            self.type_id_info(iface, t);
        }
        for f in iface.functions.iter() {
            for (_, ty) in f.params.iter() {
                self.set_param_result_ty(iface, ty, TypeInfo::PARAM);
            }
            for ty in f.results.types() {
                self.set_param_result_ty(iface, ty, TypeInfo::RESULT);
            }
        }
    }

    pub fn get(&self, id: TypeId) -> TypeInfo {
        self.type_info[&id]
    }

    fn type_id_info(&mut self, iface: &Interface, id: TypeId) -> TypeInfo {
        if let Some(info) = self.type_info.get(&id) {
            return *info;
        }

        let mut info = TypeInfo::empty();
        match &iface.types[id].kind {
            TypeDefKind::Record(r) => {
                for field in r.fields.iter() {
                    info |= self.type_info(iface, &field.ty);
                }
            }
            TypeDefKind::Variant(v) => {
                for case in v.cases.iter() {
                    info |= self.optional_type_info(iface, case.ty.as_ref());
                }
            }
            TypeDefKind::Union(u) => {
                for case in u.cases.iter() {
                    info |= self.type_info(iface, &case.ty);
                }
            }
            TypeDefKind::Type(ty) => {
                info = self.type_info(iface, ty);
            }
            TypeDefKind::Flags(_) => {}
            TypeDefKind::Enum(_) => {}
        }
        self.type_info.insert(id, info);
        info
    }

    fn type_info(&mut self, iface: &Interface, ty: &Type) -> TypeInfo {
        let mut info = TypeInfo::empty();
        match ty {
            Type::String => info |= TypeInfo::HAS_LIST,
            Type::Tuple(ty) => {
                for ty in ty.types.iter() {
                    info |= self.type_info(iface, ty);
                }
            }
            Type::List(ty) => {
                info = self.type_info(iface, ty);
                info |= TypeInfo::HAS_LIST;
            }
            Type::Option(ty) => {
                info = self.type_info(iface, ty);
            }
            Type::Result(res) => {
                info = self.optional_type_info(iface, res.ok.as_ref());
                info |= self.optional_type_info(iface, res.err.as_ref());
            }
            Type::Id(id) => return self.type_id_info(iface, *id),
            _ => {}
        }
        info
    }

    fn optional_type_info(&mut self, iface: &Interface, ty: Option<&Type>) -> TypeInfo {
        match ty {
            Some(ty) => self.type_info(iface, ty),
            None => TypeInfo::empty(),
        }
    }

    fn set_param_result_id(&mut self, iface: &Interface, ty: TypeId, new_info: TypeInfo) {
        match &iface.types[ty].kind {
            TypeDefKind::Record(r) => {
                for field in r.fields.iter() {
                    self.set_param_result_ty(iface, &field.ty, new_info)
                }
            }
            TypeDefKind::Flags(_) => {}
            TypeDefKind::Enum(_) => {}
            TypeDefKind::Variant(v) => {
                for case in v.cases.iter() {
                    self.set_optional_param_result_ty(iface, case.ty.as_ref(), new_info)
                }
            }
            TypeDefKind::Union(u) => {
                for case in u.cases.iter() {
                    self.set_param_result_ty(iface, &case.ty, new_info)
                }
            }
            TypeDefKind::Type(ty) => self.set_param_result_ty(iface, ty, new_info),
        }
    }

    pub fn set_param_result_ty(&mut self, iface: &Interface, ty: &Type, new_info: TypeInfo) {
        match ty {
            Type::Tuple(ty) => {
                for ty in ty.types.iter() {
                    self.set_param_result_ty(iface, ty, new_info)
                }
            }
            Type::List(ty) | Type::Option(ty) => self.set_param_result_ty(iface, ty, new_info),
            Type::Result(res) => {
                self.set_optional_param_result_ty(iface, res.ok.as_ref(), new_info);
                self.set_optional_param_result_ty(iface, res.err.as_ref(), new_info);
            }
            Type::Id(id) => {
                self.type_id_info(iface, *id);
                let info = self.type_info.get_mut(id).unwrap();

                if !info.symmetric_difference(new_info).is_empty() {
                    *info = *info | new_info;
                    self.set_param_result_id(iface, *id, new_info);
                }
            }
            _ => {}
        }
    }

    fn set_optional_param_result_ty(
        &mut self,
        iface: &Interface,
        ty: Option<&Type>,
        new_info: TypeInfo,
    ) {
        if let Some(ty) = ty {
            self.set_param_result_ty(iface, ty, new_info)
        }
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
        match self.files.get(name) {
            Some(data) => Some(data.len()),
            None => None,
        }
    }

    pub fn remove(&mut self, name: &str) -> Option<Vec<u8>> {
        self.files.remove(name);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&'_ str, &'_ [u8])> {
        self.files.iter().map(|p| (p.0.as_str(), p.1.as_slice()))
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
