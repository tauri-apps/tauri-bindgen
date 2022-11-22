use proc_macro::TokenStream;
use syn::parse::{Parse, ParseStream, Result};
use syn::Token;
use tauri_bindgen_gen_host::Opts;

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    rust_macro_shared::generate::<Opt, Opts>(input, |opts| opts.build())
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

impl rust_macro_shared::Configure<Opts> for Opt {
    fn configure(self, opts: &mut Opts) {
        match self {
            Opt::Async(val) => opts.async_ = val,
            Opt::Tracing(val) => opts.tracing = val,
        }
    }
}
