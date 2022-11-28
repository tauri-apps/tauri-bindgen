extern crate proc_macro;

use proc_macro::TokenStream;
use wit_parser::Interface;
use std::marker;
use std::path::{Path, PathBuf};
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{token, Token};
use tauri_bindgen_core::{Files, WorldGenerator};
// use wit_parser::World;

pub fn generate<F, O>(
    input: TokenStream,
    mkgen: impl FnOnce(O) -> Box<dyn WorldGenerator>,
) -> TokenStream
where
    F: Parse + Configure<O>,
    O: Default,
{
    let input = syn::parse_macro_input!(input as Opts<F, O>);
    let mut gen = mkgen(input.opts);
    let mut files = Files::default();
    let name = input.interface.name.clone();
    gen.generate(&name, &input.interface, &mut files, &input.file_hash);

    let (_, contents) = files.iter().next().unwrap();

    let contents = std::str::from_utf8(contents).unwrap();
    let mut contents = contents.parse::<TokenStream>().unwrap();

    // Include a dummy `include_str!` for any files we read so rustc knows that
    // we depend on the contents of those files.
    let cwd = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    for file in input.files.iter() {
        contents.extend(
            format!(
                "const _: &str = include_str!(r#\"{}\"#);\n",
                Path::new(&cwd).join(file).display()
            )
            .parse::<TokenStream>()
            .unwrap(),
        );
    }

    return contents;
}

pub trait Configure<O> {
    fn configure(self, opts: &mut O);
}

struct Opts<F, O> {
    opts: O,
    interface: Interface,
    files: Vec<String>,
    file_hash: String,
    _marker: marker::PhantomData<F>,
}

mod kw {
    syn::custom_keyword!(path);
}

impl<F, O> Parse for Opts<F, O>
where
    F: Parse + Configure<O>,
    O: Default,
{
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let call_site = proc_macro2::Span::call_site();
        
        let mut iface: Option<Interface> = None;
        let mut ret = Opts {
            opts: O::default(),
            interface: Interface::default(),
            files: Vec::new(),
            file_hash: String::new(),
            _marker: marker::PhantomData,
        };

        if input.peek(token::Brace) {
            let content;
            syn::braced!(content in input);
            let fields = Punctuated::<ConfigField<F>, Token![,]>::parse_terminated(&content)?;
            for field in fields.into_pairs() {
                match field.into_value() {
                    ConfigField::Path(path) => {
                        let span = path.span();
                        if iface.is_some() {
                            return Err(Error::new(span, "cannot specify second world"));
                        }

                        let path = ret.parse_path(&path);

                        iface = Some(wit_parser::parse_file(&path).map_err(|e| Error::new(span, e))?);
                        ret.file_hash = tauri_bindgen_core::hash::hash_file(path)
                            .map_err(|e| Error::new(span, e))?;
                    }
                    ConfigField::Other(other) => other.configure(&mut ret.opts),
                }
            }
        } else {
            let s = input.parse::<syn::LitStr>()?;

            let path = ret.parse_path(&s);

            iface = Some(wit_parser::parse_file(&path).map_err(|e| Error::new(s.span(), e))?);
            ret.file_hash =
                tauri_bindgen_core::hash::hash_file(path).map_err(|e| Error::new(s.span(), e))?;
        }

        ret.interface = iface.ok_or_else(|| {
            Error::new(
                call_site,
                "must specify a `*.wit` file to generate bindings for",
            )
        })?;

        Ok(ret)
    }
}

impl<F, O> Opts<F, O> {
    fn parse_path(&mut self, path: &syn::LitStr) -> PathBuf {
        let path = path.value();
        let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let path = manifest_dir.join(path);
        self.files.push(path.to_str().unwrap().to_string());

        path
    }
}

enum ConfigField<F> {
    Path(syn::LitStr),
    Other(F),
}

impl<F: Parse> Parse for ConfigField<F> {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let l = input.lookahead1();
        if l.peek(kw::path) {
            input.parse::<kw::path>()?;
            input.parse::<Token![:]>()?;
            Ok(ConfigField::Path(input.parse()?))
        } else {
            Ok(ConfigField::Other(input.parse()?))
        }
    }
}
