// #[cfg(feature = "macros")]
pub use guest_rust_macro::*;

// Re-export `bitflags` so that we can reference it from macros.
#[doc(hidden)]
pub use {bitflags, serde, serde_wasm_bindgen, wasm_bindgen};