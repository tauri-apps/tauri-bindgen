use serde::{de::DeserializeOwned, Serialize};
// #[cfg(feature = "macros")]
pub use tauri_bindgen_guest_rust_macro::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    pub async fn invoke(cmd: JsValue, args: JsValue) -> JsValue;
}

#[doc(hidden)]
pub async fn send<P: Serialize, R: DeserializeOwned>(cmd: &str, val: P) -> R{
    let raw = invoke(
        JsValue::from_str(cmd),
        serde_wasm_bindgen::to_value(&val).unwrap(),
    )
    .await;

    serde_wasm_bindgen::from_value(raw).unwrap()
}

// Re-export `bitflags` so that we can reference it from macros.
#[doc(hidden)]
pub use bitflags;

#[cfg(test)]
mod test {
    bitflags::bitflags! {
        struct Flags: u64 {
            const NONE = 0b00000001;
            const SOME = 0b00000010;
        }
    }
}