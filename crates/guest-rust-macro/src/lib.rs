use tauri_bindgen_gen_guest_rust::Opts;
use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Token, LitStr};

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    rust_macro_shared::generate::<Opt, Opts>(input, |opts| opts.build())
}

mod kw {
    syn::custom_keyword!(unchecked);
    syn::custom_keyword!(no_std);
    syn::custom_keyword!(skip);
}

enum Opt {
    Unchecked(bool),
    NoStd(bool),
    Skip(Vec<LitStr>),
}

impl Parse for Opt {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let l = input.lookahead1();

        if l.peek(kw::unchecked) {
            input.parse::<kw::unchecked>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::Unchecked(input.parse::<syn::LitBool>()?.value))
        } else if l.peek(kw::no_std) {
            input.parse::<kw::no_std>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::NoStd(input.parse::<syn::LitBool>()?.value))
        }  else if l.peek(kw::skip) {
            input.parse::<kw::skip>()?;
            input.parse::<Token![:]>()?;
            let contents;
            syn::bracketed!(contents in input);
            let list = Punctuated::<_, Token![,]>::parse_terminated(&contents)?;
            Ok(Opt::Skip(list.iter().cloned().collect()))
        } else {
            Err(l.error())
        }
    }
}

impl rust_macro_shared::Configure<Opts> for Opt {
    fn configure(self, opts: &mut Opts) {
        match self {
            Opt::Unchecked(val) => opts.unchecked = val,
            Opt::NoStd(val) => opts.no_std = val,
            Opt::Skip(val) => opts.skip.extend(val.iter().map(|i| i.value())),
        }
    }
}
