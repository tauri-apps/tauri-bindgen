use env_logger::fmt::Color;
use log::{log_enabled, Level};
use std::io::Write;

pub fn init(verbosity: u8) {
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
