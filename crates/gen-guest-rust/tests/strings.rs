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

    pub async fn a(x: &str) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            x: &'a str,
        }
        let params = Params { x };
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|a"),
            ::guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn b() -> String {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|b"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn c(a: &str, b: &str) -> String {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            a: &'a str,
            b: &'a str,
        }
        let params = Params { a, b };
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|c"),
            ::guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
