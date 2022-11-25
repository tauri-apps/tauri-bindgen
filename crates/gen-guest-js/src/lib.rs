use heck::*;
use std::collections::HashSet;
use std::fmt::Write as _;
use std::mem;
use tauri_bindgen_core::{postprocess, uwriteln, Files, Source, WorldGenerator};
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
        _name: &str,
        iface: &wit_parser::Interface,
        _files: &mut Files,
        world_hash: &str,
    ) {
        let mut gen = InterfaceGenerator::new(self, iface, world_hash);

        gen.print_intro();

        for func in iface.functions.iter() {
            gen.generate_guest_import(func);
        }

        let module = &gen.src[..];
        uwriteln!(self.src, "{module}");
    }

    fn finish(&mut self, name: &str, files: &mut Files, _world_hash: &str) {
        let mut src = mem::take(&mut self.src);
        if self.opts.prettier {
            postprocess(src.as_mut_string(), "prettier", ["--parser=babel"])
                .expect("failed to run `prettier`");
        } else if self.opts.romefmt {
            postprocess(
                src.as_mut_string(),
                "rome",
                ["format", "--stdin-file-path", "index.js"],
            )
            .expect("failed to run `rome format`");
        }

        files.push(&format!("{name}.js"), src.as_bytes());
    }
}

struct InterfaceGenerator<'a> {
    src: Source,
    gen: &'a mut JavaScript,
    iface: &'a Interface,
    world_hash: &'a str,
}

impl<'a> InterfaceGenerator<'a> {
    pub fn new(gen: &'a mut JavaScript, iface: &'a Interface, world_hash: &'a str) -> Self {
        Self {
            src: Source::default(),
            gen,
            iface,
            world_hash,
        }
    }

    fn push_str(&mut self, s: &str) {
        self.src.push_str(s);
    }

    fn print_intro(&mut self) {
        self.push_str("const invoke = window.__TAURI_INVOKE__;\n");
    }

    fn print_jsdoc(&mut self, func: &Function) {
        if func.docs.contents.is_none() && func.params.is_empty() && func.results.len() == 0 {
            return;
        }

        self.push_str("/**\n");

        if let Some(docs) = &func.docs.contents {
            for line in docs.trim().lines() {
                self.push_str(&format!(" * {}\n", line));
            }
        }

        for (param, ty) in func.params.iter() {
            self.push_str(" * @param {");
            self.print_ty(ty);
            self.push_str("} ");
            self.push_str(param);
            self.push_str("\n");
        }

        match func.results.len() {
            0 => {}
            1 => {
                self.push_str(" * @returns {Promise<");
                self.print_ty(func.results.iter_types().next().unwrap());
                self.push_str(">}\n");
            }
            _ => {
                self.push_str(" * @returns {Promise<[");
                for (i, ty) in func.results.iter_types().enumerate() {
                    if i != 0 {
                        self.push_str(", ");
                    }
                    self.print_ty(ty);
                }
                self.push_str("]>}\n");
            }
        }

        self.push_str(" */\n");
    }

    fn generate_guest_import(&mut self, func: &Function) {
        if self.gen.skip.contains(&func.name) {
            return;
        }

        self.print_jsdoc(func);

        self.push_str("export async function ");
        self.push_str(&func.item_name().to_lower_camel_case());
        self.push_str("(");

        let param_start = match &func.kind {
            FunctionKind::Freestanding => 0,
        };

        for (i, (name, _)) in func.params[param_start..].iter().enumerate() {
            if i > 0 {
                self.push_str(", ");
            }
            self.push_str(to_js_ident(&name.to_lower_camel_case()));
        }
        self.push_str(") {\n");

        if func.results.len() > 0 {
            self.push_str("const result = ");
        }

        self.push_str(&format!(
            "await invoke(\"plugin:{}|{}\",",
            self.world_hash,
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
                    TypeDefKind::Option(t) => {
                        if self.is_nullable(t) {
                            self.push_str("Option<");
                            self.print_ty(t);
                            self.push_str(">");
                        } else {
                            self.print_ty(t);
                            self.push_str(" | null");
                        }
                    }
                    TypeDefKind::Result(r) => {
                        // self.push_str("Result<");
                        self.print_optional_ty(r.ok.as_ref());
                        // self.push_str(", ");
                        // self.print_optional_ty(r.err.as_ref());
                        // self.push_str(">");
                    }
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

fn to_js_ident(name: &str) -> &str {
    match name {
        "in" => "in_",
        "import" => "import_",
        s => s,
    }
}
