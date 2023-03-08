#![allow(clippy::must_use_candidate)]

use heck::{ToKebabCase, ToLowerCamelCase, ToUpperCamelCase};
use std::path::PathBuf;
use tauri_bindgen_core::{postprocess, Generate};
use wit_parser::{Function, Interface, Type, TypeDefKind};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
#[cfg_attr(feature = "clap", clap(group(
    clap::ArgGroup::new("fmt")
        .args(&["prettier", "romefmt"]),
)))]
pub struct Opts {
    /// Run `prettier` to format the generated code. This requires a global installation of `prettier`.
    #[cfg_attr(feature = "clap", clap(long))]
    pub prettier: bool,
    /// Run `rome format` to format the generated code. This formatter is much faster that `prettier`. Requires a global installation of `prettier`.
    #[cfg_attr(feature = "clap", clap(long))]
    pub romefmt: bool,
}

impl Opts {
    pub fn build<'a>(self, interface: &'a Interface) -> JavaScript {
        JavaScript {
            opts: self,
            interface,
        }
    }
}

#[derive(Debug)]
pub struct JavaScript<'a> {
    opts: Opts,
    interface: &'a Interface,
}

impl<'a> JavaScript<'a> {
    pub fn print_function(&self, func: &Function) -> String {
        let docs = self.print_docs(func);

        let ident = func.ident.to_lower_camel_case();

        let params = func
            .params
            .iter()
            .map(|(ident, ty)| {
                let ident = ident.to_lower_camel_case();
                let ty = self.print_ty(ty);

                format!("{ident}: {ty}")
            })
            .collect::<Vec<_>>()
            .join(", ");

        let result = match func.result.len() {
            0 => String::new(),
            1 => {
                let ty = self.print_ty(func.result.types().next().unwrap());
                format!(": Promise<{ty}>")
            }
            _ => {
                let tys = func
                    .result
                    .types()
                    .map(|ty| self.print_ty(ty))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(": Promise<[{tys}]>")
            }
        };

        format!(
            r#"
            {docs}
            export async function {ident} ({params}) {result} {{
            }}
        "#
        )
    }

    fn print_docs(&self, func: &Function) -> String {
        let docs = func
            .docs
            .lines()
            .map(|line| format!(" * {} \n", line))
            .collect::<String>();

        format!("/**\n{docs}*/")
    }

    fn print_ty(&self, ty: &Type) -> String {
        match ty {
            Type::Bool => "boolean".to_string(),
            Type::U8
            | Type::U16
            | Type::U32
            | Type::S8
            | Type::S16
            | Type::S32
            | Type::Float32
            | Type::Float64 => "number".to_string(),
            Type::U64 | Type::S64 => "bigint".to_string(),
            Type::Char | Type::String => "string".to_string(),
            Type::Tuple(types) => {
                let types = types
                    .iter()
                    .map(|ty| self.print_ty(ty))
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("[{types}]")
            }
            Type::List(ty) => {
                let ty = self.array_ty(ty).unwrap_or(self.print_ty(ty));
                format!("{ty}[]")
            }
            Type::Option(ty) => {
                let ty = self.print_ty(ty);

                format!("{ty} | null")
            }
            Type::Result { ok, err } => {
                let ok = ok
                    .as_ref()
                    .map(|ty| self.print_ty(ty))
                    .unwrap_or("_".to_string());
                let err = err
                    .as_ref()
                    .map(|ty| self.print_ty(ty))
                    .unwrap_or("_".to_string());

                format!("Result<{ok}, {err}>")
            }
            Type::Id(id) => self.interface.typedefs[*id].ident.to_upper_camel_case(),
        }
    }

    fn array_ty(&self, ty: &Type) -> Option<String> {
        match ty {
            Type::U8 => Some("Uint8Array".to_string()),
            Type::S8 => Some("Int8Array".to_string()),
            Type::U16 => Some("Uint16Array".to_string()),
            Type::S16 => Some("Int16Array".to_string()),
            Type::U32 => Some("Uint32Array".to_string()),
            Type::S32 => Some("Int32Array".to_string()),
            Type::U64 => Some("BigUint64Array".to_string()),
            Type::S64 => Some("BigInt64Array".to_string()),
            Type::Float32 => Some("Float32Array".to_string()),
            Type::Float64 => Some("Float64Array".to_string()),
            Type::Id(id) => match &self.interface.typedefs[*id].kind {
                TypeDefKind::Alias(t) => self.array_ty(t),
                _ => None,
            },
            Type::Bool
            | Type::Tuple(_)
            | Type::List(_)
            | Type::Option(_)
            | Type::Result { .. }
            | Type::Char
            | Type::String => None,
        }
    }
}

impl<'a> Generate for JavaScript<'a> {
    fn to_file(&self) -> (std::path::PathBuf, String) {
        let mut contents = self
            .interface
            .functions
            .iter()
            .map(|func| self.print_function(func))
            .collect::<Vec<_>>()
            .join("\n");

        if self.opts.prettier {
            postprocess(&mut contents, "prettier", ["--parser=typescript"])
                .expect("failed to run `rome format`");
        } else if self.opts.romefmt {
            postprocess(
                &mut contents,
                "rome",
                ["format", "--stdin-file-path", "index.ts"],
            )
            .expect("failed to run `rome format`");
        }

        let mut filename = PathBuf::from(self.interface.ident.to_kebab_case());
        filename.set_extension("ts");

        (filename, contents)
    }
}
