use bindgen_core::{uwriteln, InterfaceGenerator as _, Source, WorldGenerator};
use heck::ToUpperCamelCase;
use heck::*;
use std::collections::HashSet;
use std::fmt::Write as _;
use std::io::{Write, Read};
use std::mem;
use std::process::{Command, Stdio};
use wit_parser::*;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Opts {
    /// Whether or not `prettier` is executed to format generated code.
    #[cfg_attr(feature = "clap", arg(long))]
    pub prettier: bool,
    /// Names of functions to skip generating bindings for.
    #[cfg_attr(feature = "clap", arg(long))]
    pub skip: Vec<String>,
}

impl Opts {
    pub fn build(self) -> Box<dyn WorldGenerator> {
        let mut r = JavaScript::default();
        r.skip = self.skip.iter().cloned().collect();
        r.opts = self;
        Box::new(r)
    }
}

#[derive(Debug, Default)]
struct JavaScript {
    src: Source,
    opts: Opts,
    skip: HashSet<String>,
}

impl WorldGenerator for JavaScript {
    fn import(
        &mut self,
        name: &str,
        iface: &wit_parser::Interface,
        _files: &mut bindgen_core::Files,
    ) {
        let mut gen = InterfaceGenerator::new(self, iface);

        gen.print_intro();
        gen.types();
    
        for func in iface.functions.iter() {
            gen.generate_guest_import(name, func);
        }

        gen.print_outro();

        let module = &gen.src[..];
        uwriteln!(self.src, "{module}");

        // files.push(&format!("{name}.ts"), gen.src.as_bytes());

        // uwriteln!(
        //     self.src.ts,
        //     "{} {{ {camel} }} from './{name}';",
        //     // In instance mode, we have no way to assert the imported types
        //     // in the ambient declaration file. Instead we just export the
        //     // import namespace types for users to use.
        //     "export"
        // );

        // uwriteln!(self.import_object, "export const {name}: typeof {camel};");
    }

    fn finish(&mut self, name: &str, files: &mut bindgen_core::Files) {
        let mut src = mem::take(&mut self.src);
        if self.opts.prettier {
            let mut child = Command::new("prettier")
                .arg(format!("--parser=typescript"))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to spawn `prettier`");
            child
                .stdin
                .take()
                .unwrap()
                .write_all(src.as_bytes())
                .unwrap();
            src.as_mut_string().truncate(0);
            child
                .stdout
                .take()
                .unwrap()
                .read_to_string(src.as_mut_string())
                .unwrap();
            let status = child.wait().unwrap();
            assert!(status.success());
        }

        files.push(&format!("{name}.ts"), src.as_bytes());
    }
}

struct InterfaceGenerator<'a> {
    src: Source,
    gen: &'a mut JavaScript,
    iface: &'a Interface,
    needs_ty_option: bool,
    needs_ty_result: bool,
    // types: Types,
}

impl<'a> InterfaceGenerator<'a> {
    pub fn new(gen: &'a mut JavaScript, iface: &'a Interface) -> Self {
        Self {
            src: Source::default(),
            gen,
            iface,
            needs_ty_option: false,
            needs_ty_result: false,
        }
    }

    fn push_str(&mut self, s: &str) {
        self.src.push_str(s);
    }

    fn print_typedoc(&mut self, docs: &Docs) {
        let docs = match &docs.contents {
            Some(docs) => docs,
            None => return,
        };
        self.push_str("/**\n");
        for line in docs.trim().lines() {
            self.push_str(&format!(" * {}\n", line));
        }
        self.push_str(" */\n");
    }

    fn print_intro(&mut self) {
        self.push_str("
        interface Tauri {
            invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T>
        }
        declare global {
            interface Window {
                __TAURI__: { tauri: Tauri };
            }
        }
        const { invoke } = window.__TAURI__.tauri;
        ");
    }

    fn print_outro(&mut self) {
        if mem::take(&mut self.needs_ty_option) {
            self.push_str("export type Option<T> = { tag: 'none' } | { tag: 'some', val; T };\n");
        }
        if mem::take(&mut self.needs_ty_result) {
            self.push_str(
                "export type Result<T, E> = { tag: 'ok', val: T } | { tag: 'err', val: E };\n",
            );
        }
    }

    fn generate_guest_import(&mut self, mod_name: &str, func: &Function) {
        if self.gen.skip.contains(&func.name) {
            return;
        }

        self.print_typedoc(&func.docs);

        self.push_str("export async function ");
        self.push_str(&func.item_name().to_lower_camel_case());
        self.push_str("(");

        let param_start = match &func.kind {
            FunctionKind::Freestanding => 0,
        };

        for (i, (name, ty)) in func.params[param_start..].iter().enumerate() {
            if i > 0 {
                self.push_str(", ");
            }
            self.push_str(to_js_ident(&name.to_lower_camel_case()));
            self.push_str(": ");
            self.print_ty(ty);
        }
        self.push_str("): Promise<");
        if let Some((ok_ty, _)) = func.results.throws(self.iface) {
            self.print_optional_ty(ok_ty);
        } else {
            match func.results.len() {
                0 => self.push_str("void"),
                1 => self.print_ty(func.results.iter_types().next().unwrap()),
                _ => {
                    self.push_str("[");
                    for (i, ty) in func.results.iter_types().enumerate() {
                        if i != 0 {
                            self.push_str(", ");
                        }
                        self.print_ty(ty);
                    }
                    self.push_str("]");
                }
            }
        }
        self.push_str("> {\n");

        if func.results.len() > 0 {
            self.push_str("const result = ");
        }

        self.push_str("await invoke<");
        
        if let Some((ok_ty, _)) = func.results.throws(self.iface) {
            self.print_optional_ty(ok_ty);
        } else {
            match func.results.len() {
                0 => self.push_str("void"),
                1 => self.print_ty(func.results.iter_types().next().unwrap()),
                _ => {
                    self.push_str("[");
                    for (i, ty) in func.results.iter_types().enumerate() {
                        if i != 0 {
                            self.push_str(", ");
                        }
                        self.print_ty(ty);
                    }
                    self.push_str("]");
                }
            }
        }

        self.push_str(">(");

        self.push_str(&format!(
            "\"plugin:{}|{}\",",
            mod_name.to_snake_case(),
            func.name.to_snake_case()
        ));

        if !func.params.is_empty() {
            self.push_str("{");
            for (i, (name, _ty)) in func.params.iter().enumerate() {
                if i > 0 {
                    self.push_str(", ");
                }
                self.push_str(&name.to_lower_camel_case());
                self.push_str(": ");
                self.push_str(to_js_ident(&name.to_lower_camel_case()));
            }
            self.push_str("}");
        }

        self.push_str(");\n");

        if func.results.len() > 0 {
            self.push_str("return result\n");
        }

        self.push_str("}\n");
    }

    fn print_ty(&mut self, ty: &Type) {
        match ty {
            Type::Bool => self.push_str("boolean"),
            Type::U8
            | Type::U16
            | Type::U32
            | Type::S8
            | Type::S16
            | Type::S32
            | Type::Float32
            | Type::Float64 => self.push_str("number"),
            Type::U64 | Type::S64 => self.push_str("bigint"),
            Type::Char | Type::String => self.push_str("string"),
            Type::Id(id) => {
                let ty = &self.iface.types[*id];
                if let Some(name) = &ty.name {
                    return self.push_str(&name.to_upper_camel_case());
                }
                match &ty.kind {
                    TypeDefKind::Record(_) => todo!(),
                    TypeDefKind::Flags(_) => todo!(),
                    TypeDefKind::Tuple(ty) => self.print_tuple(ty),
                    TypeDefKind::Variant(_) => todo!(),
                    TypeDefKind::Enum(_) => todo!(),
                    TypeDefKind::Option(_) => todo!(),
                    TypeDefKind::Result(_) => todo!(),
                    TypeDefKind::Union(_) => todo!(),
                    TypeDefKind::List(ty) => self.print_list(ty),
                    TypeDefKind::Future(_) => todo!(),
                    TypeDefKind::Stream(_) => todo!(),
                    TypeDefKind::Type(ty) => self.print_ty(ty),
                }
            }
        }
    }

    fn print_optional_ty(&mut self, ty: Option<&Type>) {
        match ty {
            Some(ty) => self.print_ty(ty),
            None => self.push_str("void"),
        }
    }

    fn print_tuple(&mut self, tuple: &Tuple) {
        self.push_str("[");
        for (i, ty) in tuple.types.iter().enumerate() {
            if i > 0 {
                self.push_str(", ");
            }
            self.print_ty(ty);
        }
        self.push_str("]");
    }

    fn print_list(&mut self, ty: &Type) {
        match self.array_ty(self.iface, ty) {
            Some(ty) => self.push_str(ty),
            None => {
                self.print_ty(ty);
                self.push_str("[]");
            }
        }
    }

    fn array_ty(&self, iface: &Interface, ty: &Type) -> Option<&'static str> {
        match ty {
            Type::Bool => None,
            Type::U8 => Some("Uint8Array"),
            Type::S8 => Some("Int8Array"),
            Type::U16 => Some("Uint16Array"),
            Type::S16 => Some("Int16Array"),
            Type::U32 => Some("Uint32Array"),
            Type::S32 => Some("Int32Array"),
            Type::U64 => Some("BigUint64Array"),
            Type::S64 => Some("BigInt64Array"),
            Type::Float32 => Some("Float32Array"),
            Type::Float64 => Some("Float64Array"),
            Type::Char => None,
            Type::String => None,
            Type::Id(id) => match &iface.types[*id].kind {
                TypeDefKind::Type(t) => self.array_ty(iface, t),
                _ => None,
            },
        }
    }

    fn is_nullable(&self, ty: &Type) -> bool {
        let id = match ty {
            Type::Id(id) => *id,
            _ => return false,
        };
        match &self.iface.types[id].kind {
            // If `ty` points to an `option<T>`, then `ty` can be represented
            // with `null` if `t` itself can't be represented with null. For
            // example `option<option<u32>>` can't be represented with `null`
            // since that's ambiguous if it's `none` or `some(none)`.
            //
            // Note, oddly enough, that `option<option<option<u32>>>` can be
            // represented as `null` since:
            //
            // * `null` => `none`
            // * `{ tag: "none" }` => `some(none)`
            // * `{ tag: "some", val: null }` => `some(some(none))`
            // * `{ tag: "some", val: 1 }` => `some(some(some(1)))`
            //
            // It's doubtful anyone would actually rely on that though due to
            // how confusing it is.
            TypeDefKind::Option(ty) => !self.is_nullable(ty),
            TypeDefKind::Type(t) => self.is_nullable(t),
            _ => false,
        }
    }
}

impl<'a> bindgen_core::InterfaceGenerator<'a> for InterfaceGenerator<'a> {
    fn iface(&self) -> &'a wit_parser::Interface {
        self.iface
    }

    fn type_record(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        record: &wit_parser::Record,
        docs: &wit_parser::Docs,
    ) {
        self.print_typedoc(docs);
        self.push_str(&format!(
            "export interface {} {{\n",
            name.to_upper_camel_case()
        ));

        for field in record.fields.iter() {
            self.print_typedoc(&field.docs);
            self.push_str(&field.name.to_lower_camel_case());
            if self.is_nullable(&field.ty) {
                self.push_str("?");
            }
            self.push_str(": ");
            self.print_ty(&field.ty);

            self.push_str(",\n");
        }

        self.push_str("};\n");
    }

    fn type_flags(
        &mut self,
        _id: wit_parser::TypeId,
        _name: &str,
        _flags: &wit_parser::Flags,
        _docs: &wit_parser::Docs,
    ) {
        todo!()
    }

    fn type_tuple(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        tuple: &wit_parser::Tuple,
        docs: &wit_parser::Docs,
    ) {
        self.print_typedoc(docs);
        self.push_str(&format!("export type {} = ", name.to_upper_camel_case()));
        self.print_tuple(tuple);
        self.push_str(";\n");
    }

    fn type_variant(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        variant: &wit_parser::Variant,
        docs: &wit_parser::Docs,
    ) {
        self.print_typedoc(docs);
        self.push_str(&format!("export type {} = ", name.to_upper_camel_case()));
        for (i, case) in variant.cases.iter().enumerate() {
            if i > 0 {
                self.push_str("| ");
            }
            self.push_str(&format!("{}_{}", name, case.name).to_upper_camel_case());
        }
        self.push_str(";\n");

        for case in variant.cases.iter() {
            self.print_typedoc(&case.docs);
            self.push_str(&format!(
                "export interface {} {{\n",
                format!("{}_{}", name, case.name).to_upper_camel_case()
            ));

            self.push_str("tag: '");
            self.push_str(&case.name);
            self.push_str("',\n");

            if let Some(ty) = case.ty {
                self.push_str("val: ");
                self.print_ty(&ty);
                self.push_str(",\n");
            }
            self.push_str("}\n");
        }
    }

    fn type_option(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        payload: &wit_parser::Type,
        docs: &wit_parser::Docs,
    ) {
        self.print_typedoc(docs);
        self.push_str(&format!("export type {} = ", name.to_upper_camel_case()));
        if self.is_nullable(payload) {
            self.needs_ty_option = true;
            self.push_str("Option<");
            self.print_ty(payload);
            self.push_str(">");
        } else {
            self.print_ty(payload);
            self.push_str(" | null");
        }
        self.push_str(";\n");
    }

    fn type_result(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        result: &wit_parser::Result_,
        docs: &wit_parser::Docs,
    ) {
        self.needs_ty_result = true;
        self.print_typedoc(docs);
        self.push_str(&format!(
            "export type {} = Result<",
            name.to_upper_camel_case()
        ));
        self.print_optional_ty(result.ok.as_ref());
        self.push_str(", ");
        self.print_optional_ty(result.err.as_ref());
        self.push_str(">;\n");
    }

    fn type_union(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        union: &wit_parser::Union,
        docs: &wit_parser::Docs,
    ) {
        self.print_typedoc(docs);
        let name = name.to_upper_camel_case();
        self.push_str(&format!("export type {name} = "));
        for i in 0..union.cases.len() {
            if i > 0 {
                self.push_str(" | ");
            }
            self.push_str(&format!("{name}{i}"));
        }
        self.push_str(";\n");
        for (i, case) in union.cases.iter().enumerate() {
            self.print_typedoc(&case.docs);
            self.push_str(&format!("export interface {name}{i} {{\n"));
            self.push_str(&format!("tag: {i},\n"));
            self.push_str("val: ");
            self.print_ty(&case.ty);
            self.push_str(",\n");
            self.push_str("}\n");
        }
    }

    fn type_enum(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        enum_: &wit_parser::Enum,
        docs: &wit_parser::Docs,
    ) {
        self.print_typedoc(docs);

        self.push_str(&format!("export type {} = ", name.to_upper_camel_case()));
        for (i, case) in enum_.cases.iter().enumerate() {
            if i != 0 {
                self.push_str(" | ");
            }
            self.push_str(&format!("'{}'", case.name));
        }

        self.push_str(";\n");
    }

    fn type_alias(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        ty: &wit_parser::Type,
        docs: &wit_parser::Docs,
    ) {
        self.print_typedoc(docs);
        self.push_str(&format!("export type {} = ", name.to_upper_camel_case()));
        self.print_ty(ty);
        self.push_str(";\n");
    }

    fn type_list(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        ty: &wit_parser::Type,
        docs: &wit_parser::Docs,
    ) {
        self.print_typedoc(docs);
        self.push_str(&format!("export type {} = ", name.to_upper_camel_case()));
        self.print_list(ty);
        self.push_str(";\n");
    }

    fn type_builtin(
        &mut self,
        id: wit_parser::TypeId,
        name: &str,
        ty: &wit_parser::Type,
        docs: &wit_parser::Docs,
    ) {
        drop((id, name, ty, docs));
    }
}

fn to_js_ident(name: &str) -> &str {
    match name {
        "in" => "in_",
        "import" => "import_",
        s => s,
    }
}
