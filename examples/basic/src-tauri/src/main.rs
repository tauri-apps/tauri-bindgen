#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod plugin {
    use tauri::{
        plugin::{self, TauriPlugin},
        Runtime,
    };

    tauri_bindgen_host::generate!({
        path: "../greet.wit",
        async: false,
        tracing: true
    });

    #[derive(Clone, Copy)]
    struct Ctx;

    impl greet::Greet for Ctx {
        fn greet(&self, name: String) -> tauri_bindgen_host::anyhow::Result<String> {
            Ok(format!("Hello, {}! You've been greeted from code-generated Rust!", name))
        }
    }

    pub fn init<R: Runtime>() -> TauriPlugin<R> {
        plugin::Builder::new(greet::WORLD_HASH)
            .invoke_handler(greet::invoke_handler(Ctx))
            .build()
    }
}

use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    tauri::Builder::default()
        .plugin(plugin::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
