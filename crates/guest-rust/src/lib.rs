use serde::{de::DeserializeOwned, Serialize};
// #[cfg(feature = "macros")]
pub use tauri_bindgen_guest_rust_macro::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window"], js_name = "__TAURI_INVOKE__")]
    async fn __TAURI_INVOKE__(cmd: JsValue, args: JsValue) -> JsValue;
}

pub async fn invoke<P: Serialize, R: DeserializeOwned>(cmd: &str, val: P) -> R{
    let raw = __TAURI_INVOKE__(
        JsValue::from_str(cmd),
        serde_wasm_bindgen::to_value(&val).unwrap(),
    )
    .await;

    serde_wasm_bindgen::from_value(raw).unwrap()
}

// Re-export `bitflags` so that we can reference it from macros.
#[doc(hidden)]
pub use bitflags;