use serde::{de::DeserializeOwned, Serialize};
// #[cfg(feature = "macros")]
pub use tauri_bindgen_guest_rust_macro::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch, js_namespace = ["window"], js_name = "__TAURI_INVOKE__")]
    async fn __TAURI_INVOKE__(cmd: JsValue, args: JsValue) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(js_namespace = ["console"], js_name = "warn")]
    pub fn console_warn(msg: &str);
}

pub async fn invoke<P: Serialize, R: DeserializeOwned>(cmd: &str, val: P) -> Result<R, ()> {
    let raw = __TAURI_INVOKE__(
        JsValue::from_str(cmd),
        serde_wasm_bindgen::to_value(&val).map_err(|_| ())?,
    )
    .await
    .map_err(|_| ())?;

    serde_wasm_bindgen::from_value(raw).map_err(|_| ())
}

// Re-export `bitflags` so that we can reference it from macros.
#[doc(hidden)]
pub use {bitflags, once_cell::sync::Lazy, wasm_bindgen_futures};
