use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    mem,
};

use id_arena::Arena;

use crate::{ast, TypeId};

use super::{lex::Span, InterfaceItem};

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    /// A name wasn't defined.
    #[error("name `{0}` not defined.")]
    NotDefined(String),
    /// Names can't be defined more than once.
    #[error("name `{0}` already defined.")]
    AlreadyDefined(String),
    /// Types can't recursively refer to themselves as that would make then possibly infinitly-sized.
    /// In Rust the compiler would force you to use heap indirection, however such a thing doesn't exist in out type system.
    ///
    /// This wouldn't be a problem with the current JSON format, but a custom binary one would have this limitation so for future proofing we deny recursive types.
    #[error("Type cannot refer to itself.")]
    RecursiveType(String),
    /// Most types such as Options, Results, or Unions can't be empty as limitations in the default JSON serialization
    /// doesn't preserve these nuances. This might be removed in the future when the default serialization changes to a custom binary one.
    #[error("Type cannot be empty.")]
    EmptyType,
}

#[derive(Debug, PartialEq)]
pub struct Errors(Vec<Error>);

impl From<Error> for Errors {
    fn from(err: Error) -> Self {
        Self(vec![err])
    }
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for err in self.0.iter() {
            f.write_str(&err.to_string())?;
        }
        Ok(())
    }
}

impl std::error::Error for Errors {}

impl IntoIterator for Errors {
    type Item = Error;

    type IntoIter = std::vec::IntoIter<Error>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Error> for Errors {
    fn from_iter<T: IntoIterator<Item = Error>>(iter: T) -> Self {
        Self(Vec::from_iter(iter))
    }
}

#[derive(Debug, Default)]
pub struct Resolver<'a> {
    name2id: HashMap<&'a str, TypeId>,
    types: Arena<crate::TypeDef>,
    functions: Vec<crate::Function>,
}

impl<'a> Resolver<'a> {
    pub fn resolve(mut self, mut iface: ast::Interface<'a>) -> Result<crate::Interface, Errors> {
        let mut funcs = HashSet::new();
        for item in iface.items.iter_mut() {
            match item {
                InterfaceItem::Record(ast::Record { docs, name, .. })
                | InterfaceItem::Variant(ast::Variant { docs, name, .. })
                | InterfaceItem::Flags(ast::Flags { docs, name, .. })
                | InterfaceItem::Union(ast::Union { docs, name, .. })
                | InterfaceItem::Enum(ast::Enum { docs, name, .. })
                | InterfaceItem::Alias(ast::Alias { docs, name, .. }) => {
                    let id = self.types.alloc(crate::TypeDef {
                        docs: Self::resolve_docs(&mem::take(docs)),
                        kind: crate::TypeDefKind::Type(crate::Type::U8),
                        name: name.to_string(),
                    });

                    if self.name2id.insert(name.as_str(), id).is_some() {
                        return Err(Error::AlreadyDefined(name.to_string()).into());
                    }
                }
                InterfaceItem::Func(ast::Func { name, .. }) => {
                    if !funcs.insert(*name) {
                        return Err(Error::AlreadyDefined(name.to_string()).into());
                    }
                }
            }
        }

        let (typedefs, funcs): (Vec<_>, Vec<_>) = iface.items.iter().partition(|item| match item {
            ast::InterfaceItem::Func(_) => false,
            _ => true,
        });

        for typedef in typedefs {
            self.resolve_typedef(typedef)?;
        }

        for func in funcs {
            match func {
                InterfaceItem::Func(func) => self.resolve_func(func)?,
                _ => panic!(),
            }
        }

        let mut valid_types = HashSet::new();
        let mut visiting = HashSet::new();
        for item in iface.items.iter() {
            let (span, ty) = match item {
                InterfaceItem::Record(ast::Record { name, .. })
                | InterfaceItem::Variant(ast::Variant { name, .. })
                | InterfaceItem::Flags(ast::Flags { name, .. })
                | InterfaceItem::Union(ast::Union { name, .. })
                | InterfaceItem::Enum(ast::Enum { name, .. })
                | InterfaceItem::Alias(ast::Alias { name, .. }) => {
                    (name, self.name2id.get(name.as_str()).unwrap())
                }
                InterfaceItem::Func(_) => continue,
            };

            self.verify_not_recursive(span, *ty, &mut visiting, &mut valid_types)?;
        }

        Ok(crate::Interface {
            docs: Self::resolve_docs(&iface.docs),
            name: iface.name.to_string(),
            functions: self.functions,
            types: self.types,
        })
    }

    fn resolve_typedef(&mut self, typedef: &ast::InterfaceItem<'a>) -> Result<(), Errors> {
        use ast::InterfaceItem::*;

        let (name, kind) = match typedef {
            Func(_) => panic!(),
            Record(ast::Record { name, fields, .. }) => {
                let results = fields
                    .iter()
                    .map(|case| -> Result<crate::RecordField, Errors> {
                        Ok(crate::RecordField {
                            docs: Self::resolve_docs(&case.docs),
                            name: case.name.to_string(),
                            ty: self.resolve_type(&case.ty)?,
                        })
                    });

                let fields = lift_errors(results)?.collect();

                let kind = crate::TypeDefKind::Record(crate::Record { fields });

                (name, kind)
            }
            Variant(ast::Variant { name, cases, .. }) => {
                let results = cases
                    .iter()
                    .map(|case| -> Result<crate::VariantCase, Errors> {
                        let mut ty = None;
                        if let Some(ty_) = &case.ty {
                            ty = Some(self.resolve_type(&ty_)?);
                        }

                        Ok(crate::VariantCase {
                            docs: Self::resolve_docs(&case.docs),
                            name: case.name.to_string(),
                            ty,
                        })
                    });

                let cases = lift_errors(results)?.collect();

                let kind = crate::TypeDefKind::Variant(crate::Variant { cases });

                (name, kind)
            }
            Flags(ast::Flags {
                name,
                flags: fields,
                ..
            }) => {
                let flags = fields
                    .into_iter()
                    .map(|case| crate::Flag {
                        docs: Self::resolve_docs(&case.docs),
                        name: case.name.to_string(),
                    })
                    .collect();

                let kind = crate::TypeDefKind::Flags(crate::Flags { flags });

                (name, kind)
            }
            Union(ast::Union { name, cases, .. }) => {
                let results = cases
                    .iter()
                    .map(|case| -> Result<crate::UnionCase, Errors> {
                        Ok(crate::UnionCase {
                            docs: Self::resolve_docs(&case.docs),
                            ty: self.resolve_type(&case.ty)?,
                        })
                    });

                let cases = lift_errors(results)?.collect();

                let kind = crate::TypeDefKind::Union(crate::Union { cases });

                (name, kind)
            }
            Enum(ast::Enum { name, cases, .. }) => {
                let cases = cases
                    .into_iter()
                    .map(|case| crate::EnumCase {
                        docs: Self::resolve_docs(&case.docs),
                        name: case.name.to_string(),
                    })
                    .collect();

                let kind = crate::TypeDefKind::Enum(crate::Enum { cases });

                (name, kind)
            }
            Alias(ast::Alias { name, ty, .. }) => {
                let ty = self.resolve_type(ty)?;

                let kind = crate::TypeDefKind::Type(ty);

                (name, kind)
            }
        };

        let id = self.name2id.get(name.as_str()).unwrap();
        self.types.get_mut(*id).unwrap().kind = kind;

        Ok(())
    }

    fn resolve_func(&mut self, func: &ast::Func<'a>) -> Result<(), Errors> {
        let params = self.resolve_params(&func.params)?;

        let results = self.resolve_results(&func.results)?;

        self.functions.push(crate::Function {
            docs: Self::resolve_docs(&func.docs),
            name: func.name.to_string(),
            params,
            results,
        });

        Ok(())
    }

    fn resolve_params(
        &mut self,
        params: &ast::NamedTypeList<'a>,
    ) -> Result<HashMap<String, crate::Type>, Errors> {
        let mut out_params: HashMap<String, crate::Type> = HashMap::new();

        for (name, ty) in params {
            let ty = self.resolve_type(ty)?;

            if out_params.insert(name.to_string(), ty).is_some() {
                return Err(Error::AlreadyDefined(name.to_string()).into());
            }
        }

        Ok(out_params)
    }

    fn resolve_results(&mut self, results: &ast::Results<'a>) -> Result<crate::Results, Errors> {
        Ok(match results {
            ast::Results::Anon(ty) => crate::Results::Anon(self.resolve_type(ty)?),
            ast::Results::Named(types) => crate::Results::Named(self.resolve_params(types)?),
        })
    }

    fn resolve_type(&mut self, ty: &ast::Type<'a>) -> Result<crate::Type, Errors> {
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
                // if types.types.is_empty() {
                //     return Err(Error::EmptyType.into());
                // }

                let results = types.types.iter().map(|ty| self.resolve_type(ty));

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
            ast::Type::Result(result) => {
                let mut ok = None;
                if let Some(ok_) = &result.ok {
                    ok = Some(self.resolve_type(ok_)?);
                }
                let mut err = None;
                if let Some(err_) = &result.err {
                    err = Some(self.resolve_type(err_)?);
                }

                Ok(crate::Type::Result(Box::new(crate::Result_ { ok, err })))
            }
            ast::Type::Id(name) => {
                let id = self
                    .name2id
                    .get(name.as_str())
                    .ok_or(Error::NotDefined(name.to_string()))?;

                Ok(crate::Type::Id(*id))
            }
        }
    }

    fn resolve_docs(docs: &ast::Docs<'a>) -> crate::Docs {
        let docs = &docs.docs;

        let contents: String = docs
            .into_iter()
            .map(|line| {
                let mut str = line.as_str();

                if let Some(str_) = line.as_str().strip_prefix("///") {
                    str = str_;
                }

                if let Some(str_) = line.as_str().strip_prefix("/**") {
                    str = str_;
                }

                if let Some(str_) = line.as_str().strip_suffix("*/") {
                    str = str_;
                }

                str
            })
            .collect();

        crate::Docs { contents }
    }

    fn verify_not_recursive(
        &self,
        span: &Span<'a>,
        ty: TypeId,
        visiting: &mut HashSet<TypeId>,
        valid: &mut HashSet<TypeId>,
    ) -> Result<(), Errors> {
        if valid.contains(&ty) {
            return Ok(());
        }

        if !visiting.insert(ty) {
            return Err(Error::RecursiveType(span.to_string()).into());
        }

        match &self.types[ty].kind {
            crate::TypeDefKind::Union(union) => {
                for case in union.cases.iter() {
                    if let crate::Type::Id(id) = case.ty {
                        self.verify_not_recursive(span, id, visiting, valid)?;
                    }
                }
            }
            crate::TypeDefKind::Record(record) => {
                for field in record.fields.iter() {
                    if let crate::Type::Id(id) = field.ty {
                        self.verify_not_recursive(span, id, visiting, valid)?;
                    }
                }
            }
            crate::TypeDefKind::Variant(variant) => {
                for case in variant.cases.iter() {
                    if let Some(crate::Type::Id(id)) = case.ty {
                        self.verify_not_recursive(span, id, visiting, valid)?;
                    }
                }
            }
            crate::TypeDefKind::Type(ty) => match ty {
                crate::Type::Tuple(tuple) => {
                    for ty in tuple.types.iter() {
                        if let crate::Type::Id(id) = ty {
                            self.verify_not_recursive(span, *id, visiting, valid)?;
                        }
                    }
                }
                crate::Type::List(ty) => {
                    if let crate::Type::Id(id) = **ty {
                        self.verify_not_recursive(span, id, visiting, valid)?;
                    }
                }
                crate::Type::Option(ty) => {
                    if let crate::Type::Id(id) = **ty {
                        self.verify_not_recursive(span, id, visiting, valid)?;
                    }
                }
                crate::Type::Result(result) => {
                    if let Some(crate::Type::Id(id)) = result.ok {
                        self.verify_not_recursive(span, id, visiting, valid)?;
                    }

                    if let Some(crate::Type::Id(id)) = result.err {
                        self.verify_not_recursive(span, id, visiting, valid)?;
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
) -> Result<impl Iterator<Item = T>, Errors>
where
    T: Debug,
    E: Into<Errors> + Debug,
{
    let (types, errors): (Vec<_>, Vec<_>) = iter.partition(Result::is_ok);

    let errors: Errors = errors
        .into_iter()
        .map(|res| Result::unwrap_err(res).into())
        .flatten()
        .collect();

    if !errors.0.is_empty() {
        return Err(errors);
    }

    Ok(types.into_iter().map(Result::unwrap))
}

#[cfg(test)]
mod test {
    use crate::ast::{lex::Tokenizer, parse::interface, resolve::Resolver};

    #[test]
    fn chars() {
        let input = include_str!("test.wit");

        let mut tokens = Tokenizer::from_str(input);
        let iface = interface(&mut tokens).unwrap();

        let iface = Resolver::default().resolve(iface).unwrap();

        println!("{:?}", iface);
    }
}
