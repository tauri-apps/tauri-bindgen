// #[cfg(feature = "macros")]
pub use tauri_bindgen_guest_rust_macro::*;


// Re-export `bitflags` so that we can reference it from macros.
#[doc(hidden)]
pub use {bitflags, serde_wasm_bindgen, wasm_bindgen, serde};