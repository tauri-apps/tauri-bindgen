use proc_macro2::TokenStream;
use std::{collections::HashMap, ops::Index, path::PathBuf};
use wit_parser::{
    FlagsField, Function, FunctionResult, Int, Interface, Type, TypeDefArena, TypeDefId,
    TypeDefKind, UnionCase,
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
            log::debug!("type info: {} {:#?}", typedef.id, this[id]);
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

        log::debug!("collected info for {:?}: {:?}", typedefs[id].id, info,);

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

fn type_ident(typedefs: &TypeDefArena, ty: &Type) -> String {
    match ty {
        Type::Bool => "Bool".to_string(),
        Type::U8 => "U8".to_string(),
        Type::U16 => "U16".to_string(),
        Type::U32 => "U32".to_string(),
        Type::U64 => "U64".to_string(),
        Type::U128 => "U128".to_string(),
        Type::S8 => "I8".to_string(),
        Type::S16 => "I16".to_string(),
        Type::S32 => "S32".to_string(),
        Type::S64 => "S64".to_string(),
        Type::S128 => "S128".to_string(),
        Type::Float32 => "F32".to_string(),
        Type::Float64 => "F64".to_string(),
        Type::Char => "Char".to_string(),
        Type::String => "String".to_string(),
        Type::List(ty) => format!("{}List", type_ident(typedefs, ty)),
        Type::Tuple(_) => "Tuple".to_string(),
        Type::Option(ty) => format!("Optional{}", type_ident(typedefs, ty)),
        Type::Result { .. } => "Result".to_string(),
        Type::Id(id) => match &typedefs[*id].kind {
            TypeDefKind::Alias(ty) => type_ident(typedefs, ty),
            TypeDefKind::Record(_) => "Record".to_string(),
            TypeDefKind::Flags(_) => "Flags".to_string(),
            TypeDefKind::Variant(_) => "Variant".to_string(),
            TypeDefKind::Enum(_) => "Enum".to_string(),
            TypeDefKind::Union(_) => "Union".to_string(),
            TypeDefKind::Resource(_) => "Resource".to_string(),
        },
    }
}

#[must_use]
pub fn union_case_names(typedefs: &TypeDefArena, cases: &[UnionCase]) -> Vec<String> {
    enum UsedState<'a> {
        /// This name has been used once before.
        ///
        /// Contains a reference to the name given to the first usage so that a suffix can be added to it.
        Once(&'a mut String),
        /// This name has already been used multiple times.
        ///
        /// Contains the number of times this has already been used.
        Multiple(usize),
    }

    // A `Vec` of the names we're assigning each of the union's cases in order.
    let mut case_names = vec![String::new(); cases.len()];
    // A map from case names to their `UsedState`.
    let mut used = HashMap::new();
    for (case, name) in cases.iter().zip(case_names.iter_mut()) {
        name.push_str(&type_ident(typedefs, &case.ty));

        match used.get_mut(name.as_str()) {
            None => {
                // Initialise this name's `UsedState`, with a mutable reference to this name
                // in case we have to add a suffix to it later.
                used.insert(name.clone(), UsedState::Once(name));
                // Since this is the first (and potentially only) usage of this name,
                // we don't need to add a suffix here.
            }
            Some(state) => match state {
                UsedState::Multiple(n) => {
                    // Add a suffix of the index of this usage.
                    name.push_str(&n.to_string());
                    // Add one to the number of times this type has been used.
                    *n += 1;
                }
                UsedState::Once(first) => {
                    // Add a suffix of 0 to the first usage.
                    first.push('0');
                    // We now get a suffix of 1.
                    name.push('1');
                    // Then update the state.
                    *state = UsedState::Multiple(2);
                }
            },
        }
    }

    case_names
}
