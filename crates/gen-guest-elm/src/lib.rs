use std::path::PathBuf;

use heck::{ToLowerCamelCase, ToUpperCamelCase};
use tauri_bindgen_core::{Generate, GeneratorBuilder};
use wit_parser::{
    EnumCase, FlagsField, Function, FunctionResult, Interface, RecordField, Type, TypeDefId,
    TypeDefKind, UnionCase, VariantCase,
};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Builder {}

impl GeneratorBuilder for Builder {
    fn build(self, interface: Interface) -> Box<dyn Generate> {
        Box::new(Elm {
            opts: self,
            interface,
        })
    }
}

#[derive(Debug)]
pub struct Elm {
    opts: Builder,
    interface: Interface,
}

impl Elm {
    fn print_function(&self, func: &Function) -> String {
        let docs = print_docs(&func.docs);

        let sig = self.print_function_annotation(func);
        let r#impl = self.print_function_impl(func);

        format!(
            r#"
{docs}
{sig}
{impl}
        "#
        )
    }

    fn print_function_annotation(&self, func: &Function) -> String {
        let ident = func.ident.to_lower_camel_case();

        let params = func
            .params
            .iter()
            .map(|(_, ty)| {
                let ty = self.print_type(ty);

                format!("{ty}")
            })
            .collect::<Vec<_>>()
            .join(" -> ");

        let result = func
            .result
            .as_ref()
            .map(|result| self.print_function_result(result))
            .unwrap_or("()".to_string());

        let sep = if params.is_empty() {
            String::new()
        } else {
            " -> ".to_string()
        };

        format!("{ident} : {params}{sep}{result}")
    }

    fn print_function_result(&self, result: &FunctionResult) -> String {
        match result.len() {
            0 => String::new(),
            1 => self.print_type(result.types().next().unwrap()),
            _ => self.print_type(&Type::Tuple(result.types().cloned().collect())),
        }
    }

    fn print_function_impl(&self, func: &Function) -> String {
        let ident = func.ident.to_lower_camel_case();

        let params = func
            .params
            .iter()
            .map(|(ident, _)| {
                let ident = ident.to_lower_camel_case();

                format!("{ident}")
            })
            .collect::<Vec<_>>()
            .join(" ");

        format!("{ident} {params} = ()")
    }

    fn print_type(&self, ty: &Type) -> String {
        match ty {
            Type::Bool => "Bool".to_string(),
            Type::U8
            | Type::U16
            | Type::U32
            | Type::U64
            | Type::S8
            | Type::S16
            | Type::S32
            | Type::S64 => "Int".to_string(),
            Type::Float32 | Type::Float64 => "Float".to_string(),
            Type::Char | Type::String => "String".to_string(),
            Type::List(ty) => {
                let ty = self.print_type(ty);

                format!("(List {ty})")
            }
            Type::Tuple(types) => {
                if types.len() > 3 {
                    let types: String = types
                        .iter()
                        .enumerate()
                        .map(|(ident, ty)| {
                            let ty = self.print_type(&ty);

                            format!("t{ident}: {ty}")
                        })
                        .collect::<Vec<_>>()
                        .join(" , ");

                    format!("{{ {types} }}")
                } else {
                    let types = types
                        .iter()
                        .map(|ty| self.print_type(ty))
                        .collect::<Vec<_>>()
                        .join(", ");

                    format!("({types})")
                }
            }
            Type::Option(ty) => {
                let ty = self.print_type(ty);

                format!("(Maybe {ty})")
            }
            Type::Result { ok, err } => {
                let ok = ok
                    .as_ref()
                    .map_or("()".to_string(), |ty| self.print_type(ty));
                let err = err
                    .as_ref()
                    .map_or("()".to_string(), |ty| self.print_type(ty));

                format!("(Result {ok} {err})")
            }
            Type::Id(id) => self.interface.typedefs[*id].ident.to_upper_camel_case(),
        }
    }

    fn print_typedef(&self, id: TypeDefId) -> String {
        let typedef = &self.interface.typedefs[id];
        let ident = &typedef.ident.to_upper_camel_case();
        let docs = print_docs(&typedef.docs);

        match &typedef.kind {
            TypeDefKind::Alias(ty) => self.print_alias(&docs, ident, ty),
            TypeDefKind::Record(fields) => self.print_record(&docs, ident, fields),
            TypeDefKind::Flags(fields) => self.print_flags(&docs, ident, fields),
            TypeDefKind::Variant(cases) => self.print_variant(&docs, ident, cases),
            TypeDefKind::Enum(cases) => self.print_enum(&docs, ident, cases),
            TypeDefKind::Union(cases) => self.print_union(&docs, ident, cases),
            TypeDefKind::Resource(functions) => self.print_resource(&docs, ident, functions),
        }
    }

    fn print_alias(&self, docs: &str, ident: &str, ty: &Type) -> String {
        let ty = self.print_type(ty);

        format!("{docs}\ntype alias {ident} = {ty}\n")
    }

    fn print_record(&self, docs: &str, ident: &str, fields: &[RecordField]) -> String {
        let fields: String = fields
            .iter()
            .map(|field| {
                // let docs = print_docs(&field.docs);
                let ident = field.ident.to_lower_camel_case();
                let ty = self.print_type(&field.ty);

                format!("\n{docs}\n  {ident}: {ty}\n")
            })
            .collect::<Vec<_>>()
            .join("  , ");

        format!("{docs}\ntype alias {ident} =\n  {{ {fields}  }}\n")
    }

    fn print_flags(&self, docs: &str, ident: &str, fields: &[FlagsField]) -> String {
        todo!()
    }

    fn print_variant(&self, docs: &str, ident: &str, cases: &[VariantCase]) -> String {
        let cases: String = cases
            .iter()
            .map(|case| {
                let docs = print_docs(&case.docs);
                let case_ident = case.ident.to_upper_camel_case();

                let case_ty = case
                    .ty
                    .as_ref()
                    .map(|ty| self.print_type(ty))
                    .unwrap_or_default();

                format!("{docs}{ident}{case_ident}{case_ty}")
            })
            .collect::<Vec<_>>()
            .join(" | ");

        format!("{docs}\ntype {ident} = {cases}")
    }

    fn print_enum(&self, docs: &str, ident: &str, cases: &[EnumCase]) -> String {
        let cases: String = cases
            .iter()
            .map(|case| {
                let docs = print_docs(&case.docs);
                let case_ident = case.ident.to_upper_camel_case();

                format!("{docs}{ident}{case_ident}")
            })
            .collect::<Vec<_>>()
            .join(" | ");

        format!("{docs}\ntype {ident} = {cases}")
    }

    fn print_union(&self, docs: &str, ident: &str, cases: &[UnionCase]) -> String {
        let cases: String = cases
            .iter()
            .map(|case| {
                let docs = print_docs(&case.docs);
                let ty = self.print_type(&case.ty);

                format!("{docs}{ident}{ty}")
            })
            .collect::<Vec<_>>()
            .join(" | ");

        format!("{docs}\ntype {ident} = {cases}")
    }

    fn print_resource(&self, docs: &str, ident: &str, functions: &[Function]) -> String {
        todo!()
    }
}

fn print_docs(docs: &str) -> String {
    if docs.is_empty() {
        return String::new();
    }

    format!("{{-| {docs}\n-}}")
}

impl Generate for Elm {
    fn to_file(&self) -> (std::path::PathBuf, String) {
        let ident = self.interface.ident.to_upper_camel_case();

        let prelude = format!("module {ident} exposing (..)");

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

        let contents = format!("{prelude}\n{typedefs}\n{functions}");

        let mut filename = PathBuf::from(ident);
        filename.set_extension("elm");

        (filename, contents)
    }
}
