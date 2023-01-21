use super::InterfaceItemKind;
use crate::{ast, error::MultiError, Error, TypeId};
use id_arena::Arena;
use miette::{bail, Result, SourceSpan};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    mem,
};

#[derive(Debug)]
pub struct Resolver<'a> {
    src: &'a str,
    name2id: HashMap<&'a str, TypeId>,
    types: Arena<crate::TypeDef>,
    functions: Vec<crate::Function>,
}

impl<'a> Resolver<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            name2id: HashMap::default(),
            types: Arena::default(),
            functions: Vec::default(),
        }
    }

    fn read_span(&self, span: SourceSpan) -> &'a str {
        &self.src[span.offset()..span.offset() + span.len()]
    }

    pub fn resolve(mut self, mut iface: ast::Interface<'a>) -> Result<crate::Interface> {
        let mut funcs = HashMap::new();
        let mut names: HashMap<&'a str, SourceSpan> = HashMap::new();

        for item in &mut iface.items {
            let name = self.read_span(item.name);

            if let InterfaceItemKind::Func(_) = item.kind {
                if let Some(prev) = funcs.insert(name, item.name) {
                    bail!(Error::already_defined(item.name, prev));
                }
            } else {
                let id = self.types.alloc(crate::TypeDef {
                    // pos: item.pos.offset()..item.pos.offset() + item.pos.len(),
                    docs: Self::resolve_docs(&mem::take(&mut item.docs)),
                    kind: crate::TypeDefKind::Type(crate::Type::U8),
                    name: name.to_string(),
                });

                if self.name2id.insert(name, id).is_some() {
                    bail!(Error::already_defined(item.name, *names.get(name).unwrap()));
                }
                names.insert(name, item.name);
            }
        }

        for item in &iface.items {
            if let InterfaceItemKind::Func(func) = &item.kind {
                self.resolve_func(func, item.name, &item.docs)?;
            } else {
                self.resolve_typedef(item)?;
            }
        }

        let mut visiting = HashSet::new();
        let mut valid_types = HashSet::new();
        for item in &iface.items {
            if let InterfaceItemKind::Func(_) = item.kind {
                continue;
            }
            let name = self.read_span(item.name);

            let ty = self.name2id.get(name).unwrap();

            self.verify_not_recursive(item.name, *ty, &mut visiting, &mut valid_types)?;
        }

        Ok(crate::Interface {
            docs: Self::resolve_docs(&iface.docs),
            name: self.read_span(iface.name).to_string(),
            functions: self.functions,
            types: self.types,
        })
    }

    fn resolve_typedef(&mut self, typedef: &ast::InterfaceItem<'a>) -> Result<()> {
        use ast::InterfaceItemKind::{Alias, Enum, Flags, Func, Record, Union, Use, Variant};

        let kind = match &typedef.kind {
            Record(ast::Record { fields, .. }) => {
                let results = fields.iter().map(|case| -> Result<crate::RecordField> {
                    Ok(crate::RecordField {
                        docs: Self::resolve_docs(&case.docs),
                        name: self.read_span(case.name).to_string(),
                        ty: self.resolve_type(&case.ty)?,
                    })
                });

                let fields = lift_errors(results)?.collect();

                crate::TypeDefKind::Record(crate::Record { fields })
            }
            Variant(ast::Variant { cases, .. }) => {
                let results = cases.iter().map(|case| -> Result<crate::VariantCase> {
                    let mut ty = None;
                    if let Some(ty_) = &case.ty {
                        ty = Some(self.resolve_type(ty_)?);
                    }

                    Ok(crate::VariantCase {
                        docs: Self::resolve_docs(&case.docs),
                        name: self.read_span(case.name).to_string(),
                        ty,
                    })
                });

                let cases = lift_errors(results)?.collect();

                crate::TypeDefKind::Variant(crate::Variant { cases })
            }
            Flags(ast::Flags { fields, .. }) => {
                let flags = fields
                    .iter()
                    .map(|case| crate::Flag {
                        docs: Self::resolve_docs(&case.docs),
                        name: self.read_span(case.name).to_string(),
                    })
                    .collect();

                crate::TypeDefKind::Flags(crate::Flags { flags })
            }
            Union(ast::Union { cases, .. }) => {
                let results = cases.iter().map(|case| -> Result<crate::UnionCase> {
                    Ok(crate::UnionCase {
                        docs: Self::resolve_docs(&case.docs),
                        ty: self.resolve_type(&case.ty)?,
                    })
                });

                let cases = lift_errors(results)?.collect();

                crate::TypeDefKind::Union(crate::Union { cases })
            }
            Enum(ast::Enum { cases, .. }) => {
                let cases = cases
                    .iter()
                    .map(|case| crate::EnumCase {
                        docs: Self::resolve_docs(&case.docs),
                        name: self.read_span(case.name).to_string(),
                    })
                    .collect();

                crate::TypeDefKind::Enum(crate::Enum { cases })
            }
            Alias(ty) => {
                let ty = self.resolve_type(ty)?;

                crate::TypeDefKind::Type(ty)
            }
            Func(_) => todo!(),
            Use(_) => todo!(),
        };

        let name = self.read_span(typedef.name);
        let id = self.name2id.get(name).unwrap();
        self.types.get_mut(*id).unwrap().kind = kind;

        Ok(())
    }

    fn resolve_func(
        &mut self,
        func: &ast::Func,
        name: SourceSpan,
        docs: &ast::Docs<'a>,
    ) -> Result<()> {
        let params = self.resolve_params(&func.params)?;

        let results = self.resolve_results(&func.results)?;

        self.functions.push(crate::Function {
            docs: Self::resolve_docs(docs),
            name: self.read_span(name).to_string(),
            params,
            results,
        });

        Ok(())
    }

    fn resolve_params(
        &mut self,
        params: &ast::NamedTypeList,
    ) -> Result<Vec<(String, crate::Type)>> {
        let mut out_params: Vec<(String, crate::Type)> = Vec::new();
        let mut names: HashMap<&'a str, SourceSpan> = HashMap::new();

        for named_type in &params.inner {
            let name = self.read_span(named_type.name);
            let ty = self.resolve_type(&named_type.ty)?;

            if names.insert(name, named_type.name).is_some() {
                bail!(Error::already_defined(
                    named_type.name,
                    *names.get(name).unwrap()
                ));
            }
            out_params.push((name.to_string(), ty));
        }

        Ok(out_params)
    }

    fn resolve_results(&mut self, results: &ast::Results) -> Result<crate::Results> {
        Ok(match results {
            ast::Results::Anon(ty) => crate::Results::Anon(self.resolve_type(ty)?),
            ast::Results::Named(types) => crate::Results::Named(self.resolve_params(types)?),
        })
    }

    fn resolve_type(&mut self, ty: &ast::Type) -> Result<crate::Type> {
        match ty {
            ast::Type::U8 => Ok(crate::Type::U8),
            ast::Type::U16 => Ok(crate::Type::U16),
            ast::Type::U32 => Ok(crate::Type::U32),
            ast::Type::U64 => Ok(crate::Type::U64),
            ast::Type::S8 => Ok(crate::Type::S8),
            ast::Type::S16 => Ok(crate::Type::S16),
            ast::Type::S32 => Ok(crate::Type::S32),
            ast::Type::S64 => Ok(crate::Type::S64),
            ast::Type::Float32 => Ok(crate::Type::Float32),
            ast::Type::Float64 => Ok(crate::Type::Float64),
            ast::Type::Char => Ok(crate::Type::Char),
            ast::Type::String => Ok(crate::Type::String),
            ast::Type::Bool => Ok(crate::Type::Bool),
            ast::Type::Tuple(types) => {
                let results = types.iter().map(|ty| self.resolve_type(ty));

                let types = lift_errors(results)?.collect();

                Ok(crate::Type::Tuple(crate::Tuple { types }))
            }
            ast::Type::List(ty) => {
                let ty = self.resolve_type(ty)?;

                Ok(crate::Type::List(Box::new(ty)))
            }
            ast::Type::Option(ty) => {
                let ty = self.resolve_type(ty)?;

                Ok(crate::Type::Option(Box::new(ty)))
            }
            ast::Type::Result { ok, err } => {
                let ok = ok.as_ref().map(|ok| self.resolve_type(ok.as_ref()));
                let ok = lift_errors(ok.into_iter())?.next();

                let err = err.as_ref().map(|err| self.resolve_type(err.as_ref()));
                let err = lift_errors(err.into_iter())?.next();

                Ok(crate::Type::Result(Box::new(crate::Result_ { ok, err })))
            }
            ast::Type::Id(span) => {
                let name = self.read_span(*span);
                let id = self
                    .name2id
                    .get(name)
                    .ok_or_else(|| Error::not_defined(*span))?;

                Ok(crate::Type::Id(*id))
            }
        }
    }

    fn resolve_docs(docs: &ast::Docs<'a>) -> crate::Docs {
        let docs = &docs.docs;

        let contents: String = docs
            .iter()
            .map(|line| {
                let mut str = *line;

                if let Some(str_) = line.strip_prefix("///") {
                    str = str_;
                }

                if let Some(str_) = line.strip_prefix("/**") {
                    str = str_;
                }

                if let Some(str_) = line.strip_suffix("*/") {
                    str = str_;
                }

                str
            })
            .collect();

        crate::Docs { contents }
    }

    fn verify_not_recursive(
        &self,
        name: SourceSpan,
        ty: TypeId,
        visiting: &mut HashSet<TypeId>,
        valid: &mut HashSet<TypeId>,
    ) -> Result<()> {
        if valid.contains(&ty) {
            return Ok(());
        }

        if !visiting.insert(ty) {
            bail!(Error::recursive_type(name));
        }

        match &self.types[ty].kind {
            crate::TypeDefKind::Union(union) => {
                for case in &union.cases {
                    if let crate::Type::Id(id) = case.ty {
                        self.verify_not_recursive(name, id, visiting, valid)?;
                    }
                }
            }
            crate::TypeDefKind::Record(record) => {
                for field in &record.fields {
                    if let crate::Type::Id(id) = field.ty {
                        self.verify_not_recursive(name, id, visiting, valid)?;
                    }
                }
            }
            crate::TypeDefKind::Variant(variant) => {
                for case in &variant.cases {
                    if let Some(crate::Type::Id(id)) = case.ty {
                        self.verify_not_recursive(name, id, visiting, valid)?;
                    }
                }
            }
            crate::TypeDefKind::Type(ty) => match ty {
                crate::Type::Tuple(tuple) => {
                    for ty in &tuple.types {
                        if let crate::Type::Id(id) = ty {
                            self.verify_not_recursive(name, *id, visiting, valid)?;
                        }
                    }
                }
                crate::Type::List(ty) | crate::Type::Option(ty) => {
                    if let crate::Type::Id(id) = **ty {
                        self.verify_not_recursive(name, id, visiting, valid)?;
                    }
                }
                crate::Type::Result(result) => {
                    if let Some(crate::Type::Id(id)) = result.ok {
                        self.verify_not_recursive(name, id, visiting, valid)?;
                    }

                    if let Some(crate::Type::Id(id)) = result.err {
                        self.verify_not_recursive(name, id, visiting, valid)?;
                    }
                }
                _ => {}
            },
            _ => {}
        }

        valid.insert(ty);
        visiting.remove(&ty);

        Ok(())
    }
}

fn lift_errors<T, E>(
    iter: impl Iterator<Item = Result<T, E>>,
) -> Result<impl Iterator<Item = T>, MultiError>
where
    T: Debug,
    E: Into<MultiError> + Debug,
{
    let (types, errors): (Vec<_>, Vec<_>) = iter.partition(Result::is_ok);

    let errors: Vec<_> = errors
        .into_iter()
        .flat_map(|res| Result::unwrap_err(res).into())
        .collect();

    if !errors.is_empty() {
        return Err(MultiError::from_iter(errors));
    }

    Ok(types.into_iter().map(Result::unwrap))
}

#[cfg(test)]
mod test {
    use crate::ast::{lex::Tokens, parse::FromTokens, resolve::Resolver, Interface};
    use miette::{NamedSource, Result};

    #[test]
    fn chars() -> Result<()> {
        let input = include_str!("test.wit");

        let mut tokens = Tokens::from_str(input);
        let iface = Interface::parse(&mut tokens)
            .map_err(|error| error.with_source_code(NamedSource::new("test.wit", input)))?;

        let iface = Resolver::new(input)
            .resolve(iface)
            .map_err(|error| error.with_source_code(NamedSource::new("test.wit", input)))?;

        println!("{:?}", iface);

        Ok(())
    }
}
