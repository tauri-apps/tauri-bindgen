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

    #[repr(C)]
    #[derive(Debug, Copy, Clone, ::serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LudicrousSpeed {
        pub how_fast_are_you_going: u32,
        pub i_am_going_extremely_slow: u64,
    }
    pub async fn kebab_case() -> () {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|kebab_case"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn foo(x: LudicrousSpeed) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: LudicrousSpeed,
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|foo"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn function_with_dashes() -> () {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|function_with_dashes"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn function_with_no_weird_characters() -> () {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|function_with_no_weird_characters"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn apple() -> () {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|apple"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn apple_pear() -> () {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|apple_pear"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn apple_pear_grape() -> () {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|apple_pear_grape"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a0() -> () {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|a0"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn is_xml() -> () {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|is_xml"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn explicit() -> () {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|explicit"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn explicit_kebab() -> () {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|explicit_kebab"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn bool() -> () {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|bool"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
