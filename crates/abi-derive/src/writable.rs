use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DataEnum, DataStruct};

pub fn writable_struct(data: &DataStruct) -> TokenStream {
    let fields = data.fields.iter().map(|syn::Field { ident, .. }| {
        quote! { self.#ident.write_to(write)?; }
    });

    quote! {
        #(#fields)*

        Ok(())
    }
}

pub fn writable_enum(data: &DataEnum) -> TokenStream {
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
            .map(|(i, syn::Variant { ident, fields, .. })| {
                let i = proc_macro2::Literal::usize_unsuffixed(i);

                match fields {
                    syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
                        let args = named.iter().map(|field| &field.ident);

                        let writes = named.iter().map(|syn::Field { ident, .. }| {
                            quote! {#ident.write_to(write)?;}
                        });

                        quote! {
                            Self::#ident{ #(#args),* } => {
                                #tag_type::write_to(&#i, write)?;
                                #(#writes)*
                            }
                        }
                    }
                    syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
                        let args = unnamed
                            .iter()
                            .enumerate()
                            .map(|(i, _)| format_ident!("_{}", i));

                        let writes = unnamed.iter().enumerate().map(|(i, _)| {
                            let ident = format_ident!("_{}", i);

                            quote! {#ident.write_to(write)?;}
                        });

                        quote! {
                            Self::#ident(#(#args),*) => {
                                #tag_type::write_to(&#i, write)?;
                                #(#writes)*
                            }
                        }
                    }
                    syn::Fields::Unit => {
                        quote! {
                            Self::#ident => {
                                #tag_type::write_to(&#i, write)?;
                            }
                        }
                    }
                }
            });

    quote! {
        match self {
            #(#variants),*
        }
        Ok(())
    }
}

pub fn writable_flags() -> TokenStream {
    quote! {
        self.bits().write_to(write)
    }
}
