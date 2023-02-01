extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::HashSet;
use std::marker;
use std::path::PathBuf;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{token, Token};
use tauri_bindgen_core::{Files, WorldGenerator};

/// # Panics
///
/// TODO
pub fn generate<F, O, G>(input: TokenStream, mkgen: G) -> TokenStream
where
    F: Parse + Configure<O>,
    O: Default,
    G: FnOnce(O) -> Box<dyn WorldGenerator>,
{
    let input = syn::parse_macro_input!(input as Opts<F, O>);
    let mut gen = mkgen(input.opts);

    let iface = wit_parser::parse_file(&input.file, |t| input.skip.contains(t)).unwrap();

    let mut files = Files::default();
    let name = iface.name.clone();
    gen.generate(&name, &iface, &mut files, &input.file_hash);

    let (_, contents) = files.iter().next().unwrap();

    let contents = std::str::from_utf8(contents).unwrap();
    let mut contents = contents.parse::<TokenStream>().unwrap();

    // Include a dummy `include_str!` for the input file we read so rustc knows that
    // we depend on the contents of those files.
    contents.extend(
        format!(
            "const _: &str = include_str!(r#\"{}\"#);\n",
            &input.file.display()
        )
        .parse::<TokenStream>()
        .unwrap(),
    );

    contents
}

pub trait Configure<O> {
    fn configure(self, opts: &mut O);
}

struct Opts<F, O> {
    opts: O,
    skip: HashSet<String>,
    file: PathBuf,
    file_hash: String,
    _marker: marker::PhantomData<F>,
}

mod kw {
    syn::custom_keyword!(path);
    syn::custom_keyword!(skip);
}

impl<F, O> Parse for Opts<F, O>
where
    F: Parse + Configure<O>,
    O: Default,
{
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let call_site = proc_macro2::Span::call_site();

        let mut file: Option<PathBuf> = None;
        let mut ret = Opts {
            opts: O::default(),
            file: PathBuf::new(),
            file_hash: String::new(),
            skip: HashSet::new(),
            _marker: marker::PhantomData,
        };

        let l = input.lookahead1();

        if l.peek(token::Brace) {
            let content;
            syn::braced!(content in input);
            let fields = Punctuated::<ConfigField<F>, Token![,]>::parse_terminated(&content)?;
            for field in fields.into_pairs() {
                match field.into_value() {
                    ConfigField::Path(path) => {
                        let span = path.span();

                        let path = parse_path(&path);

                        ret.file_hash = tauri_bindgen_core::hash::hash_file(&path)
                            .map_err(|e| Error::new(span, e))?;

                        if file.replace(path).is_some() {
                            return Err(Error::new(span, "cannot specify second file"));
                        }
                    }
                    ConfigField::Skip(skip) => {
                        ret.skip = skip.iter().map(syn::LitStr::value).collect();
                    }
                    ConfigField::Other(other) => other.configure(&mut ret.opts),
                }
            }
        } else {
            let s = input.parse::<syn::LitStr>()?;
            let path = parse_path(&s);

            ret.file_hash =
                tauri_bindgen_core::hash::hash_file(&path).map_err(|e| Error::new(s.span(), e))?;

            file.replace(path);
        }

        ret.file = file.ok_or_else(|| {
            Error::new(
                call_site,
                "must specify a `*.wit` file to generate bindings for",
            )
        })?;

        Ok(ret)
    }
}

fn parse_path(path: &syn::LitStr) -> PathBuf {
    let path = path.value();
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    manifest_dir.join(path)
}

enum ConfigField<F> {
    Path(syn::LitStr),
    Skip(Vec<syn::LitStr>),
    Other(F),
}

impl<F: Parse> Parse for ConfigField<F> {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let l = input.lookahead1();
        if l.peek(kw::path) {
            input.parse::<kw::path>()?;
            input.parse::<Token![:]>()?;
            Ok(ConfigField::Path(input.parse()?))
        } else if l.peek(kw::skip) {
            input.parse::<kw::skip>()?;
            input.parse::<Token![:]>()?;
            let contents;
            syn::bracketed!(contents in input);
            let list = Punctuated::<_, Token![,]>::parse_terminated(&contents)?;
            Ok(ConfigField::Skip(list.iter().cloned().collect()))
        } else {
            Ok(ConfigField::Other(input.parse()?))
        }
    }
}
