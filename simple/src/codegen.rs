use heck::{ToShoutySnekCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
use std::{collections::HashMap, ops::Deref};
use syn::{parse_quote, Lifetime};

use crate::typecheck::{
    EnumCase, FlagsField, Function, FunctionResult, Int, Interface, RecordField, Type, TypeDef,
    TypeDefKind, TypeInfo, UnionCase, VariantCase,
};

struct GuestGenerator {
    inner: RustGenerator,
}

impl GuestGenerator {
    pub fn new() -> Self {
        Self {
            inner: RustGenerator::new(BorrowMode::AllBorrowed(syn::parse_quote!('a))),
        }
    }

    pub fn print_interface(&self, iface: &Interface) -> TokenStream {
        let docs = self.inner.print_docs(&iface.docs);

        let ident = format_ident!("{}", iface.ident.to_snake_case());

        let typedefs = iface.typedefs.iter().flat_map(|typedef| {
            let typedef = typedef.borrow();

            eprintln!("printing typedef {:?}", typedef.ident);

            let variants = variants_of(
                &typedef.ident.to_upper_camel_case(),
                typedef.info,
                &BorrowMode::AllBorrowed(syn::parse_quote!('a)),
            );

            let variants = variants.iter().map(|TypeVariant { ident, borrow_mode }| {
                let docs = &typedef.docs;

                match &typedef.kind {
                    TypeDefKind::Alias(ty) => {
                        self.inner
                            .print_alias(docs, &ident, &ty, typedef.info, borrow_mode)
                    }
                    TypeDefKind::Record(fields) => {
                        self.inner
                            .print_record(docs, &ident, &fields, typedef.info, borrow_mode)
                    }
                    TypeDefKind::Flags(fields) => self.inner.print_flags(docs, &ident, &fields),
                    TypeDefKind::Variant(cases) => {
                        self.inner
                            .print_variant(docs, &ident, &cases, typedef.info, borrow_mode)
                    }
                    TypeDefKind::Enum(cases) => self.inner.print_enum(docs, &ident, &cases),
                    TypeDefKind::Union(cases) => {
                        self.inner
                            .print_union(docs, &ident, &cases, typedef.info, borrow_mode)
                    }
                }
            });

            quote! { #(#variants)* }
        });

        let functions = iface.functions.iter().map(|func| self.print_function(func));

        quote! {
            #docs
            pub mod #ident {
                #(#typedefs)*

                #(#functions)*
            }
        }
    }

    pub fn print_function(&self, func: &Function) -> TokenStream {
        let sig = self.inner.print_function_signature(func);

        quote! {
            #sig {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        parse::{self, FromTokens},
        typecheck::Resolver,
        Result,
    };
    use logos::Lexer;

    #[test]
    fn codegen_full() -> Result<()> {
        let source = include_str!("test.wit");

        let mut tokens = Lexer::new(source).spanned().peekable();

        let iface = parse::Interface::parse(&mut tokens)?;
        let iface = Resolver::new(source, &iface).resolve()?;

        // println!("{}", iface.into_token_stream().to_string());
        let gen = GuestGenerator::new();
        let syntax_tree = syn::parse2(gen.print_interface(&iface)).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);
        println!("{}", formatted);

        Ok(())
    }
}
