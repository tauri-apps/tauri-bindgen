#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod roundtrip;

use tauri_bindgen_host::ipc_router_wip::{BuilderExt, Router};
use tauri_plugin_log::{Builder as LoggerBuilder, LogTarget};

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

    let mut router = Router::new(roundtrip::Ctx);

    roundtrip::add_to_router(&mut router, |cx| cx).unwrap();

    tauri::Builder::default()
        .plugin(log_plugin)
        .ipc_router(router)
        .invoke_handler(tauri::generate_handler![exit_with_error])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
