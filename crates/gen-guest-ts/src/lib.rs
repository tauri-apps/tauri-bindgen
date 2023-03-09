#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc
)]

use heck::{ToKebabCase, ToLowerCamelCase, ToUpperCamelCase};
use std::path::PathBuf;
use tauri_bindgen_core::{postprocess, Generate, GeneratorBuilder};
use wit_parser::{Function, Interface, Type, TypeDefId, TypeDefKind};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
#[cfg_attr(feature = "clap", clap(group(
    clap::ArgGroup::new("fmt")
        .args(&["prettier", "romefmt"]),
)))]
pub struct Builder {
    /// Run `prettier` to format the generated code. This requires a global installation of `prettier`.
    #[cfg_attr(feature = "clap", clap(long))]
    pub prettier: bool,
    /// Run `rome format` to format the generated code. This formatter is much faster that `prettier`. Requires a global installation of `prettier`.
    #[cfg_attr(feature = "clap", clap(long))]
    pub romefmt: bool,
}

impl GeneratorBuilder for Builder {
    fn build(self, interface: Interface) -> Box<dyn Generate> {
        Box::new(TypeScript {
            opts: self,
            interface,
        })
    }
}

#[derive(Debug)]
pub struct TypeScript {
    opts: Builder,
    interface: Interface,
}

impl TypeScript {
    pub fn print_function(&self, func: &Function) -> String {
        let docs = print_docs(&func.docs);

        let ident = func.ident.to_lower_camel_case();

        let params = func
            .params
            .iter()
            .map(|(ident, ty)| {
                let ident = ident.to_lower_camel_case();
                let ty = self.print_type(ty);

                format!("{ident}: {ty}")
            })
            .collect::<Vec<_>>()
            .join(", ");

        let result = match func.result.len() {
            0 => String::new(),
            1 => {
                let ty = self.print_type(func.result.types().next().unwrap());
                format!(": Promise<{ty}>")
            }
            _ => {
                let tys = func
                    .result
                    .types()
                    .map(|ty| self.print_type(ty))
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

    fn print_type(&self, ty: &Type) -> String {
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
                    .map(|ty| self.print_type(ty))
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("[{types}]")
            }
            Type::List(ty) => {
                let ty = self.array_ty(ty).unwrap_or(self.print_type(ty));
                format!("{ty}[]")
            }
            Type::Option(ty) => {
                let ty = self.print_type(ty);

                format!("{ty} | null")
            }
            Type::Result { ok, err } => {
                let ok = ok
                    .as_ref()
                    .map_or("_".to_string(), |ty| self.print_type(ty));
                let err = err
                    .as_ref()
                    .map_or("_".to_string(), |ty| self.print_type(ty));

                format!("Result<{ok}, {err}>")
            }
            Type::Id(id) => self.interface.typedefs[*id].ident.to_upper_camel_case(),
        }
    }

    fn print_typedef(&self, id: TypeDefId) -> String {
        let typedef = &self.interface.typedefs[id];
        let ident = &typedef.ident.to_upper_camel_case();
        let docs = print_docs(&typedef.docs);

        match &typedef.kind {
            TypeDefKind::Alias(ty) => {
                let ty = self.print_type(ty);

                format!("{docs}\nexport type {ident} = {ty};\n")
            }
            TypeDefKind::Record(fields) => {
                let fields: String = fields
                    .iter()
                    .map(|field| {
                        let docs = print_docs(&field.docs);
                        let ident = field.ident.to_lower_camel_case();
                        let ty = self.print_type(&field.ty);

                        format!("{docs}\n{ident}: {ty},\n")
                    })
                    .collect();

                format!("{docs}\nexport interface {ident} {{ {fields} }}\n")
            }
            TypeDefKind::Flags(fields) => {
                let fields: String = fields
                    .iter()
                    .enumerate()
                    .map(|(i, field)| {
                        let docs = print_docs(&field.docs);
                        let ident = field.ident.to_upper_camel_case();
                        let value: u64 = 2 << i;

                        format!("{docs}\n{ident} = {value},\n")
                    })
                    .collect();

                format!("{docs}\nexport enum {ident} {{ {fields} }}\n")
            }
            TypeDefKind::Variant(cases) => {
                let interfaces: String = cases
                    .iter()
                    .enumerate()
                    .map(|(i, case)| {
                        let docs = print_docs(&case.docs);
                        let case_ident = case.ident.to_upper_camel_case();
                        let value = case
                            .ty
                            .as_ref()
                            .map(|ty| {
                                let ty = self.print_type(ty);
                                format!(", value: {ty}")
                            })
                            .unwrap_or_default();

                        format!(
                            "{docs}\nexport interface {ident}{case_ident} {{ tag: {i}{value} }}\n"
                        )
                    })
                    .collect();

                let cases: String = cases
                    .iter()
                    .map(|case| {
                        let docs = print_docs(&case.docs);
                        let case_ident = case.ident.to_upper_camel_case();

                        format!("{docs}\n{ident}{case_ident}")
                    })
                    .collect::<Vec<_>>()
                    .join(" | ");

                format!("{interfaces}\n{docs}\nexport type {ident} = {cases}\n")
            }
            TypeDefKind::Enum(cases) => {
                let cases: String = cases
                    .iter()
                    .map(|case| {
                        let docs = print_docs(&case.docs);
                        let ident = case.ident.to_upper_camel_case();

                        format!("{docs}\n{ident},\n")
                    })
                    .collect();

                format!("{docs}\nexport enum {ident} {{ {cases} }}\n")
            }
            TypeDefKind::Union(cases) => {
                let cases: String = cases
                    .iter()
                    .map(|case| {
                        let docs = print_docs(&case.docs);
                        let ty = self.print_type(&case.ty);

                        format!("{docs}\n{ty}\n")
                    })
                    .collect::<Vec<_>>()
                    .join(" | ");

                format!("{docs}\nexport type {ident} = {cases};\n")
            }
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

fn print_docs(docs: &str) -> String {
    if docs.is_empty() {
        return String::new();
    }

    let docs = docs
        .lines()
        .map(|line| format!(" * {line} \n"))
        .collect::<String>();

    format!("/**\n{docs}*/")
}

impl Generate for TypeScript {
    fn to_file(&self) -> (std::path::PathBuf, String) {
        let result_ty = self
            .interface
            .functions
            .iter()
            .any(Function::throws)
            .then_some(
                "export type Result<T, E> = { tag: 'ok', val: T } | { tag: 'err', val: E };\n",
            )
            .unwrap_or_default();

        let typedefs: String = self
            .interface
            .typedefs
            .iter()
            .map(|(id, _)| self.print_typedef(id))
            .collect();

        let functions: String = self
            .interface
            .functions
            .iter()
            .map(|func| self.print_function(func))
            .collect();

        let mut contents = format!("{result_ty}\n{typedefs}\n{functions}");

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
