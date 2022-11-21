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

    pub async fn float32_param(x: f32) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: f32,
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|float32_param"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn float64_param(x: f64) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: f64,
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|float64_param"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn float32_result() -> f32 {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|float32_result"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn float64_result() -> f64 {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|float64_result"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
