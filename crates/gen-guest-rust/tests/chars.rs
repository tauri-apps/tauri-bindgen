#[allow(clippy::all, unused)]
pub mod imports {

    #[::tauri_bindgen_guest_rust::wasm_bindgen::prelude::wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
        pub async fn invoke(
            cmd: ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue,
            args: ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue,
        ) -> ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue;
    }

    /// A function that accepts a character
    pub async fn take_char(x: char) -> () {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: char,
        }
        let params = Params { x };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|take_char"),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    /// A function that returns a character
    pub async fn return_char() -> char {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str(
                "plugin:imports|return_char",
            ),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
