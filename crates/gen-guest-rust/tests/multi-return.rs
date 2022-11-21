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

    pub async fn mra() -> () {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|mra"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn mrb() -> () {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|mrb"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn mrc() -> u32 {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|mrc"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn mrd() -> u32 {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|mrd"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn mre() -> (u32, f32) {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|mre"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
