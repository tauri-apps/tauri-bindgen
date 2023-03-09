use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, DataStruct};

pub fn readable_struct(data: &DataStruct) -> TokenStream {
    let fields = data.fields.iter().map(
        |syn::Field { ident, .. }| quote! { #ident : &tauri_bindgen_abi::Readable::read_from(bytes)?},
    );

    quote! {
        Ok(Self {
            #(#fields),*
        })
    }
}

pub fn readable_enum(data: &DataEnum) -> TokenStream {
    let tag_type = match data.variants.len() {
        n if u8::try_from(n).is_ok() => quote! { u8 },
        n if u16::try_from(n).is_ok() => quote! { u16 },
        n if u32::try_from(n).is_ok() => quote! { u32 },
        n if u64::try_from(n).is_ok() => quote! { u64 },
        n if u128::try_from(n).is_ok() => quote! { u128 },
        _ => panic!("too many cases to fit in a repr"),
    };

    let variants =
        data.variants
            .iter()
            .enumerate()
            .map(|(i, syn::Variant { fields, ident, .. })| {
                let inner = match fields {
                    syn::Fields::Named(fields) => {
                        let fields = fields.named.iter().map(|syn::Field { ident, .. }| {
                            quote! {
                                #ident: &tauri_bindgen_abi::Readable::read_from(bytes)?
                            }
                        });

                        quote! { { #(#fields),* } }
                    }
                    syn::Fields::Unnamed(fields) => {
                        let fields = fields
                            .unnamed
                            .iter()
                            .map(|_| quote! { tauri_bindgen_abi::Readable::read_from(bytes)? });

                        quote! { (#(#fields),*) }
                    }
                    syn::Fields::Unit => proc_macro2::TokenStream::new(),
                };

                let i = proc_macro2::Literal::usize_unsuffixed(i);
                quote! { #i => { Ok(Self::#ident #inner) } }
            });

    quote! {
        let tag = #tag_type::read_from(bytes)?;

        match tag {
            #(#variants),*
            _ => { panic!("unknown enum tag") },
        }
    }
}

pub fn readable_flags() -> TokenStream {
    quote! {
        let bits = tauri_bindgen_abi::Readable::read_from(bytes)?;
        Self::from_bits(bits).ok_or(tauri_bindgen_abi::Error::InvalidFlags)
    }
}
