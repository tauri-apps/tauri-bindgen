#[allow(clippy::all, unused)]
pub mod imports {

    #[::guest_rust::wasm_bindgen::prelude::wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
        pub async fn invoke(
            cmd: ::guest_rust::wasm_bindgen::JsValue,
            args: ::guest_rust::wasm_bindgen::JsValue,
        ) -> ::guest_rust::wasm_bindgen::JsValue;
    }

    /// A function that accepts a character
    pub async fn take_char(x: char) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: char,
        }
        let params = Params { x };
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|take_char"),
            ::guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    /// A function that returns a character
    pub async fn return_char() -> char {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|return_char"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
