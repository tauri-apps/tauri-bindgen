use serde::{de::DeserializeOwned, Serialize};
// #[cfg(feature = "macros")]
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
pub async fn invoke<P: Serialize, R: DeserializeOwned>(cmd: &str, val: P) -> Result<R, ()> {
    let raw = __TAURI_INVOKE__(
        JsValue::from_str(cmd),
        serde_wasm_bindgen::to_value(&val).map_err(|_| ())?,
    )
    .await
    .map_err(|_| ())?;

    serde_wasm_bindgen::from_value(raw).map_err(|_| ())
}

// pub async fn invoke<P: Writable, R: Readable>(cmd: &str, val: P) -> Result<R, ()> {
//     let mut bytes: Vec<u8> = Vec::new();
//     val.write_to(&mut bytes)
//         .expect("failed to serialize parameters");

//     let raw = __TAURI_INVOKE__(
//         JsValue::from_str(cmd),
//         JsValue::from_str(&general_purpose::STANDARD_NO_PAD.encode(bytes)),
//     )
//     .await
//     .map_err(|_| ())?;

//     let base64_encoded = raw.as_string().unwrap();
//     let bytes = general_purpose::STANDARD_NO_PAD.decode(base64_encoded).expect("failed to base64 decode response");

//     Readable::read_from(&mut bytes.as_slice()).map_err(|_| ())
// }

// Re-export `bitflags` so that we can reference it from macros.
#[doc(hidden)]
pub use bitflags;
