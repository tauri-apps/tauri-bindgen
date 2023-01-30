#![allow(clippy::must_use_candidate)]

use heck::{ToSnakeCase, ToUpperCamelCase};
use std::collections::HashMap;
use std::fmt::{self, Write};
use tauri_bindgen_core::TypeInfo;
use wit_parser::{
    Docs, Enum, Flags, Function, Int, Interface, Record, Result_, Results, Tuple, Type,
    TypeDefKind, TypeId, Union, Variant,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BorrowMode {
    Owned,
    AllBorrowed(&'static str),
    LeafBorrowed(&'static str),
}

pub trait RustGenerator<'a> {
    fn iface(&self) -> &'a Interface;

    fn info(&self, ty: TypeId) -> TypeInfo;
    fn default_param_mode(&self) -> BorrowMode;
    fn push_str(&mut self, s: &str);
    fn print_borrowed_str(&mut self, lifetime: &'static str);

    fn use_std(&self) -> bool {
        true
    }

    fn print_ty(&mut self, ty: &Type, mode: BorrowMode) {
        match ty {
            Type::Id(t) => self.print_tyid(*t, mode),
            Type::Bool => self.push_str("bool"),
            Type::U8 => self.push_str("u8"),
            Type::U16 => self.push_str("u16"),
            Type::U32 => self.push_str("u32"),
            Type::U64 => self.push_str("u64"),
            Type::S8 => self.push_str("i8"),
            Type::S16 => self.push_str("i16"),
            Type::S32 => self.push_str("i32"),
            Type::S64 => self.push_str("i64"),
            Type::Float32 => self.push_str("f32"),
            Type::Float64 => self.push_str("f64"),
            Type::Char => self.push_str("char"),
            Type::Tuple(ty) => {
                self.push_str("(");
                for ty in &ty.types {
                    self.print_ty(ty, mode);
                    self.push_str(",");
                }
                self.push_str(")");
            }
            Type::List(ty) => self.print_list(ty, mode),
            Type::Option(ty) => {
                self.push_str("Option<");
                self.print_ty(ty, mode);
                self.push_str(">");
            }
            Type::Result(r) => {
                self.push_str("Result<");
                self.print_optional_ty(r.ok.as_ref(), mode);
                self.push_str(",");
                self.print_optional_ty(r.err.as_ref(), mode);
                self.push_str(">");
            }
            Type::String => match mode {
                BorrowMode::Owned => self.push_str("String"),
                BorrowMode::AllBorrowed(lt) | BorrowMode::LeafBorrowed(lt) => {
                    self.print_borrowed_str(lt);
                }
            },
        }
    }

    fn print_optional_ty(&mut self, ty: Option<&Type>, mode: BorrowMode) {
        if let Some(ty) = ty {
            self.print_ty(ty, mode);
        } else {
            self.push_str("()");
        }
    }

    fn print_tyid(&mut self, id: TypeId, mode: BorrowMode) {
        let ty = &self.iface().types[id];
        let info = self.info(id);
        let lt = self.lifetime_for(&info, mode);

        // if ty.name.is_some() {
        let name = if lt.is_some() {
            self.param_name(id)
        } else {
            self.result_name(id)
        };
        self.push_str(&name);

        // If the type recursively owns data and it's a
        // variant/record/list, then we need to place the
        // lifetime parameter on the type as well.
        if info.owns_data() && self.typedef_needs_generics(&ty.kind) {
            self.print_generics(lt);
        }
    }

    fn typedef_needs_generics(&self, ty: &TypeDefKind) -> bool {
        match ty {
            TypeDefKind::Variant(_)
            | TypeDefKind::Record(_)
            | TypeDefKind::Flags(_)
            | TypeDefKind::Enum(_)
            | TypeDefKind::Union(_)
            | TypeDefKind::Alias(Type::String) => true,
            TypeDefKind::Alias(Type::Id(t)) => {
                self.typedef_needs_generics(&self.iface().types[*t].kind)
            }
            TypeDefKind::Alias(ty) => self.type_needs_generics(ty),
        }
    }

    fn type_needs_generics(&self, ty: &Type) -> bool {
        match ty {
            Type::Tuple(ty) => ty.types.iter().any(|ty| self.type_needs_generics(ty)),
            Type::List(_) | Type::String => true,
            Type::Option(ty) => self.type_needs_generics(ty),
            Type::Result(res) => {
                res.ok
                    .as_ref()
                    .map(|ty| self.type_needs_generics(ty))
                    .unwrap_or_default()
                    || res
                        .err
                        .as_ref()
                        .map(|ty| self.type_needs_generics(ty))
                        .unwrap_or_default()
            }
            Type::Id(id) => {
                let info = self.info(*id);
                self.lifetime_for(&info, BorrowMode::AllBorrowed("'a"))
                    .is_some()
            }
            _ => false,
        }
    }

    fn print_list(&mut self, ty: &Type, mode: BorrowMode) {
        match mode {
            BorrowMode::Owned => {
                self.push_str("Vec<");
                self.print_ty(ty, mode);
                self.push_str(">");
            }
            BorrowMode::LeafBorrowed(lt) | BorrowMode::AllBorrowed(lt) => {
                self.print_borrowed_slice(false, ty, lt);
            } // FIXME: bring this back
              // BorrowMode::LeafBorrowed(lt) => {
              //     if self.iface().all_bits_valid(ty) {
              //         self.print_borrowed_slice(false, ty, lt);
              //     } else {
              //         self.push_str("Vec<");
              //         self.print_ty(ty, mode);
              //         self.push_str(">");
              //     }
              // }
        }
    }

    fn print_rust_slice(&mut self, mutbl: bool, ty: &Type, lifetime: &'static str) {
        self.push_str("&");
        if lifetime != "'_" {
            self.push_str(lifetime);
            self.push_str(" ");
        }
        if mutbl {
            self.push_str(" mut ");
        }
        self.push_str("[");
        self.print_ty(ty, BorrowMode::AllBorrowed(lifetime));
        self.push_str("]");
    }

    fn print_borrowed_slice(&mut self, mutbl: bool, ty: &Type, lifetime: &'static str) {
        self.print_rust_slice(mutbl, ty, lifetime);
    }

    fn print_generics(&mut self, lifetime: Option<&str>) {
        if lifetime.is_none() {
            return;
        }
        self.push_str("<");
        if let Some(lt) = lifetime {
            self.push_str(lt);
            self.push_str(",");
        }
        self.push_str(">");
    }

    fn print_typedef_record(
        &mut self,
        id: TypeId,
        record: &Record,
        docs: &Docs,
        attrs: impl Fn(&str, bool, TypeInfo) -> Option<String>,
    ) {
        let info = self.info(id);
        for TypeVariant { name, borrow_mode } in self.variants_of(id) {
            let lt = self.lifetime_for(&info, borrow_mode);
            self.print_rustdoc(docs);

            if info.owns_data() {
                self.push_str("#[derive(Debug, Clone, PartialEq)]\n");
            } else {
                self.push_str("#[repr(C)]\n");
                self.push_str("#[derive(Debug, Copy, Clone, PartialEq)]\n");
            }
            if let Some(attrs) = attrs(&name, self.uses_two_names(&info), info) {
                self.push_str(&attrs);
            }

            self.push_str(&format!("pub struct {name}"));
            self.print_generics(lt);
            self.push_str(" {\n");

            for field in &record.fields {
                self.print_rustdoc(&field.docs);
                self.push_str("pub ");
                self.push_str(&to_rust_ident(&field.name));
                self.push_str(": ");
                self.print_ty(&field.ty, borrow_mode);
                self.push_str(",\n");
            }

            self.push_str("}\n");
        }
    }

    fn print_typedef_tuple(&mut self, id: TypeId, tuple: &Tuple, docs: &Docs) {
        let info = self.info(id);

        for TypeVariant { name, borrow_mode } in self.variants_of(id) {
            let lt = self.lifetime_for(&info, borrow_mode);

            self.print_rustdoc(docs);
            self.push_str(&format!("pub type {name}"));
            self.print_generics(lt);
            self.push_str(" = (");

            for ty in &tuple.types {
                self.print_ty(ty, borrow_mode);
                self.push_str(",");
            }

            self.push_str(");\n");
        }
    }

    fn print_typedef_list(&mut self, id: TypeId, ty: &Type, docs: &Docs) {
        let info = self.info(id);

        for TypeVariant { name, borrow_mode } in self.variants_of(id) {
            let lt = self.lifetime_for(&info, borrow_mode);

            self.print_rustdoc(docs);
            self.push_str(&format!("pub type {name}"));
            self.print_generics(lt);
            self.push_str(" = ");
            self.print_list(ty, borrow_mode);
            self.push_str(";\n");
        }
    }

    fn print_typedef_alias(&mut self, id: TypeId, ty: &Type, docs: &Docs) {
        let info = self.info(id);

        for TypeVariant { name, borrow_mode } in self.variants_of(id) {
            let lt = self.lifetime_for(&info, borrow_mode);

            self.print_rustdoc(docs);
            self.push_str(&format!("pub type {name}"));
            self.print_generics(lt);
            self.push_str(" = ");
            self.print_ty(ty, borrow_mode);
            self.push_str(";\n");
        }
    }

    fn print_typedef_option(&mut self, id: TypeId, payload: &Type, docs: &Docs) {
        let info = self.info(id);

        for TypeVariant { name, borrow_mode } in self.variants_of(id) {
            let lt = self.lifetime_for(&info, borrow_mode);

            self.print_rustdoc(docs);
            self.push_str(&format!("pub type {name}"));
            self.print_generics(lt);
            self.push_str(" = Option<");
            self.print_ty(payload, borrow_mode);
            self.push_str(">;\n");
        }
    }

    fn print_typedef_result(&mut self, id: TypeId, result: &Result_, docs: &Docs) {
        let info = self.info(id);

        for TypeVariant { name, borrow_mode } in self.variants_of(id) {
            let lt = self.lifetime_for(&info, borrow_mode);

            self.print_rustdoc(docs);
            self.push_str(&format!("pub type {name}"));
            self.print_generics(lt);
            self.push_str(" = Result<");
            self.print_optional_ty(result.ok.as_ref(), borrow_mode);
            self.push_str(", ");
            self.print_optional_ty(result.err.as_ref(), borrow_mode);
            self.push_str(">;\n");
        }
    }

    fn print_typedef_variant(
        &mut self,
        id: TypeId,
        variant: &Variant,
        docs: &Docs,
        attrs: impl Fn(&str, bool, TypeInfo) -> Option<String>,
    ) where
        Self: Sized,
    {
        self.print_rust_enum(
            id,
            variant
                .cases
                .iter()
                .map(|c| (c.name.to_upper_camel_case(), &c.docs, c.ty.as_ref())),
            docs,
            attrs,
        );
    }

    fn print_typedef_union(
        &mut self,
        id: TypeId,
        union: &Union,
        docs: &Docs,
        attrs: impl Fn(&str, bool, TypeInfo) -> Option<String>,
    ) where
        Self: Sized,
    {
        self.print_rust_enum(
            id,
            self.union_case_names(union)
                .into_iter()
                .zip(&union.cases)
                .map(|(name, case)| (name, &case.docs, Some(&case.ty))),
            docs,
            attrs,
        );
    }

    fn print_rust_enum<'b>(
        &mut self,
        id: TypeId,
        cases: impl IntoIterator<Item = (String, &'b Docs, Option<&'b Type>)> + Clone,
        docs: &Docs,
        attrs: impl Fn(&str, bool, TypeInfo) -> Option<String>,
    ) where
        Self: Sized,
    {
        let info = self.info(id);

        for TypeVariant { name, borrow_mode } in self.variants_of(id) {
            let lt = self.lifetime_for(&info, borrow_mode);
            let name = name.to_upper_camel_case();

            self.print_rustdoc(docs);
            if info.owns_data() {
                self.push_str("#[derive(Debug, Clone, PartialEq)]\n");
            } else {
                self.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\n");
            }
            if let Some(attrs) = attrs(&name, self.uses_two_names(&info), info) {
                self.push_str(&attrs);
            }
            self.push_str(&format!("pub enum {name}"));
            self.print_generics(lt);
            self.push_str("{\n");

            for (case_name, docs, payload) in cases.clone() {
                self.print_rustdoc(docs);
                self.push_str(&case_name);

                if let Some(payload) = payload {
                    self.push_str("(");
                    self.print_ty(payload, borrow_mode);
                    self.push_str(")");
                }

                self.push_str(",\n");
            }
            self.push_str("}\n");
        }
    }

    fn print_typedef_enum(
        &mut self,
        id: TypeId,
        enum_: &Enum,
        docs: &Docs,
        attrs: impl Fn(&str, bool, TypeInfo) -> Option<String>,
    ) {
        let info = self.info(id);

        for TypeVariant { name, borrow_mode } in self.variants_of(id) {
            let lt = self.lifetime_for(&info, borrow_mode);
            let name = name.to_upper_camel_case();

            self.print_rustdoc(docs);
            self.push_str("#[repr(");
            self.int_repr(&enum_.tag());
            self.push_str(")]\n");
            if info.owns_data() {
                self.push_str("#[derive(Debug, Clone, PartialEq)]\n");
            } else {
                self.push_str("#[derive(Debug, Clone, Copy, PartialEq)]\n");
            }
            if let Some(attrs) = attrs(&name, self.uses_two_names(&info), info) {
                self.push_str(&attrs);
            }
            self.push_str(&format!("pub enum {name}"));
            self.print_generics(lt);
            self.push_str("{\n");

            for case in &enum_.cases {
                self.print_rustdoc(&case.docs);
                self.push_str(&case.name.to_upper_camel_case());
                self.push_str(",\n");
            }

            self.push_str("}\n");
        }
    }

    fn print_signature(
        &mut self,
        func: &Function,
        param_mode: BorrowMode,
        sig: &FnSig,
    ) -> Vec<String> {
        let params = self.print_docs_and_params(func, param_mode, sig);
        self.push_str(" -> ");
        self.print_result_params(&func.results, BorrowMode::Owned);
        params
    }

    fn print_docs_and_params(
        &mut self,
        func: &Function,
        param_mode: BorrowMode,
        sig: &FnSig,
    ) -> Vec<String> {
        self.print_rustdoc(&func.docs);
        // self.rustdoc_params(&func.params, "Parameters");
        // TODO: re-add this when docs are back
        // self.rustdoc_params(&func.results, "Return");

        if !sig.private {
            self.push_str("pub ");
        }
        if sig.unsafe_ {
            self.push_str("unsafe ");
        }
        if sig.async_ {
            self.push_str("async ");
        }
        self.push_str("fn ");
        self.push_str(&to_rust_ident(&func.name));
        if let Some(generics) = &sig.generics {
            self.push_str(generics);
        }
        self.push_str("(");
        if let Some(arg) = &sig.self_arg {
            self.push_str(arg);
            self.push_str(",");
        }
        let mut params = Vec::new();
        for (i, (name, param)) in func.params.iter().enumerate() {
            if i == 0 && sig.self_is_first_param {
                params.push("self".to_string());
                continue;
            }
            let name = to_rust_ident(name);
            self.push_str(&name);
            params.push(name);
            self.push_str(": ");
            self.print_ty(param, param_mode);
            self.push_str(",");
        }
        self.push_str(")");
        params
    }

    fn print_result_params(&mut self, results: &Results, mode: BorrowMode) {
        match results.len() {
            0 => self.push_str("()"),
            1 => self.print_ty(results.types().next().unwrap(), mode),
            _ => {
                self.push_str("(");
                for ty in results.types() {
                    self.print_ty(ty, mode);
                    self.push_str(", ");
                }
                self.push_str(")");
            }
        }
    }

    fn print_rustdoc(&mut self, docs: &Docs) {
        for line in docs.contents.trim().lines() {
            self.push_str("/// ");
            self.push_str(line);
            self.push_str("\n");
        }
    }

    /// Writes the camel-cased 'name' of the passed type to `out`, as used to name union variants.
    fn write_name(&self, ty: &Type, out: &mut String) {
        match ty {
            Type::Bool => out.push_str("Bool"),
            Type::U8 => out.push_str("U8"),
            Type::U16 => out.push_str("U16"),
            Type::U32 => out.push_str("U32"),
            Type::U64 => out.push_str("U64"),
            Type::S8 => out.push_str("I8"),
            Type::S16 => out.push_str("I16"),
            Type::S32 => out.push_str("I32"),
            Type::S64 => out.push_str("I64"),
            Type::Float32 => out.push_str("F32"),
            Type::Float64 => out.push_str("F64"),
            Type::Char => out.push_str("Char"),
            Type::String => out.push_str("String"),
            Type::Tuple(_) => out.push_str("Tuple"),
            Type::List(ty) => {
                self.write_name(ty, out);
                out.push_str("List");
            }
            Type::Option(_) => {
                out.push_str("Optional");
                self.write_name(ty, out);
            }
            Type::Result(_) => out.push_str("Result"),
            Type::Id(id) => {
                let ty = &self.iface().types[*id];
                match &ty.kind {
                    TypeDefKind::Alias(ty) => self.write_name(ty, out),
                    TypeDefKind::Record(_) => out.push_str("Record"),
                    TypeDefKind::Flags(_) => out.push_str("Flags"),
                    TypeDefKind::Variant(_) => out.push_str("Variant"),
                    TypeDefKind::Enum(_) => out.push_str("Enum"),
                    TypeDefKind::Union(_) => out.push_str("Union"),
                }
            }
        }
    }

    fn write_optional_name(&self, ty: Option<&Type>, out: &mut String) {
        match ty {
            Some(ty) => self.write_name(ty, out),
            None => out.push_str("()"),
        }
    }

    /// Returns the names for the cases of the passed union.
    fn union_case_names(&self, union: &Union) -> Vec<String> {
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
        for (case, name) in union.cases.iter().zip(case_names.iter_mut()) {
            self.write_name(&case.ty, name);

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

    fn int_repr(&mut self, repr: &Int) {
        self.push_str(int_repr(repr));
    }

    fn variants_of(&self, ty: TypeId) -> Vec<TypeVariant> {
        let info = self.info(ty);
        let mut result = Vec::new();
        if info.contains(TypeInfo::PARAM) {
            result.push(TypeVariant {
                name: self.param_name(ty),
                borrow_mode: self.default_param_mode(),
            });
        }
        if info.contains(TypeInfo::RESULT)
            && (!info.contains(TypeInfo::PARAM) || self.uses_two_names(&info))
        {
            result.push(TypeVariant {
                name: self.result_name(ty),
                borrow_mode: BorrowMode::Owned,
            });
        }
        result
    }

    fn param_name(&self, ty: TypeId) -> String {
        let info = self.info(ty);
        let name = self.iface().types[ty].name.to_upper_camel_case();
        if self.uses_two_names(&info) {
            format!("{name}Param")
        } else {
            name
        }
    }

    fn result_name(&self, ty: TypeId) -> String {
        let info = self.info(ty);
        let name = self.iface().types[ty].name.to_upper_camel_case();
        if self.uses_two_names(&info) {
            format!("{name}Result")
        } else {
            name
        }
    }

    fn uses_two_names(&self, info: &TypeInfo) -> bool {
        info.owns_data()
            && info.contains(TypeInfo::PARAM | TypeInfo::RESULT)
            && match self.default_param_mode() {
                BorrowMode::AllBorrowed(_) | BorrowMode::LeafBorrowed(_) => true,
                BorrowMode::Owned => false,
            }
    }

    fn lifetime_for(&self, info: &TypeInfo, mode: BorrowMode) -> Option<&'static str> {
        match mode {
            BorrowMode::AllBorrowed(s) | BorrowMode::LeafBorrowed(s)
                if info.contains(TypeInfo::HAS_LIST) =>
            {
                Some(s)
            }
            _ => None,
        }
    }

    fn needs_borrow(&self, ty: &Type, mode: BorrowMode) -> bool {
        match ty {
            Type::Id(id) => {
                let info = self.info(*id);

                self.lifetime_for(&info, mode).is_some()
            }
            Type::Tuple(ty) => ty.types.iter().any(|ty| self.needs_borrow(ty, mode)),
            Type::List(ty) | Type::Option(ty) => self.needs_borrow(ty, mode),
            _ => false,
        }
    }

    fn needs_lifetime(&self, ty: &Type) -> bool {
        match ty {
            Type::Tuple(ty) => ty.types.iter().any(|ty| self.needs_lifetime(ty)),
            Type::List(_) | Type::String => true,
            Type::Option(ty) => self.needs_lifetime(ty),
            Type::Result(res) => {
                res.ok
                    .as_ref()
                    .map(|ty| self.needs_lifetime(ty))
                    .unwrap_or_default()
                    || res
                        .err
                        .as_ref()
                        .map(|ty| self.needs_lifetime(ty))
                        .unwrap_or_default()
            }
            Type::Id(id) => {
                let info = self.info(*id);
                self.lifetime_for(&info, BorrowMode::AllBorrowed("'a"))
                    .is_some()
            }
            _ => false,
        }
    }
}

pub struct TypeVariant {
    name: String,
    borrow_mode: BorrowMode,
}

pub fn int_repr(repr: &Int) -> &'static str {
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

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Default)]
pub struct FnSig {
    pub async_: bool,
    pub unsafe_: bool,
    pub private: bool,
    pub use_item_name: bool,
    pub generics: Option<String>,
    pub self_arg: Option<String>,
    pub self_is_first_param: bool,
}

pub enum RustFlagsRepr {
    U8,
    U16,
    U32,
    U64,
}

impl RustFlagsRepr {
    pub fn new(f: &Flags) -> RustFlagsRepr {
        match f.repr() {
            Int::U8 => RustFlagsRepr::U8,
            Int::U16 => RustFlagsRepr::U16,
            Int::U32 => RustFlagsRepr::U32,
            Int::U64 => RustFlagsRepr::U64,
            // FlagsRepr::U32(n) => panic!("unsupported number of flags: {}", n * 32),
        }
    }
}

impl fmt::Display for RustFlagsRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RustFlagsRepr::U8 => "u8".fmt(f),
            RustFlagsRepr::U16 => "u16".fmt(f),
            RustFlagsRepr::U32 => "u32".fmt(f),
            RustFlagsRepr::U64 => "u64".fmt(f),
            // RustFlagsRepr::U128 => "u128".fmt(f),
        }
    }
}
