use heck::{ToKebabCase, ToSnakeCase};
use std::path::PathBuf;
use tauri_bindgen_core::{Generate, GeneratorBuilder};
use wit_parser::{Function, FunctionResult, Interface, Type, TypeDefId};

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Builder {
    // ...
}

impl GeneratorBuilder for Builder {
    fn build(self, interface: Interface) -> Box<dyn Generate> {
        Box::new(Markdown {
            _opts: self,
            interface,
        })
    }
}

pub struct Markdown {
    _opts: Builder,
    interface: Interface,
}

impl Markdown {
    fn print_ty(&self, ty: &Type) -> String {
        match ty {
            Type::Bool => "bool".to_string(),
            Type::U8 => "u8".to_string(),
            Type::U16 => "u16".to_string(),
            Type::U32 => "u32".to_string(),
            Type::U64 => "u64".to_string(),
            Type::S8 => "s8".to_string(),
            Type::S16 => "s16".to_string(),
            Type::S32 => "s32".to_string(),
            Type::S64 => "s64".to_string(),
            Type::Float32 => "float32".to_string(),
            Type::Float64 => "float64".to_string(),
            Type::Char => "char".to_string(),
            Type::String => "string".to_string(),
            Type::List(ty) => {
                let ty = self.print_ty(ty);
                format!("list<{ty}>")
            }
            Type::Tuple(types) => {
                let types = types
                    .iter()
                    .map(|ty| self.print_ty(ty))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("tuple<{types}>")
            }
            Type::Option(ty) => {
                let ty = self.print_ty(ty);
                format!("option<{ty}>")
            }
            Type::Result { ok, err } => {
                let ok = ok.as_ref().map_or("_".to_string(), |ty| self.print_ty(ty));
                let err = err.as_ref().map_or("_".to_string(), |ty| self.print_ty(ty));

                format!("result<{ok}, {err}>")
            }
            Type::Id(id) => {
                let ident = &self.interface.typedefs[*id].ident;
                let lnk = ident.to_snake_case();

                format!("[{ident}](#{lnk})")
            }
        }
    }

    fn print_typedef(&self, id: TypeDefId) -> String {
        let typedef = &self.interface.typedefs[id];
        let ident = &typedef.ident;
        let docs = print_docs(&typedef.docs);

        match &typedef.kind {
            wit_parser::TypeDefKind::Alias(ty) => {
                let ty = self.print_ty(ty);
                format!("## Alias {ident}\n\n`{ty}`\n\n{docs}")
            }
            wit_parser::TypeDefKind::Record(fields) => {
                let fields = fields
                    .iter()
                    .map(|field| {
                        format!(
                            "#### {ident}: `{ty}`\n{docs}\n",
                            ident = field.ident,
                            ty = self.print_ty(&field.ty),
                            docs = field.docs
                        )
                    })
                    .collect::<String>();

                format!("## Struct {ident}\n\n{docs}\n\n### Fields\n\n{fields}")
            }
            wit_parser::TypeDefKind::Flags(fields) => {
                let fields = fields
                    .iter()
                    .map(|field| {
                        format!(
                            "#### {ident}\n{docs}\n",
                            ident = field.ident,
                            docs = field.docs
                        )
                    })
                    .collect::<String>();

                format!("## Flags {ident}\n\n{docs}\n\n### Fields\n\n{fields}")
            }
            wit_parser::TypeDefKind::Variant(cases) => {
                let cases = cases
                    .iter()
                    .map(|case| {
                        format!(
                            "#### {ident}{ty}\n{docs}\n",
                            ident = case.ident,
                            ty = case
                                .ty
                                .as_ref()
                                .map(|ty| format!(": `{}`", self.print_ty(ty)))
                                .unwrap_or_default(),
                            docs = case.docs
                        )
                    })
                    .collect::<String>();

                format!("## Variant {ident}\n\n{docs}\n\n### Cases\n\n{cases}")
            }
            wit_parser::TypeDefKind::Enum(cases) => {
                let cases = cases
                    .iter()
                    .map(|case| {
                        format!(
                            "#### {ident}\n{docs}\n",
                            ident = case.ident,
                            docs = case.docs
                        )
                    })
                    .collect::<String>();

                format!("## Enum {ident}\n\n{docs}\n\n### Cases\n\n{cases}")
            }
            wit_parser::TypeDefKind::Union(cases) => {
                let cases = cases
                    .iter()
                    .map(|case| {
                        format!(
                            "#### `{ty}`\n{docs}\n",
                            ty = self.print_ty(&case.ty),
                            docs = case.docs
                        )
                    })
                    .collect::<String>();

                format!("## Union {ident}\n\n{docs}\n\n### Cases\n\n{cases}")
            }
        }
    }

    fn print_function(&self, func: &Function) -> String {
        format!(
            "### Function {ident}\n\n`func {ident} ({params}){result}`\n\n{docs}",
            ident = func.ident,
            params = self.print_named_types(&func.params),
            result = self.print_result(&func.result),
            docs = func.docs
        )
    }

    fn print_named_types(&self, types: &[(String, Type)]) -> String {
        types
            .iter()
            .map(|(ident, ty)| format!("{ident}: {ty}", ty = self.print_ty(ty)))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn print_result(&self, result: &FunctionResult) -> String {
        if result.is_empty() {
            return String::new();
        }

        if let Some(Type::Tuple(types)) = result.types().next() {
            if types.is_empty() {
                return String::new();
            }
        }

        match result {
            FunctionResult::Anon(ty) => format!(" -> {ty}", ty = self.print_ty(ty)),
            FunctionResult::Named(types) => {
                format!(" -> ({types})", types = self.print_named_types(types))
            }
        }
    }
}

fn print_docs(docs: &str) -> String {
    docs.lines().map(str::trim).collect::<Vec<_>>().join("\n")
}

impl Generate for Markdown {
    fn to_file(&self) -> (std::path::PathBuf, String) {
        let ident = &self.interface.ident;
        let docs = print_docs(&self.interface.docs);
        let typedefs = self
            .interface
            .typedefs
            .iter()
            .map(|(id, _)| self.print_typedef(id))
            .collect::<Vec<_>>()
            .join("\n");
        let functions = self
            .interface
            .functions
            .iter()
            .map(|func| self.print_function(func))
            .collect::<Vec<_>>()
            .join("\n");

        let contents = format!(
            "# {ident}\n\n{docs}\n\n## Type definitions\n\n{typedefs}\n\n## Functions\n\n{functions}",
        );

        let mut filename = PathBuf::from(self.interface.ident.to_kebab_case());
        filename.set_extension("md");

        (filename, contents)
    }
}
