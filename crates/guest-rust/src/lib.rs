use std::fmt::Debug;

use base64::{engine::general_purpose, Engine};
use serde::Serialize;
use tauri_bindgen_abi::{Readable, Writable};

pub use {tauri_bindgen_abi, tracing};

pub use tauri_bindgen_guest_rust_macro::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window"], js_name = "__TAURI_INVOKE__")]
    async fn __TAURI_INVOKE__(cmd: JsValue, args: JsValue) -> Result<JsValue, JsValue>;
}

/// # Errors
///
/// Invoking commands can always fail when the Host implementation decides to fail the operation,
/// when the Host implementation panics, or when somehting fails during serialization/deserialization or transmision of the message.
/// 
/// # Panics
/// 
/// TODO
#[tracing::instrument]
pub async fn invoke<P: Writable + Debug, R: Readable + Debug>(cmd: &str, val: P) -> Result<R, ()> {
    let bytes = tauri_bindgen_abi::to_bytes(&val)
        .expect("failed to serialize parameters");

    tracing::debug!("request bytes: {:#01x?}", bytes);

    let raw = __TAURI_INVOKE__(
        JsValue::from_str(cmd),
        serde_wasm_bindgen::to_value(&Request {
            encoded: general_purpose::STANDARD_NO_PAD.encode(bytes)
        }).unwrap()
    )
    .await
    .map_err(|_| ())?;

    let base64_encoded = raw.as_string().unwrap();
    let bytes = general_purpose::STANDARD_NO_PAD.decode(base64_encoded).expect("failed to base64 decode response");

    tracing::debug!("response bytes: {:?}", bytes);

    Readable::read_from(&mut bytes.as_slice()).map_err(|_| ())
}

#[derive(Serialize)]
struct Request {
    encoded: String
}

// pub async fn invoke<P: Serialize, R: DeserializeOwned>(cmd: &str, val: P) -> Result<R, ()> {
//     let raw = __TAURI_INVOKE__(
//         JsValue::from_str(cmd),
//         serde_wasm_bindgen::to_value(&val).map_err(|_| ())?,
//     )
//     .await
//     .map_err(|_| ())?;

//     serde_wasm_bindgen::from_value(raw).map_err(|_| ())
// }

// Re-export `bitflags` so that we can reference it from macros.
#[doc(hidden)]
pub use bitflags;
