#[allow(clippy::all, unused)]
pub mod imports {

    #[::wasm_bindgen::prelude::wasm_bindgen]
    extern "C" {
        #[::wasm_bindgen::prelude::wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
        pub async fn invoke(
            cmd: ::wasm_bindgen::prelude::JsValue,
            args: ::wasm_bindgen::prelude::JsValue,
        ) -> ::wasm_bindgen::prelude::JsValue;
    }

    pub async fn mra() -> () {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|mra"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn mrb() -> () {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|mrb"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn mrc() -> u32 {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|mrc"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn mrd() -> u32 {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|mrd"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn mre() -> (u32, f32) {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|mre"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
