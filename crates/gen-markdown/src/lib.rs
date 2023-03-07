use heck::{ToKebabCase, ToSnakeCase};
use std::path::PathBuf;
use tauri_bindgen_core::Generate;
use wit_parser::{Function, FunctionResult, Type, TypeDef};

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Opts {
    // ...
}

impl Opts {
    pub fn build(&self) -> Markdown {
        Markdown {
            _opts: self.clone(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct Markdown {
    _opts: Opts,
    // hrefs: HashMap<String, String>,
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
                format!("list<{}>", ty)
            }
            Type::Tuple(types) => {
                let types = types
                    .iter()
                    .map(|ty| self.print_ty(ty))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("tuple<{}>", types)
            }
            Type::Option(ty) => {
                let ty = self.print_ty(ty);
                format!("option<{}>", ty)
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

                format!("result<{}, {}>", ok, err)
            }
            Type::Id(typedef) => {
                let ident = &typedef.borrow().ident;
                let lnk = ident.to_snake_case();
                format!("[{}](#{})", ident, lnk)
            }
        }
    }

    fn print_docs(&self, docs: &str) -> String {
        docs.lines()
            .map(|line| line.trim())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn print_typedef(&self, typedef: &TypeDef) -> String {
        let ident = &typedef.ident;
        let docs = self.print_docs(&typedef.docs);

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
            params = self.print_named_types(func.params.iter()),
            result = self.print_result(&func.result),
            docs = func.docs
        )
    }

    fn print_named_types<'a>(&self, types: impl Iterator<Item = &'a (String, Type)>) -> String {
        types
            .map(|(ident, ty)| format!("{ident}: {ty}", ty = self.print_ty(ty)))
            .collect::<Vec<_>>()
            .join(", ")
    }

    fn print_result(&self, result: &FunctionResult) -> String {
        if result.len() == 0 {
            return "".to_string();
        }

        if let Some(Type::Tuple(types)) = result.types().next() {
            if types.len() == 0 {
                return "".to_string();
            }
        }

        match result {
            FunctionResult::Anon(ty) => format!(" -> {ty}", ty = self.print_ty(ty)),
            FunctionResult::Named(types) => format!(
                " -> ({types})",
                types = self.print_named_types(types.iter())
            ),
        }
    }
}

impl Generate for Markdown {
    fn to_string(&self, iface: &wit_parser::Interface) -> (std::path::PathBuf, String) {
        let ident = &iface.ident;
        let docs = self.print_docs(&iface.docs);
        let typedefs = iface
            .typedefs
            .iter()
            .map(|typedef| self.print_typedef(&*typedef.borrow()))
            .collect::<Vec<_>>()
            .join("\n");
        let functions = iface
            .functions
            .iter()
            .map(|func| self.print_function(func))
            .collect::<Vec<_>>()
            .join("\n");

        let contents = format!(
            "# {ident}\n\n{docs}\n\n## Type definitions\n\n{typedefs}\n\n## Functions\n\n{functions}",
        );

        let mut filename = PathBuf::from(iface.ident.to_kebab_case());
        filename.set_extension("md");

        (filename, contents)
    }
}

// impl WorldGenerator for Markdown {

//     fn finish(&mut self, name: &str, files: &mut Files, _world_hash: &str) {
//         let parser = Parser::new(&self.src);
//         let mut events = Vec::new();
//         for event in parser {
//             if let Event::Code(code) = &event {
//                 if let Some(dst) = self.hrefs.get(code.as_ref()) {
//                     let tag = Tag::Link(LinkType::Inline, dst.as_str().into(), "".into());
//                     events.push(Event::Start(tag.clone()));
//                     events.push(event.clone());
//                     events.push(Event::End(tag));
//                     continue;
//                 }
//             }
//             events.push(event);
//         }
//         let mut html_output = String::new();
//         html::push_html(&mut html_output, events.into_iter());

//         files.push(&format!("{name}.md"), self.src.as_bytes());
//         files.push(&format!("{name}.html"), html_output.as_bytes());
//     }
// }

// impl InterfaceGenerator<'_> {
//     fn funcs(&mut self) {
//         if self.iface.functions.is_empty() {
//             return;
//         }
//         self.push_str("## Functions\n\n");
//         for func in &self.iface.functions {
//             self.push_str("----\n\n");
//             self.push_str(&format!(
//                 "#### <a href=\"#{0}\" name=\"{0}\"></a> `",
//                 func.name.to_snake_case()
//             ));
//             self.gen
//                 .hrefs
//                 .insert(func.name.clone(), format!("#{}", func.name.to_snake_case()));
//             self.push_str(&func.name);
//             self.push_str("` ");
//             self.push_str("\n\n");
//             self.docs(&func.docs);

//             if !func.params.is_empty() {
//                 self.push_str("##### Params\n\n");
//                 for (name, ty) in &func.params {
//                     self.push_str(&format!(
//                         "- <a href=\"#{f}.{p}\" name=\"{f}.{p}\"></a> `{}`: ",
//                         name,
//                         f = func.name.to_snake_case(),
//                         p = name.to_snake_case(),
//                     ));
//                     self.print_ty(ty, false);
//                     self.push_str("\n");
//                 }
//             }

//             if !func.results.is_empty() {
//                 self.push_str("##### Results\n\n");
//                 for (i, ty) in func.results.types().enumerate() {
//                     self.push_str(&format!(
//                         "- <a href=\"#{f}.{p}{i}\" name=\"{f}.{p}{i}\"></a> `{}{i}`: ",
//                         "result",
//                         f = func.name.to_snake_case(),
//                         p = "result",
//                     ));
//                     self.print_ty(ty, false);
//                     self.push_str("\n");
//                 }
//             }

//             self.push_str("\n");
//         }
//     }

//     fn push_str(&mut self, s: &str) {
//         self.gen.src.push_str(s);
//     }

//     fn print_type_header(&mut self, name: &str) {
//         if !self.types_header_printed {
//             self.push_str("## Types\n\n");
//             self.types_header_printed = true;
//         }
//         self.push_str(&format!(
//             "## <a href=\"#{}\" name=\"{0}\"></a> `{}`: ",
//             name.to_snake_case(),
//             name,
//         ));
//         self.gen
//             .hrefs
//             .insert(name.to_string(), format!("#{}", name.to_snake_case()));
//     }

//     fn print_type_info(&mut self, _ty: TypeId, docs: &Docs) {
//         self.docs(docs);
//         self.push_str("\n");
//         // self.push_str(&format!("Size: {}, ", self.sizes.size(&Type::Id(ty))));
//         // self.push_str(&format!("Alignment: {}\n", self.sizes.align(&Type::Id(ty))));
//     }
// }

// impl<'a> tauri_bindgen_core::InterfaceGenerator<'a> for InterfaceGenerator<'a> {
//     fn iface(&self) -> &'a Interface {
//         self.iface
//     }

//     fn type_record(&mut self, id: TypeId, name: &str, record: &Record, docs: &Docs) {
//         self.print_type_header(name);
//         self.push_str("record\n\n");
//         self.print_type_info(id, docs);
//         self.push_str("\n### Record Fields\n\n");
//         for field in &record.fields {
//             self.push_str(&format!(
//                 "- <a href=\"{r}.{f}\" name=\"{r}.{f}\"></a> [`{name}`](#{r}.{f}): ",
//                 r = name.to_snake_case(),
//                 f = field.name.to_snake_case(),
//                 name = field.name,
//             ));
//             self.gen.hrefs.insert(
//                 format!("{name}::{}", field.name),
//                 format!("#{}.{}", name.to_snake_case(), field.name.to_snake_case()),
//             );
//             self.print_ty(&field.ty, false);
//             self.gen.src.indent(1);
//             self.push_str("\n\n");
//             self.docs(&field.docs);
//             self.gen.src.deindent(1);
//             self.push_str("\n");
//         }
//     }

//     // fn type_tuple(&mut self, id: TypeId, name: &str, tuple: &Tuple, docs: &Docs) {
//     //     self.print_type_header(name);
//     //     self.push_str("tuple\n\n");
//     //     self.print_type_info(id, docs);
//     //     self.push_str("\n### Tuple Fields\n\n");
//     //     for (i, ty) in tuple.types.iter().enumerate() {
//     //         self.push_str(&format!(
//     //             "- <a href=\"{r}.{f}\" name=\"{r}.{f}\"></a> [`{name}`](#{r}.{f}): ",
//     //             r = name.to_snake_case(),
//     //             f = i,
//     //             name = i,
//     //         ));
//     //         self.gen.hrefs.insert(
//     //             format!("{}::{}", name, i),
//     //             format!("#{}.{}", name.to_snake_case(), i),
//     //         );
//     //         self.print_ty(ty, false);
//     //         self.push_str("\n");
//     //     }
//     // }

//     fn type_flags(&mut self, id: TypeId, name: &str, flags: &Flags, docs: &Docs) {
//         self.print_type_header(name);
//         self.push_str("record\n\n");
//         self.print_type_info(id, docs);
//         self.push_str("\n### Record Fields\n\n");
//         for (i, flag) in flags.flags.iter().enumerate() {
//             self.push_str(&format!(
//                 "- <a href=\"{r}.{f}\" name=\"{r}.{f}\"></a> [`{name}`](#{r}.{f}): ",
//                 r = name.to_snake_case(),
//                 f = flag.name.to_snake_case(),
//                 name = flag.name,
//             ));
//             self.gen.hrefs.insert(
//                 format!("{name}::{}", flag.name),
//                 format!("#{}.{}", name.to_snake_case(), flag.name.to_snake_case()),
//             );
//             self.gen.src.indent(1);
//             self.push_str("\n\n");
//             self.docs(&flag.docs);
//             self.gen.src.deindent(1);
//             self.push_str(&format!("Bit: {i}\n"));
//             self.push_str("\n");
//         }
//     }

//     fn type_variant(&mut self, id: TypeId, name: &str, variant: &Variant, docs: &Docs) {
//         self.print_type_header(name);
//         self.push_str("variant\n\n");
//         self.print_type_info(id, docs);
//         self.push_str("\n### Variant Cases\n\n");
//         for case in &variant.cases {
//             self.push_str(&format!(
//                 "- <a href=\"{v}.{c}\" name=\"{v}.{c}\"></a> [`{name}`](#{v}.{c})",
//                 v = name.to_snake_case(),
//                 c = case.name.to_snake_case(),
//                 name = case.name,
//             ));
//             self.gen.hrefs.insert(
//                 format!("{name}::{}", case.name),
//                 format!("#{}.{}", name.to_snake_case(), case.name.to_snake_case()),
//             );
//             if let Some(ty) = &case.ty {
//                 self.push_str(": ");
//                 self.print_ty(ty, false);
//             }
//             self.gen.src.indent(1);
//             self.push_str("\n\n");
//             self.docs(&case.docs);
//             self.gen.src.deindent(1);
//             self.push_str("\n");
//         }
//     }

//     fn type_union(&mut self, id: TypeId, name: &str, union: &Union, docs: &Docs) {
//         self.print_type_header(name);
//         self.push_str("union\n\n");
//         self.print_type_info(id, docs);
//         self.push_str("\n### Union Cases\n\n");
//         let snake = name.to_snake_case();
//         for (i, case) in union.cases.iter().enumerate() {
//             self.push_str(&format!(
//                 "- <a href=\"{snake}.{i}\" name=\"{snake}.{i}\"></a> [`{i}`](#{snake}.{i})",
//             ));
//             self.gen
//                 .hrefs
//                 .insert(format!("{name}::{i}"), format!("#{snake}.{i}"));
//             self.push_str(": ");
//             self.print_ty(&case.ty, false);
//             self.gen.src.indent(1);
//             self.push_str("\n\n");
//             self.docs(&case.docs);
//             self.gen.src.deindent(1);
//             self.push_str("\n");
//         }
//     }

//     fn type_enum(&mut self, id: TypeId, name: &str, enum_: &Enum, docs: &Docs) {
//         self.print_type_header(name);
//         self.push_str("enum\n\n");
//         self.print_type_info(id, docs);
//         self.push_str("\n### Enum Cases\n\n");
//         for case in &enum_.cases {
//             self.push_str(&format!(
//                 "- <a href=\"{v}.{c}\" name=\"{v}.{c}\"></a> [`{name}`](#{v}.{c})",
//                 v = name.to_snake_case(),
//                 c = case.name.to_snake_case(),
//                 name = case.name,
//             ));
//             self.gen.hrefs.insert(
//                 format!("{name}::{}", case.name),
//                 format!("#{}.{}", name.to_snake_case(), case.name.to_snake_case()),
//             );
//             self.gen.src.indent(1);
//             self.push_str("\n\n");
//             self.docs(&case.docs);
//             self.gen.src.deindent(1);
//             self.push_str("\n");
//         }
//     }

//     // fn type_option(&mut self, id: TypeId, name: &str, payload: &Type, docs: &Docs) {
//     //     self.print_type_header(name);
//     //     self.push_str("option<");
//     //     self.print_ty(payload, false);
//     //     self.push_str(">");
//     //     self.print_type_info(id, docs);
//     // }

//     // fn type_result(&mut self, id: TypeId, name: &str, result: &Result_, docs: &Docs) {
//     //     self.print_type_header(name);
//     //     match (result.ok, result.err) {
//     //         (Some(ok), Some(err)) => {
//     //             self.push_str("result<");
//     //             self.print_ty(&ok, false);
//     //             self.push_str(", ");
//     //             self.print_ty(&err, false);
//     //             self.push_str(">");
//     //         }
//     //         (None, Some(err)) => {
//     //             self.push_str("result<_, ");
//     //             self.print_ty(&err, false);
//     //             self.push_str(">");
//     //         }
//     //         (Some(ok), None) => {
//     //             self.push_str("result<");
//     //             self.print_ty(&ok, false);
//     //             self.push_str(">");
//     //         }
//     //         (None, None) => {
//     //             self.push_str("result");
//     //         }
//     //     }
//     //     self.print_type_info(id, docs);
//     // }

//     fn type_alias(&mut self, id: TypeId, name: &str, ty: &Type, docs: &Docs) {
//         self.print_type_header(name);
//         self.print_ty(ty, true);
//         self.push_str("\n\n");
//         self.print_type_info(id, docs);
//         self.push_str("\n");
//     }

//     // fn type_list(&mut self, id: TypeId, name: &str, _ty: &Type, docs: &Docs) {
//     //     self.type_alias(id, name, &Type::Id(id), docs);
//     // }

//     // fn type_builtin(&mut self, id: TypeId, name: &str, ty: &Type, docs: &Docs) {
//     //     self.type_alias(id, name, ty, docs)
//     // }
// }
