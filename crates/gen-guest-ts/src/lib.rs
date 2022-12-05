use heck::ToUpperCamelCase;
use heck::*;
use std::collections::HashSet;
use std::fmt::Write as _;
use std::mem;
use tauri_bindgen_core::{
    postprocess, uwriteln, Files, InterfaceGenerator as _, Source, WorldGenerator,
};
use wit_parser::*;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
#[cfg_attr(feature = "clap", command(group(
    clap::ArgGroup::new("fmt")
        .args(["prettier", "romefmt"]),
)))]
pub struct Opts {
    /// Run `prettier` to format the generated code. This requires a global installation of `prettier`.
    #[cfg_attr(feature = "clap", arg(long))]
    pub prettier: bool,
    /// Run `rome format` to format the generated code. This formatter is much faster that `prettier`. Requires a global installation of `prettier`.
    #[cfg_attr(feature = "clap", arg(long))]
    pub romefmt: bool,
    /// Names of functions to skip generating bindings for.
    #[cfg_attr(feature = "clap", arg(long))]
    pub skip: Vec<String>,
}

impl Opts {
    pub fn build(self) -> Box<dyn WorldGenerator> {
        let mut r = TypeScript::default();
        r.skip = self.skip.iter().cloned().collect();
        r.opts = self;
        Box::new(r)
    }
}

#[derive(Debug, Default)]
struct TypeScript {
    src: Source,
    opts: Opts,
    skip: HashSet<String>,
}

impl WorldGenerator for TypeScript {
    fn import(
        &mut self,
        _name: &str,
        iface: &wit_parser::Interface,
        _files: &mut Files,
        world_hash: &str,
    ) {
        let mut gen = InterfaceGenerator::new(self, iface, world_hash);

        gen.print_intro();
        gen.types();

        for func in iface.functions.iter() {
            gen.generate_guest_import(func);
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

    fn finish(&mut self, name: &str, files: &mut Files, _world_hash: &str) {
        let mut src = mem::take(&mut self.src);
        if self.opts.prettier {
            postprocess(src.as_mut_string(), "prettier", ["--parser=typescript"])
                .expect("failed to run `rome format`");
        } else if self.opts.romefmt {
            postprocess(
                src.as_mut_string(),
                "rome",
                ["format", "--stdin-file-path", "index.ts"],
            )
            .expect("failed to run `rome format`");
        }

        files.push(&format!("{name}.ts"), src.as_bytes());
    }
}

struct InterfaceGenerator<'a> {
    src: Source,
    gen: &'a mut TypeScript,
    iface: &'a Interface,
    needs_ty_option: bool,
    needs_ty_result: bool,
    world_hash: &'a str,
}

impl<'a> InterfaceGenerator<'a> {
    pub fn new(gen: &'a mut TypeScript, iface: &'a Interface, world_hash: &'a str) -> Self {
        Self {
            src: Source::default(),
            gen,
            iface,
            needs_ty_option: false,
            needs_ty_result: false,
            world_hash,
        }
    }

    fn push_str(&mut self, s: &str) {
        self.src.push_str(s);
    }

    fn print_typedoc(&mut self, docs: &Docs) {
        if !docs.contents.is_empty() {
            self.push_str("/**\n");
            for line in docs.contents.trim().lines() {
                self.push_str(&format!(" * {}\n", line));
            }
            self.push_str(" */\n");
        }
    }

    fn print_intro(&mut self) {
        self.push_str(
            "
        declare global {
            interface Window {
                __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
            }
        }
        const invoke = window.__TAURI_INVOKE__;",
        );
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

    fn generate_guest_import(&mut self, func: &Function) {
        if self.gen.skip.contains(&func.name) {
            return;
        }

        self.print_typedoc(&func.docs);

        self.push_str("export async function ");
        self.push_str(&func.name.to_lower_camel_case());
        self.push_str("(");

        for (i, (name, ty)) in func.params.iter().enumerate() {
            if i > 0 {
                self.push_str(", ");
            }
            self.push_str(to_js_ident(&name.to_lower_camel_case()));
            self.push_str(": ");
            self.print_ty(ty);
        }
        self.push_str("): Promise<");
        if let Some((ok_ty, _)) = func.results.throws() {
            self.print_optional_ty(ok_ty);
        } else {
            match func.results.len() {
                0 => self.push_str("void"),
                1 => self.print_ty(func.results.types().next().unwrap()),
                _ => {
                    self.push_str("[");
                    for (i, ty) in func.results.types().enumerate() {
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

        if let Some((ok_ty, _)) = func.results.throws() {
            self.print_optional_ty(ok_ty);
        } else {
            match func.results.len() {
                0 => self.push_str("void"),
                1 => self.print_ty(func.results.types().next().unwrap()),
                _ => {
                    self.push_str("[");
                    for (i, ty) in func.results.types().enumerate() {
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

        self.push_str(&format!("\"plugin:{}|{}\",", self.world_hash, func.name));

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
            Type::Tuple(ty) => self.print_tuple(ty),
            Type::List(ty) => self.print_list(ty),
            Type::Option(ty) => {
                if self.is_nullable(ty) {
                    self.needs_ty_option = true;
                    self.push_str("Option<");
                    self.print_ty(ty);
                    self.push_str(">");
                } else {
                    self.print_ty(ty);
                    self.push_str(" | null");
                }
            }
            Type::Result(r) => {
                self.needs_ty_result = true;
                self.push_str("Result<");
                self.print_optional_ty(r.ok.as_ref());
                self.push_str(", ");
                self.print_optional_ty(r.err.as_ref());
                self.push_str(">");
            }
            Type::Id(id) => {
                let ty = &self.iface.types[*id];

                self.push_str(&ty.name.to_upper_camel_case());
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
            Type::Id(id) => match &iface.types[*id].kind {
                TypeDefKind::Type(t) => self.array_ty(iface, t),
                _ => None,
            },
            Type::Tuple(_)
            | Type::List(_)
            | Type::Option(_)
            | Type::Result(_)
            | Type::Char
            | Type::String => None,
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
            // TypeDefKind::Option(ty) => !self.is_nullable(ty),
            TypeDefKind::Type(t) => self.is_nullable(t),
            _ => false,
        }
    }
}

impl<'a> tauri_bindgen_core::InterfaceGenerator<'a> for InterfaceGenerator<'a> {
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

    fn type_flags(&mut self, _id: TypeId, name: &str, flags: &Flags, docs: &Docs) {
        self.print_typedoc(docs);

        match flags.repr() {
            Int::U8 | Int::U16 => {
                self.push_str(&format!("export enum {} {{\n", name.to_upper_camel_case()))
            }
            Int::U32 | Int::U64 => {
                self.push_str(&format!(
                    "export type {} = typeof {};",
                    name.to_upper_camel_case(),
                    name.to_upper_camel_case()
                ));
                self.push_str(&format!(
                    "export const {} = {{\n",
                    name.to_upper_camel_case()
                ))
            }
        }

        let base: usize = 1;
        for (i, flag) in flags.flags.iter().enumerate() {
            self.print_typedoc(&flag.docs);

            match flags.repr() {
                Int::U8 | Int::U16 => self.push_str(&format!(
                    "{} = {},\n",
                    flag.name.to_upper_camel_case(),
                    base << i
                )),
                Int::U32 | Int::U64 => self.push_str(&format!(
                    "{}: BigInt({}),\n",
                    flag.name.to_upper_camel_case(),
                    base << i
                )),
            }
        }

        self.push_str("}\n");
    }

    // fn type_tuple(
    //     &mut self,
    //     _id: wit_parser::TypeId,
    //     name: &str,
    //     tuple: &wit_parser::Tuple,
    //     docs: &wit_parser::Docs,
    // ) {
    //     self.print_typedoc(docs);
    //     self.push_str(&format!("export type {} = ", name.to_upper_camel_case()));
    //     self.print_tuple(tuple);
    //     self.push_str(";\n");
    // }

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

            if let Some(ty) = &case.ty {
                self.push_str("val: ");
                self.print_ty(&ty);
                self.push_str(",\n");
            }
            self.push_str("}\n");
        }
    }

    // fn type_option(
    //     &mut self,
    //     _id: wit_parser::TypeId,
    //     name: &str,
    //     payload: &wit_parser::Type,
    //     docs: &wit_parser::Docs,
    // ) {
    //     self.print_typedoc(docs);
    //     self.push_str(&format!("export type {} = ", name.to_upper_camel_case()));
    //     if self.is_nullable(payload) {
    //         self.needs_ty_option = true;
    //         self.push_str("Option<");
    //         self.print_ty(payload);
    //         self.push_str(">");
    //     } else {
    //         self.print_ty(payload);
    //         self.push_str(" | null");
    //     }
    //     self.push_str(";\n");
    // }

    // fn type_result(
    //     &mut self,
    //     _id: wit_parser::TypeId,
    //     name: &str,
    //     result: &wit_parser::Result_,
    //     docs: &wit_parser::Docs,
    // ) {
    //     self.needs_ty_result = true;
    //     self.print_typedoc(docs);
    //     self.push_str(&format!(
    //         "export type {} = Result<",
    //         name.to_upper_camel_case()
    //     ));
    //     self.print_optional_ty(result.ok.as_ref());
    //     self.push_str(", ");
    //     self.print_optional_ty(result.err.as_ref());
    //     self.push_str(">;\n");
    // }

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

    // fn type_list(
    //     &mut self,
    //     _id: wit_parser::TypeId,
    //     name: &str,
    //     ty: &wit_parser::Type,
    //     docs: &wit_parser::Docs,
    // ) {
    //     self.print_typedoc(docs);
    //     self.push_str(&format!("export type {} = ", name.to_upper_camel_case()));
    //     self.print_list(ty);
    //     self.push_str(";\n");
    // }

    // fn type_builtin(
    //     &mut self,
    //     id: wit_parser::TypeId,
    //     name: &str,
    //     ty: &wit_parser::Type,
    //     docs: &wit_parser::Docs,
    // ) {
    //     drop((id, name, ty, docs));
    // }
}

fn to_js_ident(name: &str) -> &str {
    match name {
        "in" => "in_",
        "import" => "import_",
        s => s,
    }
}
