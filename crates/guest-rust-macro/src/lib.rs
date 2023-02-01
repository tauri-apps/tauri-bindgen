use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;
use tauri_bindgen_gen_guest_rust::Opts;

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    rust_macro_shared::generate::<Opt, Opts, _>(input, |opts| opts.build())
}

mod kw {
    syn::custom_keyword!(unchecked);
    syn::custom_keyword!(no_std);
    syn::custom_keyword!(skip);
}

enum Opt {
    Unchecked(bool),
    NoStd(bool),
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
        }
    }
}
