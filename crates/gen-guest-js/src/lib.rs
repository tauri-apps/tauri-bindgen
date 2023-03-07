#![allow(clippy::must_use_candidate)]

use heck::{ToKebabCase, ToLowerCamelCase, ToUpperCamelCase};
use std::path::PathBuf;
use tauri_bindgen_core::{postprocess, Generate};
use wit_parser::{Function, Type, TypeDefKind};

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
    pub fn build(self) -> JavaScript {
        JavaScript {
            opts: self,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct JavaScript {
    opts: Opts,
}

impl JavaScript {
    pub fn print_function(&self, func: &Function) -> String {
        let docs = self.print_docs(func);

        let ident = func.ident.to_lower_camel_case();

        let params = func
            .params
            .iter()
            .map(|(name, _)| name.to_lower_camel_case())
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            r#"
            {docs}
            export async function {ident} ({params}) {{
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

        let param_docs = func
            .params
            .iter()
            .map(|(name, ty)| {
                let ident = &name.to_lower_camel_case();
                let ty = self.print_ty(ty);

                format!("* @param {{{ty}}} {ident} \n")
            })
            .collect::<String>();

        let result_docs = match func.result.len() {
            0 => "".to_string(),
            1 => {
                let ty = self.print_ty(func.result.types().next().unwrap());
                format!("* @returns {{Promise<{ty}>}} \n")
            }
            _ => {
                let types = func
                    .result
                    .types()
                    .map(|ty| self.print_ty(ty))
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("* @returns {{Promise<[{types}]>}} \n")
            }
        };

        format!("/**\n{docs}{param_docs}{result_docs}*/")
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
                let ty = array_ty(ty).unwrap_or(self.print_ty(ty));
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
            Type::Id(typedef) => typedef.borrow().ident.to_upper_camel_case(),
        }
    }
}

impl Generate for JavaScript {
    fn to_string(&self, iface: &wit_parser::Interface) -> (std::path::PathBuf, String) {
        let mut contents = iface
            .functions
            .iter()
            .map(|func| self.print_function(func))
            .collect::<Vec<_>>()
            .join("\n");

        if self.opts.prettier {
            postprocess(&mut contents, "prettier", ["--parser=babel"])
                .expect("failed to run `prettier`");
        } else if self.opts.romefmt {
            postprocess(
                &mut contents,
                "rome",
                ["format", "--stdin-file-path", "index.js"],
            )
            .expect("failed to run `rome format`");
        }

        let mut filename = PathBuf::from(iface.ident.to_kebab_case());
        filename.set_extension("js");

        (filename, contents)
    }
}

fn array_ty(ty: &Type) -> Option<String> {
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
        Type::Id(typedef) => match &typedef.borrow().kind {
            TypeDefKind::Alias(t) => array_ty(t),
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
