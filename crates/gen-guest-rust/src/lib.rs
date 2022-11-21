use bindgen_core::{InterfaceGenerator as _, *};
use gen_rust::{FnSig, RustGenerator, TypeMode};
use heck::*;
use std::fmt::Write as _;
use std::{
    collections::HashSet,
    io::{Read, Write},
    mem,
    process::{Command, Stdio},
};
use wit_parser::*;

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Opts {
    /// Whether or not `rustfmt` is executed to format generated code.
    #[cfg_attr(feature = "clap", arg(long))]
    pub rustfmt: bool,

    /// Whether or not the bindings assume interface values are always
    /// well-formed or whether checks are performed.
    #[cfg_attr(feature = "clap", arg(long))]
    pub unchecked: bool,

    /// If true, code generation should avoid any features that depend on `std`.
    #[cfg_attr(feature = "clap", arg(long))]
    pub no_std: bool,

    /// Names of functions to skip generating bindings for.
    #[cfg_attr(feature = "clap", arg(long))]
    pub skip: Vec<String>,
}

impl Opts {
    pub fn build(self) -> Box<dyn WorldGenerator> {
        let mut r = RustWasm::default();
        r.skip = self.skip.iter().cloned().collect();
        r.opts = self;
        Box::new(r)
    }
}

#[derive(Debug, Default)]
struct RustWasm {
    src: Source,
    opts: Opts,
    skip: HashSet<String>,
}

impl WorldGenerator for RustWasm {
    fn import(
        &mut self,
        name: &str,
        iface: &wit_parser::Interface,
        _files: &mut bindgen_core::Files,
    ) {
        let mut gen = InterfaceGenerator::new(self, iface, TypeMode::AllBorrowed("'a"));
        gen.generate_invoke_bindings();
        gen.types();

        for func in iface.functions.iter() {
            gen.generate_guest_import(name, func);
        }

        let snake = name.to_snake_case();
        let module = &gen.src[..];

        uwriteln!(
            self.src,
            "
                #[allow(clippy::all, unused)]
                pub mod {snake} {{
                    {module}
                }}
            "
        );
    }

    fn finish(&mut self, name: &str, files: &mut bindgen_core::Files) {
        let mut src = mem::take(&mut self.src);
        if self.opts.rustfmt {
            let mut child = Command::new("rustfmt")
                .arg("--edition=2018")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("failed to spawn `rustfmt`");
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

        files.push(&format!("{name}.rs"), src.as_bytes());
    }
}

struct InterfaceGenerator<'a> {
    src: Source,
    types: Types,
    gen: &'a mut RustWasm,
    iface: &'a Interface,
    default_param_mode: TypeMode,
}

impl<'a> InterfaceGenerator<'a> {
    pub fn new(gen: &'a mut RustWasm, iface: &'a Interface, default_param_mode: TypeMode) -> Self {
        let mut types = Types::default();
        types.analyze(iface);

        InterfaceGenerator {
            src: Source::default(),
            gen,
            types,
            iface,
            default_param_mode,
        }
    }

    fn generate_invoke_bindings(&mut self) {
        uwriteln!(
            self.src,
            r#"
        #[::guest_rust::wasm_bindgen::prelude::wasm_bindgen]
        extern "C" {{
            #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
            pub async fn invoke(cmd: ::guest_rust::wasm_bindgen::JsValue, args: ::guest_rust::wasm_bindgen::JsValue) -> ::guest_rust::wasm_bindgen::JsValue;
        }}
        "#
        );
    }

    fn generate_guest_import(&mut self, mod_name: &str, func: &Function) {
        if self.gen.skip.contains(&func.name) {
            return;
        }

        let mut sig = FnSig::default();
        sig.async_ = true;

        let param_mode = TypeMode::AllBorrowed("'_");
        match &func.kind {
            FunctionKind::Freestanding => {}
        }
        self.print_signature(func, param_mode, &sig);
        self.src.push_str("{\n");

        if !func.params.is_empty() {
            self.push_str("#[derive(::serde::Serialize)]\n");
            self.push_str("#[serde(rename_all = \"camelCase\")]\n");

            let print_lifetime = func.params.iter().any(|(_, ty)| match ty {
                Type::String => true,
                Type::Id(id) => {
                    let info = self.info(*id);
                    self.lifetime_for(&info, TypeMode::AllBorrowed("'a"))
                        .is_some()
                }
                _ => false,
            });

            self.src.push_str("struct Params");
            self.print_generics(print_lifetime.then_some("'a"));
            self.src.push_str(" {\n");

            for (param, ty) in func.params.iter() {
                // if self.needs_borrow(ty, TypeMode::AllBorrowed("'a")) {
                //     self.src.push_str("#[serde(borrow)]\n")
                // }

                self.src.push_str(&param.to_snake_case());
                self.src.push_str(" : ");
                self.print_ty(ty, TypeMode::AllBorrowed("'a"));
                self.push_str(",\n");
            }

            self.src.push_str("}\n");

            self.src.push_str("let params = Params {");

            for (param, _) in func.params.iter() {
                self.src.push_str(&param.to_snake_case());
                self.src.push_str(",");
            }

            self.src.push_str("};\n");
        }

        uwrite!(
            self.src,
            r#"let raw = invoke(::guest_rust::wasm_bindgen::JsValue::from_str("plugin:{}|{}"),"#,
            mod_name.to_snake_case(),
            func.name.to_snake_case()
        );

        if func.params.is_empty() {
            self.push_str("::guest_rust::wasm_bindgen::JsValue::NULL");
        } else {
            self.push_str("::guest_rust::serde_wasm_bindgen::to_value(&params).unwrap()");
        }

        self.push_str(").await;\n");

        self.src
            .push_str("::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()\n");

        self.src.push_str("}\n");

        match &func.kind {
            FunctionKind::Freestanding => {}
        }
    }
}

impl<'a> RustGenerator<'a> for InterfaceGenerator<'a> {
    fn iface(&self) -> &'a Interface {
        self.iface
    }

    fn use_std(&self) -> bool {
        !self.gen.opts.no_std
    }

    fn info(&self, ty: TypeId) -> TypeInfo {
        self.types.get(ty)
    }

    fn default_param_mode(&self) -> TypeMode {
        self.default_param_mode
    }

    fn push_str(&mut self, s: &str) {
        self.src.push_str(s);
    }

    fn print_borrowed_str(&mut self, lifetime: &'static str) {
        self.push_str("&");
        if lifetime != "'_" {
            self.push_str(lifetime);
            self.push_str(" ");
        }
        self.push_str("str");
    }
}

impl<'a> bindgen_core::InterfaceGenerator<'a> for InterfaceGenerator<'a> {
    fn iface(&self) -> &'a Interface {
        self.iface
    }

    fn type_record(&mut self, id: TypeId, _name: &str, record: &Record, docs: &Docs) {
        self.print_typedef_record(id, record, docs);
    }

    fn type_tuple(&mut self, id: TypeId, _name: &str, tuple: &Tuple, docs: &Docs) {
        self.print_typedef_tuple(id, tuple, docs);
    }

    fn type_flags(&mut self, _id: TypeId, name: &str, flags: &Flags, docs: &Docs) {
        self.push_str("guest_rust::bitflags! {{\n");
        self.print_rustdoc(docs);
        self.push_str(&format!("struct {}: ", name));

        let repr = match flags.repr() {
            FlagsRepr::U8 => "u8",
            FlagsRepr::U16 => "u16",
            FlagsRepr::U32(_) => "u32",
        };
        self.push_str(repr);
        self.push_str(" { \n");

        let mut val: u32 = 0;
        for flag in flags.flags.iter() {
            self.print_rustdoc(&flag.docs);
            self.push_str(&format!(
                "const {} = {:#010b};\n",
                flag.name.TO_SHOUTY_SNEK_CASE(),
                val
            ));

            val = val << 1;
        }

        self.push_str("}\n");
    }

    fn type_variant(&mut self, id: TypeId, _name: &str, variant: &Variant, docs: &Docs) {
        self.print_typedef_variant(id, variant, docs);
    }

    fn type_union(&mut self, id: TypeId, _name: &str, union: &Union, docs: &Docs) {
        self.print_typedef_union(id, union, docs);
    }

    fn type_option(&mut self, id: TypeId, _name: &str, payload: &Type, docs: &Docs) {
        self.print_typedef_option(id, payload, docs);
    }

    fn type_result(&mut self, id: TypeId, _name: &str, result: &Result_, docs: &Docs) {
        self.print_typedef_result(id, result, docs);
    }

    fn type_enum(&mut self, id: TypeId, _name: &str, enum_: &Enum, docs: &Docs) {
        self.print_typedef_enum(id, enum_, docs);
    }

    fn type_alias(&mut self, id: TypeId, _name: &str, ty: &Type, docs: &Docs) {
        self.print_typedef_alias(id, ty, docs);
    }

    fn type_list(&mut self, id: TypeId, _name: &str, ty: &Type, docs: &Docs) {
        self.print_typedef_list(id, ty, docs);
    }

    fn type_builtin(&mut self, _id: TypeId, name: &str, ty: &Type, docs: &Docs) {
        self.print_rustdoc(docs);
        self.src
            .push_str(&format!("pub type {}", name.to_upper_camel_case()));
        self.src.push_str(" = ");
        self.print_ty(ty, TypeMode::Owned);
        self.src.push_str(";\n");
    }
}
