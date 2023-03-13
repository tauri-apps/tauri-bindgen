#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod roundtrip;

use tauri_plugin_log::{LogTarget, LoggerBuilder};

#[tauri::command]
fn exit_with_error(e: &str) -> bool {
    eprintln!("{}", e);
    std::process::exit(1);
}

fn main() {
    let log_plugin = {
        let targets = [
            LogTarget::LogDir,
            #[cfg(debug_assertions)]
            LogTarget::Stdout,
            #[cfg(debug_assertions)]
            LogTarget::Webview,
        ];

        LoggerBuilder::new().targets(targets).build()
    };

    tauri::Builder::default()
        .plugin(log_plugin)
        .plugin(roundtrip::init())
        .invoke_handler(tauri::generate_handler![exit_with_error])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
