#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

use crate::{
    lex,
    util::{find_similar, print_list, IteratorExt},
    EnumCase, Error, FlagsField, Function, FunctionResult, Interface, RecordField, Result, Type,
    TypeDef, TypeDefKind, UnionCase, VariantCase,
};
use id_arena::{Arena, Id};
use logos::Span;
use std::collections::{HashMap, HashSet};

use crate::parse;

pub struct RestInterface {
    ident: Span,
    docs: Vec<Span>,
    functions: Vec<parse::InterfaceItem>,
}

pub struct Resolver<'a> {
    source: &'a str,
    iface_typedefs: HashMap<&'a str, parse::InterfaceItem>,

    ident2id: HashMap<&'a str, Id<TypeDef>>,
    typedefs: Arena<TypeDef>,
}

impl<'a> Resolver<'a> {
    #[must_use]
    pub fn new(source: &'a str, interface: parse::Interface) -> (Self, RestInterface) {
        let (iface_funcs, iface_typedefs): (Vec<_>, Vec<_>) = interface
            .items
            .into_iter()
            .partition(|item| matches!(item.inner, parse::InterfaceItemInner::Func(_)));

        let iface_typedefs: HashMap<_, _> = iface_typedefs
            .into_iter()
            .map(|item| (&source[item.ident.clone()], item))
            .collect();

        let this = Self {
            ident2id: HashMap::with_capacity(iface_typedefs.len()),
            typedefs: Arena::with_capacity(iface_typedefs.len()),

            source,
            iface_typedefs,
        };

        let rest = RestInterface {
            ident: interface.ident,
            docs: interface.docs,
            functions: iface_funcs,
        };

        (this, rest)
    }

    fn read_span(&self, span: &Span) -> &'a str {
        &self.source[span.clone()]
    }

    fn resolve_ident(&self, span: &Span) -> &'a str {
        self.read_span(span).trim_start_matches('%')
    }

    fn resolve_docs(&self, docs: &[Span]) -> String {
        docs.iter()
            .map(|span| {
                let str = self.read_span(span);
                let str = str.strip_prefix("///").unwrap_or(str);
                let str = str.strip_prefix("/**").unwrap_or(str);
                let str = str.trim();

                str
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn resolve_typedef(&mut self, typedef: &parse::InterfaceItem) -> Result<Id<TypeDef>> {
        let ident = self.resolve_ident(&typedef.ident);

        if let Some(id) = self.ident2id.get(ident) {
            return Ok(*id);
        }

        let docs = self.resolve_docs(&typedef.docs);

        let kind = match &typedef.inner {
            parse::InterfaceItemInner::Alias(ty) => {
                let ty = self.resolve_type(ty)?;

                TypeDefKind::Alias(ty)
            }
            parse::InterfaceItemInner::Record(fields) => {
                let inner = fields
                    .iter()
                    .map(|field| {
                        let docs = self.resolve_docs(&field.docs);
                        let ident = self.resolve_ident(&field.ident).to_string();
                        let ty = self.resolve_type(&field.ty)?;

                        Ok(RecordField {
                            docs,
                            id: ident,
                            ty,
                        })
                    })
                    .transponse_result::<_, Vec<_>, _, _>()?;

                TypeDefKind::Record(inner)
            }
            parse::InterfaceItemInner::Flags(fields) => {
                let fields = fields.iter().map(|field| {
                    let docs = self.resolve_docs(&field.docs);
                    let ident = self.resolve_ident(&field.ident).to_string();

                    FlagsField { docs, id: ident }
                });

                TypeDefKind::Flags(fields.collect())
            }
            parse::InterfaceItemInner::Variant(cases) => {
                let inner = cases
                    .iter()
                    .map(|case| {
                        let docs = self.resolve_docs(&case.docs);
                        let ident = self.resolve_ident(&case.ident).to_string();
                        let ty = case
                            .ty
                            .as_ref()
                            .map(|ty| self.resolve_type(ty))
                            .transpose()?;

                        Ok(VariantCase {
                            docs,
                            id: ident,
                            ty,
                        })
                    })
                    .transponse_result::<_, Vec<_>, _, _>()?;

                TypeDefKind::Variant(inner)
            }
            parse::InterfaceItemInner::Enum(cases) => {
                let cases = cases.iter().map(|case| {
                    let docs = self.resolve_docs(&case.docs);
                    let ident = self.resolve_ident(&case.ident).to_string();

                    EnumCase { docs, id: ident }
                });

                TypeDefKind::Enum(cases.collect())
            }
            parse::InterfaceItemInner::Union(cases) => {
                let inner = cases
                    .iter()
                    .map(|case| {
                        let docs = self.resolve_docs(&case.docs);
                        let ty = self.resolve_type(&case.ty)?;

                        Ok(UnionCase { docs, ty })
                    })
                    .transponse_result::<_, Vec<_>, _, _>()?;

                TypeDefKind::Union(inner)
            }
            parse::InterfaceItemInner::Resource(methods) => {
                let functions = methods
                    .iter()
                    .map(|method| self.resolve_func(&method.docs, &method.ident, &method.inner))
                    .transponse_result::<_, Vec<_>, _, _>()?;

                TypeDefKind::Resource(functions)
            }
            parse::InterfaceItemInner::Func(_) => unreachable!(),
        };

        let id = self.typedefs.alloc(TypeDef {
            docs,
            ident: ident.to_string(),
            kind,
        });
        self.ident2id.insert(ident, id);
        self.iface_typedefs.remove(ident);

        Ok(id)
    }

    fn resolve_type(&mut self, ty: &parse::Type) -> Result<Type> {
        let ty = match ty {
            parse::Type::Bool => Type::Bool,
            parse::Type::U8 => Type::U8,
            parse::Type::U16 => Type::U16,
            parse::Type::U32 => Type::U32,
            parse::Type::U64 => Type::U64,
            parse::Type::U128 => Type::U128,
            parse::Type::S8 => Type::S8,
            parse::Type::S16 => Type::S16,
            parse::Type::S32 => Type::S32,
            parse::Type::S64 => Type::S64,
            parse::Type::S128 => Type::S128,
            parse::Type::Float32 => Type::Float32,
            parse::Type::Float64 => Type::Float64,
            parse::Type::Char => Type::Char,
            parse::Type::String => Type::String,
            parse::Type::List(ty) => Type::List(Box::new(self.resolve_type(ty)?)),
            parse::Type::Option(ty) => Type::Option(Box::new(self.resolve_type(ty)?)),
            parse::Type::Tuple(types) => {
                let types = types
                    .iter()
                    .map(|ty| self.resolve_type(ty))
                    .transponse_result::<_, Vec<_>, _, _>()?;

                Type::Tuple(types)
            }
            parse::Type::Result { ok, err } => {
                let ok = ok.as_ref().map(|ty| self.resolve_type(ty)).transpose()?;

                let err = err.as_ref().map(|ty| self.resolve_type(ty)).transpose()?;

                Type::Result {
                    ok: ok.map(Box::new),
                    err: err.map(Box::new),
                }
            }
            parse::Type::Id(span) => {
                let ident = self.resolve_ident(span);

                if let Some(id) = self.ident2id.get(ident) {
                    Type::Id(*id)
                } else {
                    let typedef = self.iface_typedefs.get(ident).ok_or_else(|| {
                        let expected = lex::Token::TYPE_KEYWORD
                            .iter()
                            .map(lex::Token::as_str)
                            .chain(self.iface_typedefs.keys().map(|str| &**str));

                        let suggestions = find_similar(expected, ident);

                        if suggestions.is_empty() {
                            Error::not_defined(span.clone())
                        } else {
                            Error::not_defined_with_help(
                                span.clone(),
                                format!("Did you mean \"{}\"?", print_list(suggestions)),
                            )
                        }
                    })?;

                    let id = self.resolve_typedef(&typedef.clone())?; // TODO: avoid clone

                    Type::Id(id)
                }
            }
        };

        Ok(ty)
    }

    fn resolve_named_types(
        &mut self,
        named_types: &[(Span, parse::Type)],
    ) -> Result<Vec<(String, Type)>> {
        named_types
            .iter()
            .map(|(ident, ty)| {
                let ident = self.resolve_ident(ident).to_string();
                let ty = self.resolve_type(ty)?;

                Ok((ident, ty))
            })
            .transponse_result::<_, Vec<_>, _, _>()
            .map_err(Into::into)
    }

    fn resolve_func(
        &mut self,
        docs: &[Span],
        ident: &Span,
        func: &parse::Func,
    ) -> Result<Function> {
        let docs = self.resolve_docs(docs);
        let ident = self.resolve_ident(ident).to_string();

        let params = self.resolve_named_types(&func.params)?;

        let result = match &func.result {
            None => None,
            Some(parse::FuncResult::Anon(ty)) => {
                let ty = self.resolve_type(ty)?;
                Some(FunctionResult::Anon(ty))
            }
            Some(parse::FuncResult::Named(types)) => {
                let types = self.resolve_named_types(types)?;
                Some(FunctionResult::Named(types))
            }
        };

        Ok(Function {
            docs,
            id: ident,
            params,
            result,
        })
    }

    fn verify_not_recursive(
        &self,
        ident: Span,
        id: Id<TypeDef>,
        visiting: &mut HashSet<Id<TypeDef>>,
        valid: &mut HashSet<Id<TypeDef>>,
    ) -> Result<()> {
        if valid.contains(&id) {
            return Ok(());
        }

        if !visiting.insert(id) {
            return Err(Error::recursive_type(ident));
        }

        match &self.typedefs[id].kind {
            TypeDefKind::Record(fields) => {
                for field in fields {
                    if let Type::Id(id) = field.ty {
                        self.verify_not_recursive(ident.clone(), id, visiting, valid)?;
                    }
                }
            }
            TypeDefKind::Union(cases) => {
                for case in cases {
                    if let Type::Id(id) = case.ty {
                        self.verify_not_recursive(ident.clone(), id, visiting, valid)?;
                    }
                }
            }
            TypeDefKind::Variant(cases) => {
                for case in cases {
                    if let Some(Type::Id(id)) = case.ty {
                        self.verify_not_recursive(ident.clone(), id, visiting, valid)?;
                    }
                }
            }
            TypeDefKind::Alias(ty) => match ty {
                Type::Tuple(types) => {
                    for ty in types {
                        if let Type::Id(id) = ty {
                            self.verify_not_recursive(ident.clone(), *id, visiting, valid)?;
                        }
                    }
                }
                Type::List(ty) | Type::Option(ty) => {
                    if let Type::Id(id) = **ty {
                        self.verify_not_recursive(ident, id, visiting, valid)?;
                    }
                }
                Type::Result { ok, err } => {
                    if let Some(Type::Id(id)) = ok.as_deref() {
                        self.verify_not_recursive(ident.clone(), *id, visiting, valid)?;
                    }

                    if let Some(Type::Id(id)) = err.as_deref() {
                        self.verify_not_recursive(ident, *id, visiting, valid)?;
                    }
                }
                _ => {}
            },
            _ => {}
        }

        valid.insert(id);
        visiting.remove(&id);

        Ok(())
    }

    pub fn resolve(mut self, rest_data: RestInterface) -> Result<Interface> {
        let docs = self.resolve_docs(&rest_data.docs);
        let ident = self.resolve_ident(&rest_data.ident).to_string();

        let ident2span = self
            .iface_typedefs
            .iter()
            .map(|(ident, item)| (*ident, item.ident.clone()))
            .collect::<HashMap<_, _>>();

        let mut functions = Vec::new();
        for item in rest_data.functions {
            if let parse::InterfaceItemInner::Func(func) = &item.inner {
                let func = self.resolve_func(&item.docs, &item.ident, func)?;
                functions.push(func);
            }
        }

        let mut visiting = HashSet::new();
        let mut valid_types = HashSet::new();
        for (id, typedef) in &self.typedefs {
            let ident = ident2span[typedef.ident.as_str()].clone();

            self.verify_not_recursive(ident, id, &mut visiting, &mut valid_types)?;
        }

        if !self.iface_typedefs.is_empty() {
            // we use `partition_result` here to aggregate all errors before throwing them,
            // this way all errors are reported together instead of one by one.
            self.iface_typedefs
                .values()
                .map(|item| Err(Error::unused_type(item.ident.clone())))
                .transponse_result::<_, Vec<_>, _, _>()?;
        }

        Ok(Interface {
            docs,
            ident,
            functions,
            typedefs: self.typedefs,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use logos::Lexer;
    use parse::FromTokens;

    #[test]
    fn interface_() -> Result<()> {
        let source = "interface chars {
            /// A function that accepts a character
            func take_char(x: char)
            /// A function that returns a character
            func return_char() -> char
          }";
        let mut tokens = Lexer::new(source).spanned().peekable();

        let iface = parse::Interface::parse(&mut tokens)?;
        let (resolver, rest_data) = Resolver::new(source, iface);
        let iface = resolver.resolve(rest_data)?;

        println!("{iface:#?}");

        Ok(())
    }

    #[test]
    fn interface__() -> Result<()> {
        let source = "interface test {
            record a {
                foo: nested
            }
            record nested {
                bar: string
            }
            func fn(x: a) -> nested
          }";
        let mut tokens = Lexer::new(source).spanned().peekable();

        let iface = parse::Interface::parse(&mut tokens)?;
        let (resolver, rest_data) = Resolver::new(source, iface);
        let iface = resolver.resolve(rest_data)?;

        println!("{iface:#?}");

        Ok(())
    }

    #[test]
    fn full() -> Result<()> {
        let source = include_str!("test.wit");

        let mut tokens = Lexer::new(source).spanned().peekable();

        let iface = parse::Interface::parse(&mut tokens)?;
        let (resolver, rest_data) = Resolver::new(source, iface);
        resolver.resolve(rest_data)?;

        Ok(())
    }
}
