use heck::*;
use std::{fmt::Write as _, mem};
use tauri_bindgen_core::{
    postprocess, uwrite, uwriteln, Files, InterfaceGenerator as _, Source, TypeInfo, Types,
    WorldGenerator,
};
use tauri_bindgen_gen_rust::{FnSig, RustFlagsRepr, RustGenerator, TypeMode};
use wit_parser::*;

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Opts {
    /// Whether or not `rustfmt` is executed to format generated code.
    #[cfg_attr(feature = "clap", arg(long))]
    pub rustfmt: bool,

    /// Whether or not to emit `tracing` macro calls on function entry/exit.
    #[cfg_attr(feature = "clap", arg(long))]
    pub tracing: bool,

    /// Whether or not to use async rust functions and traits.
    #[cfg_attr(feature = "clap", arg(long = "async"))]
    pub async_: bool,
}

impl Opts {
    pub fn build(self) -> Box<dyn WorldGenerator> {
        let mut r = Host::default();
        r.opts = self;
        Box::new(r)
    }
}

#[derive(Debug, Default)]
struct Host {
    src: Source,
    opts: Opts,
    imports: Vec<String>,
}

impl WorldGenerator for Host {
    fn import(&mut self, name: &str, iface: &Interface, _files: &mut Files, world_hash: &str) {
        let mut gen = InterfaceGenerator::new(self, iface, TypeMode::Owned, world_hash);
        gen.types();
        gen.generate_invoke_handler(name);

        let snake = name.to_snake_case();
        let module = &gen.src[..];

        uwriteln!(
            self.src,
            "
                #[allow(clippy::all)]
                pub mod {snake} {{
                    {module}
                }}
            "
        );

        self.imports.push(snake); // TODO
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
    gen: &'a mut Host,
    iface: &'a Interface,
    default_param_mode: TypeMode,
    types: Types,
    world_hash: &'a str,
}

impl<'a> InterfaceGenerator<'a> {
    fn new(
        gen: &'a mut Host,
        iface: &'a Interface,
        default_param_mode: TypeMode,
        world_hash: &'a str,
    ) -> InterfaceGenerator<'a> {
        let mut types = Types::default();
        types.analyze(iface);
        InterfaceGenerator {
            src: Source::default(),
            gen,
            iface,
            types,
            default_param_mode,
            world_hash,
        }
    }

    pub(crate) fn generate_invoke_handler(&mut self, name: &str) {
        let camel = name.to_upper_camel_case();

        if self.gen.opts.async_ {
            uwriteln!(self.src, "#[::tauri_bindgen_host::async_trait]")
        }

        uwriteln!(self.src, "pub trait {camel}: Sized {{");

        for func in self.iface.functions.iter() {
            let mut fnsig = FnSig::default();

            fnsig.async_ = self.gen.opts.async_;
            fnsig.private = true;
            fnsig.self_arg = Some("&self".to_string());

            self.print_docs_and_params(func, TypeMode::Owned, &fnsig);
            self.push_str(" -> ");

            self.push_str("::tauri_bindgen_host::anyhow::Result<");
            self.print_result_ty(&func.results, TypeMode::Owned);
            self.push_str(">;\n");
        }

        uwriteln!(self.src, "}}");

        uwriteln!(
            self.src,
            "
                pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
                where
                    U: {camel} + Send + Sync + 'static,
                    R: ::tauri_bindgen_host::tauri::Runtime + 'static
                {{
            "
        );

        uwriteln!(self.src, "move |invoke| {{");

        if self.gen.opts.tracing {
            uwriteln!(
                self.src,
                r#"let span = ::tauri_bindgen_host::tracing::span!(
                    ::tauri_bindgen_host::tracing::Level::TRACE,
                    "tauri-bindgen invoke handler",
                    module = "{name}", function = invoke.message.command(), payload = ?invoke.message.payload()
                );
                let _enter = span.enter();
               "#
            );
        }

        uwriteln!(self.src, "match invoke.message.command() {{");

        for func in self.iface.functions.iter() {
            uwrite!(self.src, "\"{}\" => {{", &func.name);
            uwriteln!(
                self.src,
                "
            #[allow(unused_variables)]
            let ::tauri_bindgen_host::tauri::Invoke {{
                message: __tauri_message__,
                resolver: __tauri_resolver__,
            }} = invoke;"
            );

            for (param, _) in func.params.iter() {
                let func_name = &func.name;

                uwriteln!(
                    self.src,
                    r#"let {snake_param} = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {{
                        name: "{func_name}",
                        key: "{camel_param}",
                        message: &__tauri_message__,
                    }}) {{
                        Ok(arg) => arg,
                        Err(err) => {{"#,
                    snake_param = param.to_snake_case(),
                    camel_param = param.to_lower_camel_case()
                );

                if self.gen.opts.tracing {
                    uwriteln!(
                        self.src,
                        r#"::tauri_bindgen_host::tracing::error!(module = "{name}", function = "{func_name}", "Invoke handler returned error {{:?}}", err);"#
                    );
                }

                uwriteln!(
                    self.src,
                    r#"return __tauri_resolver__.invoke_error(err);
                        }},
                    }};
                    "#
                );
            }

            if self.gen.opts.async_ {
                uwriteln!(
                    self.src,
                    "
                __tauri_resolver__
                .respond_async(async move {{
                "
                )
            }

            uwriteln!(self.src, "let result = ctx.{}(", func.name.to_snake_case());

            for (param, _) in func.params.iter() {
                self.src.push_str(&param.to_snake_case());
                self.src.push_str(", ");
            }

            uwriteln!(self.src, ");");

            if self.gen.opts.async_ {
                uwriteln!(
                    self.src,
                    "
                    result.await.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                    }});
                "
                )
            } else {
                uwriteln!(self.src, "
                    __tauri_resolver__.respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
                ")
            }

            uwriteln!(self.src, "}},");
        }

        uwriteln!(
            self.src,
            r#"
            #[cfg(debug_assertions)]
            "{}" => {{
            invoke.resolver.respond(Ok(()));
        }},
        "#,
            self.world_hash
        );

        uwriteln!(self.src, "func_name => {{");
        if self.gen.opts.tracing {
            uwriteln!(
                self.src,
                r#"::tauri_bindgen_host::tracing::error!(module = "{name}", function = func_name, "Not Found");"#
            );
        }
        uwriteln!(self.src, "invoke.resolver.reject(\"Not Found\")");
        uwriteln!(self.src, "}}");
        uwriteln!(self.src, "}}");
        uwriteln!(self.src, "}}");

        uwriteln!(self.src, "}}");
    }

    fn print_result_ty(&mut self, results: &Results, mode: TypeMode) {
        match results {
            Results::Named(rs) => match rs.len() {
                0 => self.push_str("()"),
                1 => self.print_ty(&rs[0].1, mode),
                _ => {
                    self.push_str("(");
                    for (i, (_, ty)) in rs.iter().enumerate() {
                        if i > 0 {
                            self.push_str(", ")
                        }
                        self.print_ty(ty, mode)
                    }
                    self.push_str(")");
                }
            },
            Results::Anon(ty) => self.print_ty(ty, mode),
        }
    }
}

impl<'a> RustGenerator<'a> for InterfaceGenerator<'a> {
    fn iface(&self) -> &'a Interface {
        self.iface
    }

    fn push_str(&mut self, s: &str) {
        self.src.push_str(s)
    }

    fn print_borrowed_str(&mut self, lifetime: &'static str) {
        self.push_str("&");
        if lifetime != "'_" {
            self.push_str(lifetime);
            self.push_str(" ");
        }
        self.push_str(" str");
    }

    fn default_param_mode(&self) -> TypeMode {
        self.default_param_mode
    }

    fn info(&self, ty: TypeId) -> TypeInfo {
        self.types.get(ty)
    }
}

impl<'a> tauri_bindgen_core::InterfaceGenerator<'a> for InterfaceGenerator<'a> {
    fn iface(&self) -> &'a Interface {
        self.iface
    }

    fn type_record(&mut self, id: TypeId, _name: &str, record: &wit_parser::Record, docs: &Docs) {
        self.print_typedef_record(id, record, docs, get_serde_attrs);
    }

    fn type_flags(&mut self, id: TypeId, name: &str, flags: &Flags, docs: &Docs) {
        self.push_str("::tauri_bindgen_host::bitflags::bitflags! {\n");
        self.print_rustdoc(docs);

        let repr = RustFlagsRepr::new(&flags);
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

    fn type_tuple(&mut self, id: TypeId, _name: &str, tuple: &wit_parser::Tuple, docs: &Docs) {
        self.print_typedef_tuple(id, tuple, docs);
    }

    fn type_variant(
        &mut self,
        id: TypeId,
        _name: &str,
        variant: &wit_parser::Variant,
        docs: &Docs,
    ) {
        self.print_typedef_variant(id, variant, docs, get_serde_attrs);
    }

    fn type_option(&mut self, id: TypeId, _name: &str, payload: &Type, docs: &Docs) {
        self.print_typedef_option(id, payload, docs);
    }

    fn type_result(&mut self, id: TypeId, _name: &str, result: &wit_parser::Result_, docs: &Docs) {
        self.print_typedef_result(id, result, docs);
    }

    fn type_union(&mut self, id: TypeId, _name: &str, union: &wit_parser::Union, docs: &Docs) {
        self.print_typedef_union(id, union, docs, get_serde_attrs);
    }

    fn type_enum(&mut self, id: TypeId, _name: &str, enum_: &wit_parser::Enum, docs: &Docs) {
        self.print_typedef_enum(id, enum_, docs, get_serde_attrs);
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

fn get_serde_attrs(name: &str, uses_two_names: bool, info: TypeInfo) -> Option<String> {
    let mut attrs = vec![];

    match (info.param, info.result) {
        (true, false) => {
            attrs.push("::tauri_bindgen_host::serde::Deserialize");
        }
        (true, true) if uses_two_names && name.ends_with("Param") => {
            attrs.push("::tauri_bindgen_host::serde::Deserialize");
        }
        (false, true) => {
            attrs.push("::tauri_bindgen_host::serde::Serialize");
        }
        (true, true) if uses_two_names && name.ends_with("Result") => {
            attrs.push("::tauri_bindgen_host::serde::Serialize");
        }
        (true, true) => {
            attrs.push("::tauri_bindgen_host::serde::Serialize");
            attrs.push("::tauri_bindgen_host::serde::Deserialize");
        }
        _ => return None,
    }

    Some(format!("#[derive({})]\n", attrs.join(", ")))
}
