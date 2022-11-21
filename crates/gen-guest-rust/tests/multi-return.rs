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

    pub async fn mra() -> () {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|mra"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn mrb() -> () {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|mrb"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn mrc() -> u32 {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|mrc"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn mrd() -> u32 {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|mrd"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn mre() -> (u32, f32) {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|mre"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
