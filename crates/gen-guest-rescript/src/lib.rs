use heck::ToUpperCamelCase;
use heck::*;
use std::fmt::Write as _;
use std::io::{Read, Write};
use std::mem;
use std::process::Command;
use std::{collections::HashSet, process::Stdio};
use tauri_bindgen_core::{uwriteln, Files, InterfaceGenerator as _, Source, WorldGenerator};
use wit_parser::*;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Opts {
    /// Whether or not `rescript format` is executed to format generated code.
    #[cfg_attr(feature = "clap", arg(long))]
    pub fmt: bool,
    /// Names of functions to skip generating bindings for.
    #[cfg_attr(feature = "clap", arg(long))]
    pub skip: Vec<String>,
}

impl Opts {
    pub fn build(self) -> Box<dyn WorldGenerator> {
        let mut r = ReScript::default();
        r.skip = self.skip.iter().cloned().collect();
        r.opts = self;
        Box::new(r)
    }
}

#[derive(Debug, Default)]
struct ReScript {
    src: Source,
    opts: Opts,
    skip: HashSet<String>,
}

impl WorldGenerator for ReScript {
    fn import(&mut self, name: &str, iface: &wit_parser::Interface, _files: &mut Files) {
        let mut gen = InterfaceGenerator::new(self, iface);

        gen.print_intro();
        gen.types();

        for func in iface.functions.iter() {
            gen.generate_guest_import(name, func);
        }

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

    fn finish(&mut self, name: &str, files: &mut Files) {
        let mut src = mem::take(&mut self.src);
        if self.opts.fmt {
            let mut child = Command::new("rescript")
                .arg("format")
                .arg("-stdin")
                .arg(".res")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to spawn `rescript format`");
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

        files.push(
            &format!("{}.res", name.to_upper_camel_case()),
            src.as_bytes(),
        );
    }
}

struct InterfaceGenerator<'a> {
    src: Source,
    gen: &'a mut ReScript,
    iface: &'a Interface,
    // types: Types,
}

impl<'a> InterfaceGenerator<'a> {
    pub fn new(gen: &'a mut ReScript, iface: &'a Interface) -> Self {
        Self {
            src: Source::default(),
            gen,
            iface,
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
        self.push_str(
            r#"
            @scope(("window"))
            external invoke: (~cmd: string, ~payload: 'a=?) => Promise.t<'b> = "__TAURI_INVOKE__"
            "#,
        );
    }

    fn generate_guest_import(&mut self, mod_name: &str, func: &Function) {
        if self.gen.skip.contains(&func.name) {
            return;
        }

        self.print_typedoc(&func.docs);

        // let greet = (name) =>
        self.push_str("let ");
        self.push_str(&func.item_name().to_lower_camel_case());
        self.push_str(" = (");

        let param_start = match &func.kind {
            FunctionKind::Freestanding => 0,
        };

        for (i, (name, ty)) in func.params[param_start..].iter().enumerate() {
            if i > 0 {
                self.push_str(", ");
            }
            self.push_str(to_rescript_ident(&name.to_lower_camel_case()));
            self.push_str(": ");
            self.print_ty(ty);
        }

        self.push_str("): Promise.t<");
        if let Some((ok_ty, _)) = func.results.throws(self.iface) {
            self.print_optional_ty(ok_ty);
        } else {
            match func.results.len() {
                0 => self.push_str("unit"),
                1 => self.print_ty(func.results.iter_types().next().unwrap()),
                _ => {
                    self.push_str("(");
                    for (i, ty) in func.results.iter_types().enumerate() {
                        if i != 0 {
                            self.push_str(", ");
                        }
                        self.print_ty(ty);
                    }
                    self.push_str(")");
                }
            }
        }
        self.push_str("> => {\n");

        self.push_str(&format!(
            r#"invoke(~cmd="plugin:{}|{}", "#,
            mod_name.to_snake_case(),
            func.name.to_snake_case()
        ));

        if !func.params.is_empty() {
            self.push_str("~payload={");
            for (i, (name, _ty)) in func.params.iter().enumerate() {
                if i > 0 {
                    self.push_str(", ");
                }
                self.push_str(&format!(r#""{}": "#, &name.to_lower_camel_case()));
                self.push_str(to_rescript_ident(&name.to_lower_camel_case()));
            }
            self.push_str("}");
        }

        self.push_str(")\n");
        self.push_str("->ignore\n");

        // ->then(msg => {
        //     // TODO
        //     resolve(msg)
        // })

        self.push_str("}\n");
    }

    fn print_ty(&mut self, ty: &Type) {
        match ty {
            Type::Bool => self.push_str("bool"),
            Type::U8 | Type::U16 | Type::U32 | Type::S8 | Type::S16 | Type::S32 => {
                self.push_str("int")
            }
            Type::U64 | Type::S64 => self.push_str("int64"),
            Type::Float32 | Type::Float64 => self.push_str("float"),
            Type::Char => self.push_str("char"),
            Type::String => self.push_str("string"),
            Type::Id(id) => {
                let ty = &self.iface.types[*id];
                if let Some(name) = &ty.name {
                    return self.push_str(&name.to_lower_camel_case());
                }
                match &ty.kind {
                    TypeDefKind::Record(_) => todo!(),
                    TypeDefKind::Flags(_) => todo!(),
                    TypeDefKind::Tuple(ty) => self.print_tuple(ty),
                    TypeDefKind::Variant(_) => todo!(),
                    TypeDefKind::Enum(_) => todo!(),
                    TypeDefKind::Union(_) => todo!(),
                    TypeDefKind::Option(t) => {
                        self.push_str("option<");
                        self.print_ty(t);
                        self.push_str(">");
                    }
                    TypeDefKind::Result(r) => {
                        self.push_str("Result.t<");
                        self.print_optional_ty(r.ok.as_ref());
                        self.push_str(", ");
                        self.print_optional_ty(r.err.as_ref());
                        self.push_str(">");
                    }
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
            None => self.push_str("unit"),
        }
    }

    fn print_tuple(&mut self, tuple: &Tuple) {
        self.push_str("(");
        for (i, ty) in tuple.types.iter().enumerate() {
            if i > 0 {
                self.push_str(", ");
            }
            self.print_ty(ty);
        }
        self.push_str(")");
    }

    fn print_list(&mut self, ty: &Type) {
        match self.array_ty(self.iface, ty) {
            Some(ty) => self.push_str(ty),
            None => {
                self.push_str("array<");
                self.print_ty(ty);
                self.push_str(">");
            }
        }
    }

    fn array_ty(&self, iface: &Interface, ty: &Type) -> Option<&'static str> {
        match ty {
            Type::Bool => None,
            Type::U8 => Some("TypedArray.uint8Array"),
            Type::S8 => Some("TypedArray.int8Array"),
            Type::U16 => Some("TypedArray.uint16Array"),
            Type::S16 => Some("TypedArray.int16Array"),
            Type::U32 => Some("TypedArray.uint32Array"),
            Type::S32 => Some("TypedArray.int32Array"),
            // Type::U64 => Some("BigUint64Array"),
            // Type::S64 => Some("BigInt64Array"),
            Type::U64 | Type::S64 => None,
            Type::Float32 => Some("TypedArray.float32Array"),
            Type::Float64 => Some("TypedArray.float64Array"),
            Type::Char => None,
            Type::String => None,
            Type::Id(id) => match &iface.types[*id].kind {
                TypeDefKind::Type(t) => self.array_ty(iface, t),
                _ => None,
            },
        }
    }

    fn print_name(&mut self, ty: &Type) {
        match ty {
            Type::Bool => self.push_str("Bool"),
            Type::U8 => self.push_str("U8"),
            Type::U16 => self.push_str("U16"),
            Type::U32 => self.push_str("U32"),
            Type::U64 => self.push_str("U64"),
            Type::S8 => self.push_str("I8"),
            Type::S16 => self.push_str("I16"),
            Type::S32 => self.push_str("I32"),
            Type::S64 => self.push_str("I64"),
            Type::Float32 => self.push_str("F32"),
            Type::Float64 => self.push_str("F64"),
            Type::Char => self.push_str("Char"),
            Type::String => self.push_str("String"),
            Type::Id(id) => {
                let ty = &self.iface().types[*id];
                match &ty.name {
                    Some(name) => self.push_str(&name.to_upper_camel_case()),
                    None => match &ty.kind {
                        TypeDefKind::Option(ty) => {
                            self.push_str("Optional");
                            self.print_name(ty);
                        }
                        TypeDefKind::Result(_) => self.push_str("Result"),
                        TypeDefKind::Tuple(_) => self.push_str("Tuple"),
                        TypeDefKind::List(ty) => {
                            self.print_name(ty);
                            self.push_str("List")
                        }
                        TypeDefKind::Future(ty) => {
                            self.print_optional_name(ty.as_ref());
                            self.push_str("Future");
                        }
                        TypeDefKind::Stream(s) => {
                            self.print_optional_name(s.element.as_ref());
                            self.print_optional_name(s.end.as_ref());
                            self.push_str("Stream");
                        }

                        TypeDefKind::Type(ty) => self.print_name(ty),
                        TypeDefKind::Record(_) => self.push_str("Record"),
                        TypeDefKind::Flags(_) => self.push_str("Flags"),
                        TypeDefKind::Variant(_) => self.push_str("Variant"),
                        TypeDefKind::Enum(_) => self.push_str("Enum"),
                        TypeDefKind::Union(_) => self.push_str("Union"),
                    },
                }
            }
        }
    }

    fn print_optional_name(&mut self, ty: Option<&Type>) {
        match ty {
            Some(ty) => self.print_name(ty),
            None => self.push_str("unit"),
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

        if record.fields.is_empty() {
            self.push_str(&format!("type {} = unit\n", name.to_lower_camel_case()));
            return;
        }

        self.push_str(&format!("type {} = {{\n", name.to_lower_camel_case()));

        for field in record.fields.iter() {
            self.print_typedoc(&field.docs);
            self.push_str(&field.name.to_lower_camel_case());
            self.push_str(": ");
            self.print_ty(&field.ty);
            self.push_str(",\n");
        }

        self.push_str("}\n");
    }

    fn type_flags(&mut self, _id: TypeId, _name: &str, _flags: &Flags, _docs: &Docs) {
        todo!()
        //     self.print_typedoc(docs);

        //     match flags.repr() {
        //         FlagsRepr::U8 | FlagsRepr::U16 => {
        //             self.push_str(&format!("export enum {} {{\n", name.to_upper_camel_case()))
        //         }
        //         FlagsRepr::U32(_) => {
        //             self.push_str(&format!(
        //                 "export type {} = typeof {};",
        //                 name.to_upper_camel_case(),
        //                 name.to_upper_camel_case()
        //             ));
        //             self.push_str(&format!(
        //                 "export const {} = {{\n",
        //                 name.to_upper_camel_case()
        //             ))
        //         }
        //     }

        //     let base: usize = 1;
        //     for (i, flag) in flags.flags.iter().enumerate() {
        //         self.print_typedoc(&flag.docs);

        //         match flags.repr() {
        //             FlagsRepr::U8 | FlagsRepr::U16 => self.push_str(&format!(
        //                 "{} = {},\n",
        //                 flag.name.to_upper_camel_case(),
        //                 base << i
        //             )),
        //             FlagsRepr::U32(_) => self.push_str(&format!(
        //                 "{}: BigInt({}),\n",
        //                 flag.name.to_upper_camel_case(),
        //                 base << i
        //             )),
        //         }
        //     }

        //     self.push_str("}\n");
    }

    fn type_tuple(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        tuple: &wit_parser::Tuple,
        docs: &wit_parser::Docs,
    ) {
        self.print_typedoc(docs);
        self.push_str(&format!("type {} = ", name.to_lower_camel_case()));
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

        self.push_str(&format!("type {} = \n", name.to_lower_camel_case()));
        for case in variant.cases.iter() {
            self.push_str(" | ");
            self.print_typedoc(&case.docs);
            self.push_str(&case.name.to_upper_camel_case());

            if let Some(ty) = case.ty {
                self.push_str("(");
                self.print_ty(&ty);
                self.push_str(")");
            }

            self.push_str("\n")
        }

        self.push_str("\n");
    }

    fn type_option(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        payload: &wit_parser::Type,
        docs: &wit_parser::Docs,
    ) {
        self.print_typedoc(docs);
        self.push_str(&format!("type {} = ", name.to_lower_camel_case()));
        self.push_str("option<");
        self.print_ty(payload);
        self.push_str(">\n");
    }

    fn type_result(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        result: &wit_parser::Result_,
        docs: &wit_parser::Docs,
    ) {
        self.print_typedoc(docs);
        self.push_str(&format!("type {} = Result.t<", name.to_lower_camel_case()));
        self.print_optional_ty(result.ok.as_ref());
        self.push_str(", ");
        self.print_optional_ty(result.err.as_ref());
        self.push_str(">\n");
    }

    fn type_union(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        union: &wit_parser::Union,
        docs: &wit_parser::Docs,
    ) {
        self.print_typedoc(docs);

        self.push_str(&format!("type {} = \n", name.to_lower_camel_case()));
        for case in union.cases.iter() {
            // self.print_typedoc(&case.docs);
            self.push_str(" | ");

            self.print_name(&case.ty);
            self.push_str("(");
            self.print_ty(&case.ty);
            self.push_str(")\n");
        }

        self.push_str("\n");
    }

    fn type_enum(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        enum_: &wit_parser::Enum,
        docs: &wit_parser::Docs,
    ) {
        self.print_typedoc(docs);

        self.push_str(&format!("type {} = \n", name.to_lower_camel_case()));
        for case in enum_.cases.iter() {
            self.print_typedoc(&case.docs);
            self.push_str(" | ");
            self.push_str(&case.name.to_upper_camel_case());
            self.push_str("\n")
        }

        self.push_str("\n");
    }

    fn type_alias(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        ty: &wit_parser::Type,
        docs: &wit_parser::Docs,
    ) {
        self.print_typedoc(docs);
        self.push_str(&format!("type {} = ", name.to_lower_camel_case()));
        self.print_ty(ty);
        self.push_str("\n");
    }

    fn type_list(
        &mut self,
        _id: wit_parser::TypeId,
        name: &str,
        ty: &wit_parser::Type,
        docs: &wit_parser::Docs,
    ) {
        self.print_typedoc(docs);
        self.push_str(&format!("type {} = ", name.to_lower_camel_case()));
        self.print_list(ty);
        self.push_str("\n");
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

fn to_rescript_ident(name: &str) -> &str {
    match name {
        "and" => "and_",
        "as" => "as_",
        "assert" => "assert_",
        "constraint" => "constraint_",
        "else" => "else_",
        "exception" => "exception_",
        "external" => "external_",
        "false" => "false_",
        "for" => "for_",
        "if" => "if_",
        "in" => "in_",
        "include" => "include_",
        "lazy" => "lazy_",
        "let" => "let_",
        "module" => "module_",
        "mutable" => "mutable_",
        "of" => "of_",
        "open" => "open_",
        "rec" => "rec_",
        "switch" => "switch_",
        "true" => "true_",
        "try" => "try_",
        "type" => "type_",
        "when" => "when_",
        "while" => "while_",
        "with" => "with_",
        s => s,
    }
}
