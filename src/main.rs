use clap::{ArgAction, Parser};
use env_logger::fmt::Color;
use log::{log_enabled, Level};
use miette::{bail, IntoDiagnostic, Result, WrapErr};
use std::io::Write;
use std::path::PathBuf;
use tauri_bindgen_core::{Files, WorldGenerator};

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
    /// Generators for webview libraries.
    #[clap(subcommand)]
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
    // #[clap(value_name = "DOCUMENT", value_parser = parse_interface)]
    /// Generate bindings for the WIT document.
    wit: PathBuf,
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

    init_logger(opt.verbose);

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
        let dst = match &opt.common.out_dir {
            Some(path) => path.join(name),
            None => name.into(),
        };
        log::info!("Generating {dst:?}");
        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent)
                .into_diagnostic()
                .wrap_err(format!("failed to create {parent:?}"))?;
        }
        std::fs::write(&dst, contents)
            .into_diagnostic()
            .wrap_err(format!("failed to write {dst:?}"))?;
    }

    Ok(())
}

fn gen_world(
    mut generator: Box<dyn WorldGenerator>,
    opts: WorldOpt,
    files: &mut Files,
) -> Result<()> {
    if !opts.wit.is_file() {
        bail!("wit file `{}` does not exist", opts.wit.display());
    }

    let world = wit_parser::parse_file(&opts.wit)?;
    let world_hash = tauri_bindgen_core::hash::hash_file(opts.wit)?;

    generator.generate(&world.name, &world, files, &world_hash);
    Ok(())
}

/// This maps the occurrence of `--verbose` flags to the correct log level
fn verbosity_level(num: u8) -> Level {
    match num {
        0 => Level::Info,
        1 => Level::Debug,
        2.. => Level::Trace,
    }
}

/// The default string representation for `Level` is all uppercaps which doesn't mix well with the other printed actions.
fn prettyprint_level(lvl: Level) -> &'static str {
    match lvl {
        Level::Error => "Error",
        Level::Warn => "Warn",
        Level::Info => "Info",
        Level::Debug => "Debug",
        Level::Trace => "Trace",
    }
}

fn init_logger(verbosity: u8) {
    let mut builder = env_logger::Builder::from_default_env();

    builder
        .format_indent(Some(12))
        .filter(None, verbosity_level(verbosity).to_level_filter())
        .format(|f, record| {
            let mut is_command_output = false;
            if let Some(action) = record.key_values().get("action".into()) {
                let action = action.to_str().unwrap();
                is_command_output = action == "stdout" || action == "stderr";
                if !is_command_output {
                    let mut action_style = f.style();
                    action_style.set_color(Color::Green).set_bold(true);

                    write!(f, "{:>12} ", action_style.value(action))?;
                }
            } else {
                let mut level_style = f.default_level_style(record.level());
                level_style.set_bold(true);

                write!(
                    f,
                    "{:>12} ",
                    level_style.value(prettyprint_level(record.level()))
                )?;
            }

            if !is_command_output && log_enabled!(Level::Debug) {
                let mut target_style = f.style();
                target_style.set_color(Color::Black);

                write!(f, "[{}] ", target_style.value(record.target()))?;
            }

            writeln!(f, "{}", record.args())
        })
        .init();
}
