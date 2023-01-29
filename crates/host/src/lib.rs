use base64::{engine::general_purpose, Engine};
pub use tauri_bindgen_host_macro::*;

#[doc(hidden)]
pub use {anyhow, async_trait::async_trait, bitflags, tauri, tracing};

pub use tauri_bindgen_abi;
// #[allow(unused_imports)]
// #[macro_use]
// pub extern crate tauri_bindgen_abi;

#[must_use]
pub fn decode_base64(base64_encoded: &str) -> Vec<u8> {
    general_purpose::STANDARD_NO_PAD
        .decode(base64_encoded)
        .expect("failed to base64 decode response")
}

#[must_use]
pub fn encode_base64(bytes: &[u8]) -> String {
    general_purpose::STANDARD_NO_PAD.encode(bytes)
}
