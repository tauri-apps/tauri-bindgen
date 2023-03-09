#![allow(clippy::module_name_repetitions)]
mod readable;
mod writable;

use quote::quote;
use readable::{readable_enum, readable_flags, readable_struct};
use syn::{parse_macro_input, DeriveInput};
use writable::{
    size_hint_enum, size_hint_flags, size_hint_struct, writable_enum, writable_flags,
    writable_struct,
};

#[proc_macro_derive(Readable, attributes(abi))]
pub fn derive_readable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident,
        data,
        attrs,
        generics,
        ..
    } = parse_macro_input!(input);

    let uses_flags_abi = attrs
        .iter()
        .find(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "abi")
        .map(|a| a.tokens.to_string() == "(flags)")
        .unwrap_or_default();

    let inner = match data {
        syn::Data::Struct(_) if uses_flags_abi => readable_flags(),
        syn::Data::Struct(data) => readable_struct(&data),
        syn::Data::Enum(data) => readable_enum(&data),
        syn::Data::Union(_) => unimplemented!(),
    };

    let output = quote! {
        impl #generics tauri_bindgen_abi::Readable for #ident #generics {
            fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, tauri_bindgen_abi::Error> {
                #inner
            }
        }
    };

    output.into()
}

#[proc_macro_derive(Writable, attributes(abi))]
pub fn derive_writable(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident,
        data,
        attrs,
        generics,
        ..
    } = parse_macro_input!(input);

    let uses_flags_abi = attrs
        .iter()
        .find(|a| a.path.segments.len() == 1 && a.path.segments[0].ident == "abi")
        .map(|a| a.tokens.to_string() == "(flags)")
        .unwrap_or_default();

    let write_to_inner = match &data {
        syn::Data::Struct(_) if uses_flags_abi => writable_flags(),
        syn::Data::Struct(data) => writable_struct(data),
        syn::Data::Enum(data) => writable_enum(data),
        syn::Data::Union(_) => unimplemented!(),
    };

    let size_hint_inner = match &data {
        syn::Data::Struct(_) if uses_flags_abi => size_hint_flags(),
        syn::Data::Struct(data) => size_hint_struct(data),
        syn::Data::Enum(data) => size_hint_enum(data),
        syn::Data::Union(_) => unimplemented!(),
    };

    let output = quote! {
        impl #generics tauri_bindgen_abi::Writable for #ident #generics {
            fn write_to(&self, write: &mut impl ::std::io::Write) -> Result<(), tauri_bindgen_abi::Error> {
                #write_to_inner
            }

            fn size_hint(&self) -> usize {
                #size_hint_inner
            }
        }
    };

    output.into()
}
