#![allow(clippy::must_use_candidate, clippy::unused_self)]

use heck::{ToKebabCase, ToLowerCamelCase, ToSnakeCase, ToUpperCamelCase};
use std::path::PathBuf;
use tauri_bindgen_core::{postprocess, Generate, GeneratorBuilder, TypeInfo, TypeInfos};
use tauri_bindgen_gen_js::{JavaScriptGenerator, SerdeUtils};
use wit_parser::{Function, FunctionResult, Interface, Type, TypeDefKind};

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
    /// Run `rome format` to format the generated code. This formatter is much faster than `prettier`. Requires a global installation of `rome`.
    #[cfg_attr(feature = "clap", clap(long))]
    pub romefmt: bool,
}

impl GeneratorBuilder for Builder {
    fn build(self, interface: Interface) -> Box<dyn Generate> {
        let methods = interface
            .typedefs
            .iter()
            .filter_map(|(_, typedef)| {
                if let TypeDefKind::Resource(methods) = &typedef.kind {
                    Some(methods.iter())
                } else {
                    None
                }
            })
            .flatten();

        let infos = TypeInfos::collect_from_functions(
            &interface.typedefs,
            interface.functions.iter().chain(methods),
        );

        let serde_utils =
            SerdeUtils::collect_from_functions(&interface.typedefs, &interface.functions);

        Box::new(JavaScript {
            opts: self,
            interface,
            infos,
            serde_utils,
        })
    }
}

#[derive(Debug)]
pub struct JavaScript {
    opts: Builder,
    interface: Interface,
    infos: TypeInfos,
    serde_utils: SerdeUtils,
}

impl JavaScript {
    fn print_function(&self, intf_name: &str, func: &Function) -> String {
        let docs = self.print_docs(func);
        let ident = func.id.to_lower_camel_case();
        let name = func.id.to_snake_case();
        let params = print_function_params(&func.params);

        let deserialize_result = func
            .result
            .as_ref()
            .map(|res| self.print_deserialize_function_result(res))
            .unwrap_or_default();

        let serialize_params = func
            .params
            .iter()
            .map(|(ident, ty)| self.print_serialize_ty(&ident.to_lower_camel_case(), ty))
            .collect::<Vec<_>>()
            .join(";\n");

        format!(
            r#"
{docs}
export async function {ident} ({params}) {{
    const out = []
    {serialize_params}

    return fetch('ipc://localhost/{intf_name}/{name}', {{ method: "POST", body: Uint8Array.from(out), headers: {{ 'Content-Type': 'application/octet-stream' }} }}){deserialize_result}
}}
"#
        )
    }

    fn print_resource(
        &self,
        mod_ident: &str,
        docs: &str,
        ident: &str,
        functions: &[Function],
        info: TypeInfo,
    ) -> String {
        let ident = ident.to_upper_camel_case();

        let functions: String = functions
            .iter()
            .map(|func| {
                let docs = self.print_docs(func);
                let mod_ident = mod_ident.to_snake_case();
                let resource_ident = ident.to_snake_case();
                let ident = func.id.to_lower_camel_case();

                let params = print_function_params(&func.params);

                let deserialize_result = func
                    .result
                    .as_ref()
                    .map(|res| self.print_deserialize_function_result(res))
                    .unwrap_or_default();

                let serialize_params = func
                    .params
                    .iter()
                    .map(|(ident, ty)| self.print_serialize_ty(&ident.to_lower_camel_case(), ty))
                    .collect::<Vec<_>>()
                    .join(";\n");

                format!(
                    r#"{docs}
async {ident} ({params}) {{
    const out = []
    serializeU32(out, this.#id);
    {serialize_params}

    await fetch('ipc://localhost/{mod_ident}::resource::{resource_ident}/{ident}', {{ method: "POST", body: Uint8Array.from(out), headers: {{ 'Content-Type': 'application/octet-stream' }} }}){deserialize_result}
}}
"#
                )
            })
            .collect();

        let deserialize = if info.contains(TypeInfo::RESULT) {
            format!(
                "static deserialize(de) {{
    const self = new {ident}();
    self.#id = deserializeU32(de);
    return self
}}"
            )
        } else {
            String::new()
        };

        format!(
            "{docs}\nexport class {ident} {{
            #id;
            {functions}
            {deserialize}
        }}"
        )
    }

    fn print_docs(&self, func: &Function) -> String {
        let docs = func
            .docs
            .lines()
            .map(|line| format!(" * {line} \n"))
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

        let result_docs = func
            .result
            .as_ref()
            .map(|result| match result {
                FunctionResult::Anon(ty) => {
                    let ty = self.print_ty(ty);
                    format!("* @returns {{Promise<{ty}>}} \n")
                }
                FunctionResult::Named(types) => {
                    let types = types
                        .iter()
                        .map(|(_, ty)| self.print_ty(ty))
                        .collect::<Vec<_>>()
                        .join(", ");

                    format!("* @returns {{Promise<[{types}]>}} \n")
                }
            })
            .unwrap_or_default();

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
            Type::U64 | Type::S64 | Type::U128 | Type::S128 => "bigint".to_string(),
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
                let ok = ok.as_ref().map_or("_".to_string(), |ty| self.print_ty(ty));
                let err = err.as_ref().map_or("_".to_string(), |ty| self.print_ty(ty));

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
            Type::U128
            | Type::S128
            | Type::Bool
            | Type::Tuple(_)
            | Type::List(_)
            | Type::Option(_)
            | Type::Result { .. }
            | Type::Char
            | Type::String => None,
        }
    }
}

impl JavaScriptGenerator for JavaScript {
    fn interface(&self) -> &Interface {
        &self.interface
    }

    fn infos(&self) -> &TypeInfos {
        &self.infos
    }
}

impl Generate for JavaScript {
    fn to_file(&mut self) -> (std::path::PathBuf, String) {
        let deserializers: String = self
            .interface
            .typedefs
            .iter()
            .filter_map(|(id, _)| {
                let info = self.infos[id];

                if info.contains(TypeInfo::RESULT) {
                    Some(self.print_deserialize_typedef(id))
                } else {
                    None
                }
            })
            .collect();

        let serializers: String = self
            .interface
            .typedefs
            .iter()
            .filter_map(|(id, _)| {
                let info = self.infos[id];

                if info.contains(TypeInfo::PARAM) {
                    Some(self.print_serialize_typedef(id))
                } else {
                    None
                }
            })
            .collect();

        let functions: String = self
            .interface
            .functions
            .iter()
            .map(|func| self.print_function(&self.interface.ident.to_snake_case(), func))
            .collect();

        let resources: String = self
            .interface
            .typedefs
            .iter()
            .filter_map(|(id, typedef)| {
                let info = self.infos[id];
                if let TypeDefKind::Resource(functions) = &typedef.kind {
                    Some(self.print_resource(
                        &self.interface.ident,
                        &typedef.docs,
                        &typedef.ident,
                        functions,
                        info,
                    ))
                } else {
                    None
                }
            })
            .collect();

        let serde_utils = self.serde_utils.to_string();

        let mut contents =
            format!("{serde_utils}{deserializers}{serializers}\n{functions}\n{resources}");

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

        let mut filename = PathBuf::from(self.interface.ident.to_kebab_case());
        filename.set_extension("js");

        (filename, contents)
    }
}

fn print_function_params(params: &[(String, Type)]) -> String {
    params
        .iter()
        .map(|(name, _)| name.to_lower_camel_case())
        .collect::<Vec<_>>()
        .join(", ")
}
