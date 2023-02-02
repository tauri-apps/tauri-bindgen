use crate::{ast, error::MultiError, util::IteratorExt, Error, TypeId};
use id_arena::Arena;
use miette::{bail, Result, SourceSpan};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    mem,
};

pub fn resolve_interface(
    mut iface: ast::Interface<'_>,
    src: &str,
    skip: impl Fn(&str) -> bool,
) -> Result<crate::Interface> {
    let mut ctx = Resolver::new(src);

    let mut funcs: HashMap<&str, SourceSpan> = HashMap::new();
    let mut names: HashMap<&str, SourceSpan> = HashMap::new();

    log::debug!("Resolving names...");

    for item in &mut iface.items {
        let name = ctx.read_span(item.name);

        if let ast::InterfaceItemKind::Func(_) = item.kind {
            if skip(name) {
                log::debug!("Skipping name resolution for function {}", name);
                continue;
            }
            if let Some(prev) = funcs.insert(name, item.name) {
                bail!(Error::already_defined(item.name, prev));
            }
        } else {
            let id = ctx.types.alloc(crate::TypeDef {
                docs: mem::take(&mut item.docs).resolve(&ctx)?,
                kind: crate::TypeDefKind::Alias(crate::Type::U8),
                name: name.to_string(),
            });

            if ctx.name2id.insert(name, id).is_some() {
                bail!(Error::already_defined(item.name, *names.get(name).unwrap()));
            }
            names.insert(name, item.name);
        }
    }

    log::debug!("Resolving types...");

    for item in &iface.items {
        let name = ctx.read_span(item.name);
        if let ast::InterfaceItemKind::Func(func) = &item.kind {
            if skip(name) {
                log::debug!("Skipping type resolution for function {}", name);
                continue;
            }

            ctx.functions.push(crate::Function {
                docs: item.docs.resolve(&ctx)?,
                name: ctx.read_span(item.name).to_string(),
                params: func.params.resolve(&ctx)?,
                results: func.results.resolve(&ctx)?,
            });
        } else {
            let kind = item.kind.resolve(&ctx)?;
            let id = ctx.name2id.get(name).unwrap();
            ctx.types.get_mut(*id).unwrap().kind = kind;
        }
    }

    let mut visiting = HashSet::new();
    let mut valid_types = HashSet::new();
    for item in &iface.items {
        if let ast::InterfaceItemKind::Func(_) = item.kind {
            continue;
        }
        let name = ctx.read_span(item.name);

        let ty = ctx.name2id.get(name).unwrap();

        verify_not_recursive(&ctx, item.name, *ty, &mut visiting, &mut valid_types)?;
    }

    Ok(crate::Interface {
        docs: iface.docs.resolve(&ctx)?,
        name: ctx.read_span(iface.name).to_string(),
        functions: ctx.functions,
        types: ctx.types,
    })
}

trait Resolve<'a, T> {
    fn resolve(&self, ctx: &'a Resolver<'a>) -> Result<T>;
}

impl<'a> Resolve<'a, crate::Docs> for ast::Docs<'a> {
    fn resolve(&self, _ctx: &'a Resolver<'a>) -> Result<crate::Docs> {
        let contents: String = self
            .docs
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

        Ok(crate::Docs { contents })
    }
}

impl<'a> Resolve<'a, crate::Type> for ast::Type {
    fn resolve(&self, ctx: &'a Resolver<'a>) -> Result<crate::Type> {
        match self {
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
                let types = types
                    .iter()
                    .map(|ty| ty.resolve(ctx))
                    .partition_result()
                    .map_err(MultiError::from)?;

                Ok(crate::Type::Tuple(types))
            }
            ast::Type::List(ty) => {
                let ty = ty.resolve(ctx)?;

                Ok(crate::Type::List(Box::new(ty)))
            }
            ast::Type::Option(ty) => {
                let ty = ty.resolve(ctx)?;

                Ok(crate::Type::Option(Box::new(ty)))
            }
            ast::Type::Result { ok, err } => {
                let ok = ok.as_ref().map(|ok| ok.resolve(ctx)).transpose()?;

                let err = err.as_ref().map(|err| err.resolve(ctx)).transpose()?;

                Ok(crate::Type::Result(Box::new(crate::Result_ { ok, err })))
            }
            ast::Type::Id(span) => {
                let name = ctx.read_span(*span);
                let id = ctx
                    .name2id
                    .get(name)
                    .ok_or_else(|| Error::not_defined(*span))?;

                Ok(crate::Type::Id(*id))
            }
        }
    }
}

impl<'a> Resolve<'a, crate::Record> for ast::Record<'a> {
    fn resolve(&self, ctx: &'a Resolver<'a>) -> Result<crate::Record> {
        let fields = self
            .fields
            .iter()
            .map(|field| field.resolve(ctx))
            .partition_result()
            .map_err(MultiError::from)?;

        Ok(crate::Record { fields })
    }
}

impl<'a> Resolve<'a, crate::RecordField> for ast::RecordField<'a> {
    fn resolve(&self, ctx: &'a Resolver<'a>) -> Result<crate::RecordField> {
        Ok(crate::RecordField {
            docs: self.docs.resolve(ctx)?,
            name: ctx.read_span(self.name).to_string(),
            ty: self.ty.resolve(ctx)?,
        })
    }
}

impl<'a> Resolve<'a, crate::Flags> for ast::Flags<'a> {
    fn resolve(&self, ctx: &'a Resolver<'a>) -> Result<crate::Flags> {
        let flags = self
            .fields
            .iter()
            .map(|field| field.resolve(ctx))
            .partition_result()
            .map_err(MultiError::from)?;

        Ok(crate::Flags { flags })
    }
}

impl<'a> Resolve<'a, crate::Flag> for ast::FlagsField<'a> {
    fn resolve(&self, ctx: &'a Resolver<'a>) -> Result<crate::Flag> {
        Ok(crate::Flag {
            docs: self.docs.resolve(ctx)?,
            name: ctx.read_span(self.name).to_string(),
        })
    }
}

impl<'a> Resolve<'a, crate::Variant> for ast::Variant<'a> {
    fn resolve(&self, ctx: &'a Resolver<'a>) -> Result<crate::Variant> {
        let cases = self
            .cases
            .iter()
            .map(|case| case.resolve(ctx))
            .partition_result()
            .map_err(MultiError::from)?;

        Ok(crate::Variant { cases })
    }
}

impl<'a> Resolve<'a, crate::VariantCase> for ast::VariantCase<'a> {
    fn resolve(&self, ctx: &'a Resolver<'a>) -> Result<crate::VariantCase> {
        Ok(crate::VariantCase {
            docs: self.docs.resolve(ctx)?,
            name: ctx.read_span(self.name).to_string(),
            ty: self.ty.as_ref().map(|ty| ty.resolve(ctx)).transpose()?,
        })
    }
}

impl<'a> Resolve<'a, crate::Union> for ast::Union<'a> {
    fn resolve(&self, ctx: &'a Resolver<'a>) -> Result<crate::Union> {
        let cases = self
            .cases
            .iter()
            .map(|case| case.resolve(ctx))
            .partition_result()
            .map_err(MultiError::from)?;

        Ok(crate::Union { cases })
    }
}

impl<'a> Resolve<'a, crate::UnionCase> for ast::UnionCase<'a> {
    fn resolve(&self, ctx: &'a Resolver<'a>) -> Result<crate::UnionCase> {
        Ok(crate::UnionCase {
            docs: self.docs.resolve(ctx)?,
            ty: self.ty.resolve(ctx)?,
        })
    }
}

impl<'a> Resolve<'a, crate::Enum> for ast::Enum<'a> {
    fn resolve(&self, ctx: &'a Resolver<'a>) -> Result<crate::Enum> {
        let cases = self
            .cases
            .iter()
            .map(|case| case.resolve(ctx))
            .partition_result()
            .map_err(MultiError::from)?;

        Ok(crate::Enum { cases })
    }
}

impl<'a> Resolve<'a, crate::EnumCase> for ast::EnumCase<'a> {
    fn resolve(&self, ctx: &'a Resolver<'a>) -> Result<crate::EnumCase> {
        Ok(crate::EnumCase {
            docs: self.docs.resolve(ctx)?,
            name: ctx.read_span(self.name).to_string(),
        })
    }
}

impl<'a> Resolve<'a, Vec<(String, crate::Type)>> for ast::NamedTypeList {
    fn resolve(&self, ctx: &'a Resolver<'a>) -> Result<Vec<(String, crate::Type)>> {
        let mut params: Vec<(String, crate::Type)> = Vec::new();
        let mut names: HashMap<&'a str, SourceSpan> = HashMap::new();

        for named_type in &self.inner {
            let name = ctx.read_span(named_type.name);
            let ty = named_type.ty.resolve(ctx)?;

            if names.insert(name, named_type.name).is_some() {
                bail!(Error::already_defined(
                    named_type.name,
                    *names.get(name).unwrap()
                ));
            }
            params.push((name.to_string(), ty));
        }

        Ok(params)
    }
}

impl<'a> Resolve<'a, crate::Results> for ast::Results {
    fn resolve(&self, ctx: &'a Resolver<'a>) -> Result<crate::Results> {
        Ok(match self {
            ast::Results::Anon(ty) => crate::Results::Anon(ty.resolve(ctx)?),
            ast::Results::Named(types) => crate::Results::Named(types.resolve(ctx)?),
        })
    }
}

impl<'a> Resolve<'a, crate::TypeDefKind> for ast::InterfaceItemKind<'a> {
    fn resolve(&self, ctx: &'a Resolver<'a>) -> Result<crate::TypeDefKind> {
        let kind = match &self {
            ast::InterfaceItemKind::Record(record) => {
                crate::TypeDefKind::Record(record.resolve(ctx)?)
            }
            ast::InterfaceItemKind::Flags(flags) => crate::TypeDefKind::Flags(flags.resolve(ctx)?),
            ast::InterfaceItemKind::Variant(variant) => {
                crate::TypeDefKind::Variant(variant.resolve(ctx)?)
            }
            ast::InterfaceItemKind::Union(union_) => {
                crate::TypeDefKind::Union(union_.resolve(ctx)?)
            }
            ast::InterfaceItemKind::Enum(enum_) => crate::TypeDefKind::Enum(enum_.resolve(ctx)?),
            ast::InterfaceItemKind::Alias(alias) => crate::TypeDefKind::Alias(alias.resolve(ctx)?),
            ast::InterfaceItemKind::Func(_) => unreachable!(),
            ast::InterfaceItemKind::Use(_) => todo!(),
        };

        Ok(kind)
    }
}

#[derive(Debug)]
struct Resolver<'a> {
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
}

fn verify_not_recursive(
    ctx: &Resolver<'_>,
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

    match &ctx.types[ty].kind {
        crate::TypeDefKind::Union(union) => {
            for case in &union.cases {
                if let crate::Type::Id(id) = case.ty {
                    verify_not_recursive(ctx, name, id, visiting, valid)?;
                }
            }
        }
        crate::TypeDefKind::Record(record) => {
            for field in &record.fields {
                if let crate::Type::Id(id) = field.ty {
                    verify_not_recursive(ctx, name, id, visiting, valid)?;
                }
            }
        }
        crate::TypeDefKind::Variant(variant) => {
            for case in &variant.cases {
                if let Some(crate::Type::Id(id)) = case.ty {
                    verify_not_recursive(ctx, name, id, visiting, valid)?;
                }
            }
        }
        crate::TypeDefKind::Alias(ty) => match ty {
            crate::Type::Tuple(types) => {
                for ty in types {
                    if let crate::Type::Id(id) = ty {
                        verify_not_recursive(ctx, name, *id, visiting, valid)?;
                    }
                }
            }
            crate::Type::List(ty) | crate::Type::Option(ty) => {
                if let crate::Type::Id(id) = **ty {
                    verify_not_recursive(ctx, name, id, visiting, valid)?;
                }
            }
            crate::Type::Result(result) => {
                if let Some(crate::Type::Id(id)) = result.ok {
                    verify_not_recursive(ctx, name, id, visiting, valid)?;
                }

                if let Some(crate::Type::Id(id)) = result.err {
                    verify_not_recursive(ctx, name, id, visiting, valid)?;
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

#[cfg(test)]
mod test {
    use crate::ast::{lex::Tokens, parse::FromTokens, Interface};
    use miette::{NamedSource, Result};

    #[test]
    fn chars() -> Result<()> {
        let input = include_str!("test.wit");

        let mut tokens = Tokens::from_str(input);
        let iface = Interface::parse(&mut tokens)
            .map_err(|error| error.with_source_code(NamedSource::new("test.wit", input)))?;

        let iface = super::resolve_interface(iface, input, |_| false)
            .map_err(|error| error.with_source_code(NamedSource::new("test.wit", input)))?;

        println!("{iface:?}");

        Ok(())
    }
}
