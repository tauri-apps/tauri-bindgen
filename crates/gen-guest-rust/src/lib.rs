use heck::*;
use std::fmt::Write as _;
use std::{collections::HashSet, mem};
use tauri_bindgen_core::{InterfaceGenerator as _, *};
use tauri_bindgen_gen_rust::{FnSig, RustFlagsRepr, RustGenerator, TypeMode};
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
        Box::new(RustWasm {
            skip: self.skip.iter().cloned().collect(),
            opts: self,
            ..Default::default()
        })
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
        _files: &mut Files,
        world_hash: &str,
    ) {
        let mut gen = InterfaceGenerator::new(self, iface, TypeMode::AllBorrowed("'a"), world_hash);

        gen.types();

        for func in iface.functions.iter() {
            gen.generate_guest_import(func);
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

    fn finish(&mut self, name: &str, files: &mut Files, _world_hash: &str) {
        let mut src = mem::take(&mut self.src);
        if self.opts.rustfmt {
            postprocess(src.as_mut_string(), "rustfmt", ["--edition=2018"])
                .expect("failed to run `rustfmt`");
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
    world_hash: &'a str,
}

impl<'a> InterfaceGenerator<'a> {
    pub fn new(
        gen: &'a mut RustWasm,
        iface: &'a Interface,
        default_param_mode: TypeMode,
        world_hash: &'a str,
    ) -> Self {
        let mut types = Types::default();
        types.analyze(iface);

        InterfaceGenerator {
            src: Source::default(),
            gen,
            types,
            iface,
            default_param_mode,
            world_hash,
        }
    }

    fn generate_guest_import(&mut self, func: &Function) {
        if self.gen.skip.contains(&func.name) {
            return;
        }

        let sig = FnSig {
            async_: true,
            ..Default::default()
        };

        let param_mode = TypeMode::AllBorrowed("'_");

        self.print_signature(func, param_mode, &sig);
        self.src.push_str(" {\n");

        let lifetime = func.params.iter().any(|(_, ty)| self.needs_lifetime(ty));

        if !func.params.is_empty() {
            self.push_str("#[derive(::serde::Serialize)]\n");
            self.push_str("#[serde(rename_all = \"camelCase\")]\n");
            self.src.push_str("struct Params");
            self.print_generics(lifetime.then_some("'a"));
            self.src.push_str(" {\n");

            for (param, ty) in func.params.iter() {
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
            r#"::tauri_bindgen_guest_rust::invoke("plugin:{}|{}", "#,
            self.world_hash,
            func.name
        );

        if func.params.is_empty() {
            self.push_str("()");
        } else {
            self.push_str("&params");
        }

        self.push_str(").await.unwrap()\n");

        self.src.push_str("}\n");
    }

    fn needs_lifetime(&self, ty: &Type) -> bool {
        match ty {
            Type::String => true,
            Type::Tuple(ty) => ty.types.iter().any(|ty| self.needs_lifetime(ty)),
            Type::List(_) => true,
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
                self.lifetime_for(&info, TypeMode::AllBorrowed("'a"))
                    .is_some()
            }
            _ => false,
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

impl<'a> tauri_bindgen_core::InterfaceGenerator<'a> for InterfaceGenerator<'a> {
    fn iface(&self) -> &'a Interface {
        self.iface
    }

    fn type_record(&mut self, id: TypeId, _name: &str, record: &Record, docs: &Docs) {
        self.print_typedef_record(id, record, docs, get_serde_attrs);
    }

    fn type_flags(&mut self, id: TypeId, name: &str, flags: &Flags, docs: &Docs) {
        self.push_str("::tauri_bindgen_guest_rust::bitflags::bitflags! {\n");
        self.print_rustdoc(docs);

        let repr = RustFlagsRepr::new(flags);
        let info = self.info(id);

        if let Some(attrs) = get_serde_attrs(name, self.uses_two_names(&info), info) {
            self.push_str(&attrs);
        }

        self.push_str(&format!(
            "pub struct {}: {} {{\n",
            name.to_upper_camel_case(),
            repr
        ));

        for (i, flag) in flags.flags.iter().enumerate() {
            self.print_rustdoc(&flag.docs);
            self.src.push_str(&format!(
                "const {} = 1 << {};\n",
                flag.name.to_shouty_snake_case(),
                i,
            ));
        }

        self.push_str("}\n}\n");
    }

    fn type_variant(&mut self, id: TypeId, _name: &str, variant: &Variant, docs: &Docs) {
        self.print_typedef_variant(id, variant, docs, get_serde_attrs);
    }

    fn type_union(&mut self, id: TypeId, _name: &str, union: &Union, docs: &Docs) {
        self.print_typedef_union(id, union, docs, get_serde_attrs);
    }

    fn type_enum(&mut self, id: TypeId, _name: &str, enum_: &Enum, docs: &Docs) {
        self.print_typedef_enum(id, enum_, docs, get_serde_attrs);
    }

    fn type_alias(&mut self, id: TypeId, _name: &str, ty: &Type, docs: &Docs) {
        self.print_typedef_alias(id, ty, docs);
    }
}

fn get_serde_attrs(name: &str, uses_two_names: bool, info: TypeInfo) -> Option<String> {
    let mut attrs = vec![];

    if uses_two_names {
        if name.ends_with("Param") {
            attrs.push("::serde::Serialize");
        }
        if name.ends_with("Result") {
            attrs.push("::serde::Deserialize")
        }
    } else {
        if info.contains(TypeInfo::PARAM) {
            attrs.push("::serde::Serialize");
        }
        if info.contains(TypeInfo::RESULT) {
            attrs.push("::serde::Deserialize")
        }
    }

    Some(format!("#[derive({})]\n", attrs.join(", ")))
}
