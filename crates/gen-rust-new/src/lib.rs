use heck::{ToSnakeCase, ToUpperCamelCase};
use std::collections::HashMap;
use std::fmt::Write;
use wit_parser::{Docs, Type, TypeDef, TypeDefKind, TypeInfo, Union, Int};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TypeMode {
    Owned,
    AllBorrowed(&'static str),
    LeafBorrowed(&'static str),
}

pub trait ToRust {
    fn write_rust(&self, f: &mut core::fmt::Formatter<'_>, mode: TypeMode) -> core::fmt::Result;
}

impl ToRust for Type {
    fn write_rust(&self, f: &mut std::fmt::Formatter<'_>, mode: TypeMode) -> std::fmt::Result {
        match self {
            Type::U8 => f.write_str("u8")?,
            Type::U16 => f.write_str("u16")?,
            Type::U32 => f.write_str("u32")?,
            Type::U64 => f.write_str("u64")?,
            Type::S8 => f.write_str("i8")?,
            Type::S16 => f.write_str("i16")?,
            Type::S32 => f.write_str("i32")?,
            Type::S64 => f.write_str("i64")?,
            Type::Float32 => f.write_str("f32")?,
            Type::Float64 => f.write_str("f64")?,
            Type::Bool => f.write_str("bool")?,
            Type::Char => f.write_str("char")?,
            Type::String => match mode {
                TypeMode::Owned => f.write_str("String")?,
                TypeMode::AllBorrowed(lt) | TypeMode::LeafBorrowed(lt) => {
                    print_borrowed_str(f, lt)?;
                }
            },
            Type::Tuple(ty) => {
                f.write_str("(")?;
                for ty in ty.types.iter() {
                    ty.write_rust(f, mode)?;
                    f.write_str(",")?;
                }
                f.write_str(")")?;
            }
            Type::List(ty) => match mode {
                TypeMode::Owned => {
                    f.write_str("Vec<")?;
                    ty.write_rust(f, mode)?;
                    f.write_str(">")?;
                }
                TypeMode::AllBorrowed(lt) => print_rust_slice(f, false, ty, lt)?,
                TypeMode::LeafBorrowed(_) => todo!(),
            },
            Type::Option(ty) => {
                f.write_str("Option<")?;
                ty.write_rust(f, mode)?;
                f.write_str(">")?;
            }
            Type::Result(r) => {
                f.write_str("Result<")?;
                r.ok.write_rust(f, mode)?;
                f.write_str(",")?;
                r.err.write_rust(f, mode)?;
                f.write_str(">")?;
            }
            Type::Id(ptr) => ptr.write_rust(f, mode)?,
        }

        Ok(())
    }
}

impl<T: ToRust> ToRust for Option<T> {
    fn write_rust(&self, f: &mut core::fmt::Formatter<'_>, mode: TypeMode) -> core::fmt::Result {
        match self {
            Some(ty) => ty.write_rust(f, mode),
            None => f.write_str("()"),
        }
    }
}

impl ToRust for TypeDef {
    fn write_rust(&self, f: &mut core::fmt::Formatter<'_>, mode: TypeMode) -> core::fmt::Result {
        for (name, mode) in modes_of(&self, mode) {
            match &self.kind {
                TypeDefKind::Type(ty) => {
                    let lt = lifetime_for(self, mode);

                    self.docs.write_rust(f, mode)?;
                    write!(f, "pub type {} ", name)?;
                    print_generics(f, lt)?;
                    write!(f, "= ")?;
                    ty.write_rust(f, mode)?;
                    writeln!(f, ";")?;
                }

                TypeDefKind::Record(record) => {
                    let lt = lifetime_for(&self, mode);
                    self.docs.write_rust(f, mode)?;

                    if !self.info.owns_data() {
                        write!(f, "#[repr(C)]\n")?;
                        write!(f, "#[derive(Debug, Copy, Clone, PartialEq)]\n")?;
                    } else {
                        write!(f, "#[derive(Debug, Clone, PartialEq)]\n")?;
                    }
                    // if let Some(attrs) = attrs(&name, self.uses_two_names(&ty.info), ty.info) {
                    //     write!(f, &attrs);
                    // }
                    write!(f, "#[serde(rename_all = \"camelCase\")]\n")?;
                    write!(f, "pub struct {}", name)?;
                    print_generics(f, lt)?;
                    writeln!(f, " {{")?;
                    
                    for field in record.fields.iter() {
                        if needs_borrow(&field.ty, mode) {
                            writeln!(f, "#[serde(borrow)]")?;
                        }
                        field.docs.write_rust(f, mode)?;
                        write!(f, "pub ")?;
                        write!(f, "{}", &to_rust_ident(&field.name))?;
                        write!(f, ": ")?;
                        field.ty.write_rust(f, mode)?;
                        writeln!(f, ",")?;
                    }

                    writeln!(f, "}}")?;
                }

                TypeDefKind::Flags(_) => todo!(),

                TypeDefKind::Variant(variant) => {
                    print_rust_enum(
                        f,
                        self,
                        mode,
                        variant
                            .cases
                            .iter()
                            .map(|c| (c.name.to_upper_camel_case(), &c.docs, c.ty.as_ref())),
                    )?;
                }
                TypeDefKind::Union(union) => {
                    print_rust_enum(
                        f,
                        self,
                        mode,
                        union_case_names(union)
                            .into_iter()
                            .zip(&union.cases)
                            .map(|(name, case)| (name, &case.docs, Some(&case.ty))),
                    )?;
                }
                TypeDefKind::Enum(enum_) => {
                    let lt = lifetime_for(&self, mode);

                    self.docs.write_rust(f, mode)?;
                    writeln!(f, "#[repr({})]", int_repr(enum_.tag()))?;
                    if !self.info.owns_data() {
                        writeln!(f, "#[derive(Debug, Clone, Copy, PartialEq)]\n")?;
                    } else {
                        writeln!(f, "#[derive(Debug, Clone, PartialEq)]")?;
                    }
                    // if let Some(attrs) = attrs(&name, uses_two_names(&ty.info), ty.info) {
                    //     write!(f, &attrs);
                    // }
                    write!(f, "pub enum {name}")?;
                    print_generics(f, lt)?;
                    writeln!(f, "{{")?;

                    for case in enum_.cases.iter() {
                        case.docs.write_rust(f, mode)?;
                        writeln!(f, "{},", case.name.to_upper_camel_case())?;
                    }

                    writeln!(f, "}}")?;
                }

                TypeDefKind::Resource(_) => todo!(),
            }
        }

        Ok(())
    }
}

fn print_rust_enum<'a>(
    f: &mut core::fmt::Formatter<'_>,
    ty: &TypeDef,
    mode: TypeMode,
    cases: impl IntoIterator<Item = (String, &'a Docs, Option<&'a Type>)> + Clone,
) -> core::fmt::Result {
    let lt = lifetime_for(ty, mode);
    let name = ty.name.to_upper_camel_case();

    ty.docs.write_rust(f, mode)?;
    if !ty.info.owns_data() {
        writeln!(f, "#[derive(Debug, Clone, Copy, PartialEq)]")?;
    } else {
        writeln!(f, "#[derive(Debug, Clone, PartialEq)]")?;
    }
    // if let Some(attrs) = attrs(&name, uses_two_names(&ty.info, mode), ty.info) {
    //     write!(f, &attrs);
    // }
    write!(f, "pub enum {name}")?;
    print_generics(f, lt)?;
    writeln!(f, "{{")?;

    for (case_name, docs, payload) in cases.clone() {
        docs.write_rust(f, mode)?;
        write!(f, "{}", case_name)?;

        if let Some(payload) = payload {
            write!(f, "(")?;
            payload.write_rust(f, mode)?;
            write!(f, ")")?;
        }

        writeln!(f, ",")?;
    }
    writeln!(f, "}}")?;

    Ok(())
}

impl ToRust for Docs {
    fn write_rust(&self, f: &mut core::fmt::Formatter<'_>, _mode: TypeMode) -> core::fmt::Result {
        for line in self.contents.trim().lines() {
            writeln!(f, "/// {line}")?;
        }

        Ok(())
    }
}

fn print_borrowed_str(
    f: &mut core::fmt::Formatter<'_>,
    lifetime: &'static str,
) -> core::fmt::Result {
    f.write_str("&")?;
    if lifetime != "'_" {
        f.write_str(lifetime)?;
        f.write_str(" ")?;
    }
    f.write_str("str")?;

    Ok(())
}

fn print_rust_slice(
    f: &mut core::fmt::Formatter<'_>,
    mutbl: bool,
    ty: &Type,
    lifetime: &'static str,
) -> core::fmt::Result {
    f.write_str("&")?;
    if lifetime != "'_" {
        f.write_str(lifetime)?;
        f.write_str(" ")?;
    }
    if mutbl {
        f.write_str(" mut ")?;
    }
    f.write_str("[")?;
    ty.write_rust(f, TypeMode::AllBorrowed(lifetime))?;
    f.write_str("]")?;

    Ok(())
}

fn print_generics(f: &mut core::fmt::Formatter<'_>, lifetime: Option<&str>) -> core::fmt::Result {
    if lifetime.is_none() {
        return Ok(());
    }
    f.write_str("<")?;
    if let Some(lt) = lifetime {
        f.write_str(lt)?;
        f.write_str(",")?;
    }
    f.write_str(">")?;

    Ok(())
}

fn lifetime_for(ty: &TypeDef, mode: TypeMode) -> Option<&'static str> {
    match mode {
        TypeMode::AllBorrowed(s) | TypeMode::LeafBorrowed(s)
            if ty.info.contains(TypeInfo::HAS_LIST) =>
        {
            Some(s)
        }
        _ => None,
    }
}

fn modes_of(ty: &TypeDef, mode: TypeMode) -> Vec<(String, TypeMode)> {
    // let info = self.info(ty);
    let mut result = Vec::new();
    if ty.info.contains(TypeInfo::PARAM) {
        result.push((param_name(ty, mode), mode));
    }
    if ty.info.contains(TypeInfo::RESULT)
        && (!ty.info.contains(TypeInfo::PARAM) || uses_two_names(&ty.info, mode))
    {
        result.push((result_name(ty, mode), TypeMode::Owned));
    }
    result
}

fn param_name(ty: &TypeDef, mode: TypeMode) -> String {
    let name = ty.name.to_upper_camel_case();
    if uses_two_names(&ty.info, mode) {
        format!("{}Param", name)
    } else {
        name
    }
}

fn result_name(ty: &TypeDef, mode: TypeMode) -> String {
    let name = ty.name.to_upper_camel_case();
    if uses_two_names(&ty.info, mode) {
        format!("{}Result", name)
    } else {
        name
    }
}

fn uses_two_names(info: &TypeInfo, mode: TypeMode) -> bool {
    info.owns_data()
        && info.contains(TypeInfo::PARAM | TypeInfo::RESULT)
        && match mode {
            TypeMode::AllBorrowed(_) | TypeMode::LeafBorrowed(_) => true,
            TypeMode::Owned => false,
        }
}

fn needs_borrow(ty: &Type, mode: TypeMode) -> bool {
    match ty {
        Type::Id(ty) => lifetime_for(&ty, mode).is_some(),
        Type::Tuple(ty) => ty.types.iter().any(|ty| needs_borrow(ty, mode)),
        Type::List(ty) => needs_borrow(ty, mode),
        Type::Option(ty) => needs_borrow(ty, mode),
        _ => false,
    }
}

/// Returns the names for the cases of the passed union.
fn union_case_names(union: &Union) -> Vec<String> {
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
    let mut case_names = vec![String::new(); union.cases.len()];
    // A map from case names to their `UsedState`.
    let mut used = HashMap::new();
    for (_case, name) in union.cases.iter().zip(case_names.iter_mut()) {
        // self.write_name(&case.ty, name);

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
                    write!(name, "{n}").unwrap();
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

pub fn int_repr(repr: Int) -> &'static str {
    match repr {
        Int::U8 => "u8",
        Int::U16 => "u16",
        Int::U32 => "u32",
        Int::U64 => "u64",
    }
}

pub fn to_rust_ident(name: &str) -> String {
    match name {
        // Escape Rust keywords.
        // Source: https://doc.rust-lang.org/reference/keywords.html
        "as" => "as_".into(),
        "break" => "break_".into(),
        "const" => "const_".into(),
        "continue" => "continue_".into(),
        "crate" => "crate_".into(),
        "else" => "else_".into(),
        "enum" => "enum_".into(),
        "extern" => "extern_".into(),
        "false" => "false_".into(),
        "fn" => "fn_".into(),
        "for" => "for_".into(),
        "if" => "if_".into(),
        "impl" => "impl_".into(),
        "in" => "in_".into(),
        "let" => "let_".into(),
        "loop" => "loop_".into(),
        "match" => "match_".into(),
        "mod" => "mod_".into(),
        "move" => "move_".into(),
        "mut" => "mut_".into(),
        "pub" => "pub_".into(),
        "ref" => "ref_".into(),
        "return" => "return_".into(),
        "self" => "self_".into(),
        "static" => "static_".into(),
        "struct" => "struct_".into(),
        "super" => "super_".into(),
        "trait" => "trait_".into(),
        "true" => "true_".into(),
        "type" => "type_".into(),
        "unsafe" => "unsafe_".into(),
        "use" => "use_".into(),
        "where" => "where_".into(),
        "while" => "while_".into(),
        "async" => "async_".into(),
        "await" => "await_".into(),
        "dyn" => "dyn_".into(),
        "abstract" => "abstract_".into(),
        "become" => "become_".into(),
        "box" => "box_".into(),
        "do" => "do_".into(),
        "final" => "final_".into(),
        "macro" => "macro_".into(),
        "override" => "override_".into(),
        "priv" => "priv_".into(),
        "typeof" => "typeof_".into(),
        "unsized" => "unsized_".into(),
        "virtual" => "virtual_".into(),
        "yield" => "yield_".into(),
        "try" => "try_".into(),
        s => s.to_snake_case(),
    }
}
