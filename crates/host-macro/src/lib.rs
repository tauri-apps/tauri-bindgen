use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;
use tauri_bindgen_gen_host::Builder;

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    rust_macro_shared::generate::<Opt, Builder>(input)
}

mod kw {
    syn::custom_keyword!(tracing);
}

enum Opt {
    Async(bool),
    Tracing(bool),
}

impl Parse for Opt {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let l = input.lookahead1();

        if l.peek(Token![async]) {
            input.parse::<Token![async]>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::Async(input.parse::<syn::LitBool>()?.value))
        } else if l.peek(kw::tracing) {
            input.parse::<kw::tracing>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::Tracing(input.parse::<syn::LitBool>()?.value))
        } else {
            Err(l.error())
        }
    }
}

impl rust_macro_shared::Configure<Builder> for Opt {
    fn configure(self, builder: &mut Builder) {
        match self {
            Opt::Async(val) => builder.async_ = val,
            Opt::Tracing(val) => builder.tracing = val,
        }
    }
}
