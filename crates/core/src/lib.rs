use proc_macro2::TokenStream;
use std::{collections::HashMap, ops::Index, path::PathBuf};
use wit_parser::{
    FlagsField, Function, FunctionResult, Int, Interface, Type, TypeDefArena, TypeDefId,
    TypeDefKind,
};

pub trait GeneratorBuilder {
    fn build(self, interface: Interface) -> Box<dyn Generate>;
}

pub trait Generate {
    fn to_file(&mut self) -> (PathBuf, String);
    fn to_tokens(&mut self) -> TokenStream {
        unimplemented!("to_tokens is not implemented for this generator")
    }
}

use std::{
    ffi::OsStr,
    io::{Read, Write},
    process::{Command, Stdio},
};

/// # Errors
///
/// Returns an error when the underlying postprocess command didn't finish successfully
///
/// # Panics
///
/// Attempts to take the stdin and stdout pipes from the spawned child, will panic otherwise
pub fn postprocess<I, S>(
    file: &mut String,
    cmd: impl AsRef<OsStr>,
    args: I,
) -> Result<(), Box<dyn std::error::Error>>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut child = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    child.stdin.take().unwrap().write_all(file.as_bytes())?;
    file.truncate(0);
    child.stdout.take().unwrap().read_to_string(file)?;
    let status = child.wait()?;
    assert!(status.success());

    Ok(())
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct TypeInfo: u32 {
        /// Whether or not this type is ever used (transitively) within the
        /// parameter of a function.
        const PARAM = 0b0000_0001;
        /// Whether or not this type is ever used (transitively) within the
        /// result of a function.
        const RESULT = 0b0000_0010;
        /// Whether or not this type (transitively) has a list.
        const HAS_LIST = 0b0000_1000;
    }
}

#[derive(Debug, Default)]
pub struct TypeInfos {
    infos: HashMap<TypeDefId, TypeInfo>,
}

impl TypeInfos {
    #[must_use]
    pub fn collect_from_functions(typedefs: &TypeDefArena, functions: &[Function]) -> Self {
        let mut this = Self::default();

        for func in functions {
            for (_, ty) in &func.params {
                this.collect_type_info(typedefs, ty, TypeInfo::PARAM);
            }

            match &func.result {
                Some(FunctionResult::Anon(ty)) => {
                    this.collect_type_info(typedefs, ty, TypeInfo::RESULT);
                }
                Some(FunctionResult::Named(results)) => {
                    for (_, ty) in results {
                        this.collect_type_info(typedefs, ty, TypeInfo::RESULT);
                    }
                }
                None => {}
            }
        }

        for (id, typedef) in typedefs {
            log::debug!("type info: {} {:#?}", typedef.ident, this[id]);
        }

        this
    }

    fn collect_typedef_info(
        &mut self,
        typedefs: &TypeDefArena,
        id: TypeDefId,
        base_info: TypeInfo,
    ) -> TypeInfo {
        let mut info = base_info;

        match &typedefs[id].kind {
            TypeDefKind::Alias(ty) => {
                info |= self.collect_type_info(typedefs, ty, base_info);
            }
            TypeDefKind::Record(fields) => {
                for field in fields {
                    info |= self.collect_type_info(typedefs, &field.ty, base_info);
                }
            }
            TypeDefKind::Variant(cases) => {
                for case in cases {
                    if let Some(ty) = &case.ty {
                        info |= self.collect_type_info(typedefs, ty, base_info);
                    }
                }
            }
            TypeDefKind::Union(cases) => {
                for case in cases {
                    info |= self.collect_type_info(typedefs, &case.ty, base_info);
                }
            }
            _ => {}
        }

        log::debug!("collected info for {:?}: {:?}", typedefs[id].ident, info,);

        self.infos
            .entry(id)
            .and_modify(|i| *i |= info)
            .or_insert(info);

        info
    }

    fn collect_type_info(
        &mut self,
        typedefs: &TypeDefArena,
        ty: &Type,
        base_info: TypeInfo,
    ) -> TypeInfo {
        match ty {
            Type::String => base_info | TypeInfo::HAS_LIST,
            Type::List(ty) => self.collect_type_info(typedefs, ty, base_info) | TypeInfo::HAS_LIST,
            Type::Option(ty) => self.collect_type_info(typedefs, ty, base_info),
            Type::Tuple(types) => {
                let mut info = base_info;
                for ty in types {
                    info |= self.collect_type_info(typedefs, ty, base_info);
                }
                info
            }
            Type::Result { ok, err } => {
                let mut info = base_info;
                if let Some(ty) = &ok {
                    info |= self.collect_type_info(typedefs, ty, base_info);
                }
                if let Some(ty) = &err {
                    info |= self.collect_type_info(typedefs, ty, base_info);
                }
                info
            }
            Type::Id(id) => base_info | self.collect_typedef_info(typedefs, *id, base_info),
            _ => base_info,
        }
    }
}

impl Index<TypeDefId> for TypeInfos {
    type Output = TypeInfo;

    fn index(&self, id: TypeDefId) -> &Self::Output {
        &self.infos[&id]
    }
}

/// # Panics
///
/// Panics if the number of flags field is larger than 64
#[must_use]
pub fn flags_repr(fields: &[FlagsField]) -> Int {
    match fields.len() {
        n if n <= 8 => Int::U8,
        n if n <= 16 => Int::U16,
        n if n <= 32 => Int::U32,
        n if n <= 64 => Int::U64,
        _ => panic!("too many flags to fit in a repr"),
    }
}
