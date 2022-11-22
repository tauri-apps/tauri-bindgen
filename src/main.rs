use anyhow::{bail, Context};
use clap::Parser;
use std::path::{Path, PathBuf};
use tauri_bindgen_core::{Files, WorldGenerator};
use wit_parser::World;

/// Helper for passing VERSION to opt.
/// If CARGO_VERSION_INFO is set, use it, otherwise use CARGO_PKG_VERSION.
fn version() -> &'static str {
    option_env!("CARGO_VERSION_INFO").unwrap_or(env!("CARGO_PKG_VERSION"))
}

#[derive(Debug, Parser)]
#[command(version = version())]
struct Opt {
    #[clap(flatten)]
    common: Common,
    #[command(subcommand)]
    category: Category,
}

#[derive(Debug, Parser)]
enum Category {
    /// Generator for creating bindings that are exposed to the WebView.
    Host(HostGenerator),
    /// Generators for webview libraries.
    #[command(subcommand)]
    Guest(GuestGenerator),
    /// This generator outputs a Markdown file describing an interface.
    Markdown {
        #[clap(flatten)]
        opts: tauri_bindgen_gen_markdown::Opts,
        #[clap(flatten)]
        world: WorldOpt,
    },
}

#[derive(Debug, Parser)]
struct HostGenerator {
    #[clap(flatten)]
    opts: tauri_bindgen_gen_host::Opts,
    #[clap(flatten)]
    world: WorldOpt,
}

#[derive(Debug, Parser)]
enum GuestGenerator {
    /// Generates bindings for Rust guest modules using wasm-bindgen.
    Rust {
        #[clap(flatten)]
        opts: tauri_bindgen_gen_guest_rust::Opts,
        #[clap(flatten)]
        world: WorldOpt,
    },
    /// Generates bindings for JavaScript guest modules.
    Javascript {
        #[clap(flatten)]
        opts: tauri_bindgen_gen_guest_js::Opts,
        #[clap(flatten)]
        world: WorldOpt,
    },
    /// Generates bindings for TypeScript guest modules.
    Typescript {
        #[clap(flatten)]
        opts: tauri_bindgen_gen_guest_ts::Opts,
        #[clap(flatten)]
        world: WorldOpt,
    },
}

#[derive(Debug, Parser)]
struct WorldOpt {
    /// Generate bindings for the WIT document.
    #[clap(value_name = "DOCUMENT", value_parser = parse_world)]
    wit: World,
}

fn parse_world(s: &str) -> anyhow::Result<World> {
    let path = Path::new(s);
    if !path.is_file() {
        bail!("wit file `{}` does not exist", path.display());
    }

    let world = World::parse_file(&path)
        .with_context(|| format!("failed to parse wit file `{}`", path.display()))
        .map_err(|e| {
            eprintln!("{e:?}");
            e
        })?;

    Ok(world)
}

#[derive(Debug, Parser)]
struct ComponentOpts {
    /// Path to the input wasm component to generate bindings for.
    component: PathBuf,

    /// Optionally rename the generated bindings instead of inferring the name
    /// from the input `component` path.
    #[clap(long)]
    name: Option<String>,

    #[clap(flatten)]
    common: Common,
}

#[derive(Debug, Parser, Clone)]
struct Common {
    /// Where to place output files
    #[clap(long = "out-dir")]
    out_dir: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let opt: Opt = Opt::parse();
    let common = opt.common;

    let mut files = Files::default();
    match opt.category {
        Category::Host(HostGenerator { opts, world, .. }) => {
            gen_world(opts.build(), world, &mut files)?;
        }
        Category::Guest(GuestGenerator::Rust { opts, world, .. }) => {
            gen_world(opts.build(), world, &mut files)?;
        }
        Category::Guest(GuestGenerator::Javascript { opts, world, .. }) => {
            gen_world(opts.build(), world, &mut files)?;
        }
        Category::Guest(GuestGenerator::Typescript { opts, world, .. }) => {
            gen_world(opts.build(), world, &mut files)?;
        }
        Category::Markdown { opts, world, .. } => {
            gen_world(opts.build(), world, &mut files)?;
        }
    }

    for (name, contents) in files.iter() {
        let dst = match &common.out_dir {
            Some(path) => path.join(name),
            None => name.into(),
        };
        println!("Generating {:?}", dst);
        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {:?}", parent))?;
        }
        std::fs::write(&dst, contents).with_context(|| format!("failed to write {:?}", dst))?;
    }

    Ok(())
}

fn gen_world(
    mut generator: Box<dyn WorldGenerator>,
    opts: WorldOpt,
    files: &mut Files,
) -> anyhow::Result<()> {
    let name = opts.wit.name.clone();
    let interfaces = ComponentInterfaces::from(opts.wit);
    generator.generate(&name, &interfaces, files);
    Ok(())
}
