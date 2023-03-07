mod logger;

use clap::{ArgAction, Parser};
use miette::{IntoDiagnostic, Result, WrapErr};
use std::path::PathBuf;

/// Helper for passing VERSION to opt.
/// If `CARGO_VERSION_INFO` is set, use it, otherwise use `CARGO_PKG_VERSION`.
fn version() -> &'static str {
    option_env!("CARGO_VERSION_INFO").unwrap_or(env!("CARGO_PKG_VERSION"))
}

#[derive(Debug, Parser)]
#[clap(version = version())]
struct Opt {
    #[clap(flatten)]
    common: Common,
    #[clap(subcommand)]
    category: Category,
    /// Enables verbose logging
    #[clap(short, long, global = true, action = ArgAction::Count)]
    verbose: u8,
}

#[derive(Debug, Parser)]
enum Category {
    /// Generator for creating bindings that are exposed to the WebView.
    Host(HostGenerator),
    // /// Generators for webview libraries.
    // #[clap(subcommand)]
    // Guest(GuestGenerator),
    // /// This generator outputs a Markdown file describing an interface.
    // Markdown {
    //     #[clap(flatten)]
    //     opts: tauri_bindgen_gen_markdown::Opts,
    //     #[clap(flatten)]
    //     world: WorldOpt,
    // },
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
    // /// Generates bindings for Rust guest modules using wasm-bindgen.
    // Rust {
    //     #[clap(flatten)]
    //     opts: tauri_bindgen_gen_guest_rust::Opts,
    //     #[clap(flatten)]
    //     world: WorldOpt,
    // },
    // /// Generates bindings for JavaScript guest modules.
    // Javascript {
    //     #[clap(flatten)]
    //     opts: tauri_bindgen_gen_guest_js::Opts,
    //     #[clap(flatten)]
    //     world: WorldOpt,
    // },
    // /// Generates bindings for TypeScript guest modules.
    // Typescript {
    //     #[clap(flatten)]
    //     opts: tauri_bindgen_gen_guest_ts::Opts,
    //     #[clap(flatten)]
    //     world: WorldOpt,
    // },
}

#[derive(Debug, Parser)]
struct WorldOpt {
    // #[clap(value_name = "DOCUMENT", value_parser = parse_interface)]
    /// Generate bindings for the WIT document.
    wit: PathBuf,
    /// Names of functions to skip generating bindings for.
    #[clap(long)]
    skip: Vec<String>,
}

#[derive(Debug, Parser, Clone)]
struct Common {
    /// Where to place output files
    #[clap(global = true, long = "out-dir")]
    out_dir: Option<PathBuf>,
}

// A thin wrapper around `run` so we can pretty-print the error
fn main() {
    if let Err(err) = run() {
        log::error!("{:?}", err);
    }
}

fn run() -> Result<()> {
    let opt: Opt = Opt::parse();

    logger::init(opt.verbose);

    let (path, contents) = match opt.category {
        Category::Host(HostGenerator { opts, world, .. }) => {
            let source = std::fs::read_to_string(&world.wit).unwrap();
            let iface = wit_parser::parse_str(source, |_| false).unwrap();
            opts.build().generate(&iface)
        } // Category::Guest(GuestGenerator::Rust { opts, world, .. }) => {
          //     todo!()
          //     // gen_world(opts.build(), world, &mut files)?;
          // }
          // Category::Guest(GuestGenerator::Javascript { opts, world, .. }) => {
          //     todo!()
          //     // gen_world(opts.build(), world, &mut files)?;
          // }
          // Category::Guest(GuestGenerator::Typescript { opts, world, .. }) => {
          //     // gen_world(opts.build(), world, &mut files)?;
          // }
          // Category::Markdown { opts, world, .. } => {
          //     // gen_world(opts.build(), world, &mut files)?;
          // }
    };

    let dst = opt.common.out_dir.unwrap_or_default().join(path);

    log::info!("Generating {dst:?}");
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent)
            .into_diagnostic()
            .wrap_err(format!("failed to create {parent:?}"))?;
    }
    std::fs::write(&dst, contents)
        .into_diagnostic()
        .wrap_err(format!("failed to write {dst:?}"))?;

    Ok(())
}

// fn gen_world(
//     mut generator: Box<dyn WorldGenerator>,
//     opts: WorldOpt,
//     files: &mut Files,
// ) -> Result<()> {
//     if !opts.wit.is_file() {
//         bail!("wit file `{}` does not exist", opts.wit.display());
//     }

//     let skipset: HashSet<String, std::collections::hash_map::RandomState> =
//         opts.skip.into_iter().collect();

//     let world = wit_parser::parse_file(&opts.wit, |t| skipset.contains(t))?;
//     let world_hash = tauri_bindgen_core::hash::hash_file(opts.wit)?;

//     generator.generate(&world.name, &world, files, &world_hash);
//     Ok(())
// }
