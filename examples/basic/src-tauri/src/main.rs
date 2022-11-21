#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod plugin {
    use tauri::{
        plugin::{self, TauriPlugin},
        Runtime,
    };

    host::generate!({
        path: "../greet.wit",
        async: false
    });

    struct Ctx;

    impl greet::Greet for Ctx {
        fn greet(&self, name: String) -> host::anyhow::Result<String> {
            Ok(format!("Hello, {}! You've been greeted from code-generated Rust!", name))
        }
    }

    pub fn init<R: Runtime>() -> TauriPlugin<R> {
        plugin::Builder::new("greet")
            .invoke_handler(greet::invoke_handler(Ctx))
            .build()
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(plugin::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
