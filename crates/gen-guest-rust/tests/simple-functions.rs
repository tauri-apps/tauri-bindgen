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

    pub async fn f1() -> () {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|f1"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn f2(a: u32) -> () {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            a: u32,
        }
        let params = Params { a };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|f2"),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn f3(a: u32, b: u32) -> () {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            a: u32,
            b: u32,
        }
        let params = Params { a, b };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|f3"),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn f4() -> u32 {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|f4"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn f5() -> (u32, u32) {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|f5"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn f6(a: u32, b: u32, c: u32) -> (u32, u32, u32) {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            a: u32,
            b: u32,
            c: u32,
        }
        let params = Params { a, b, c };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|f6"),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
