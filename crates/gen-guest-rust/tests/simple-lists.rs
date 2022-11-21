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

    pub async fn simple_list1(l: &[u32]) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            l: &'a [u32],
        }
        let params = Params { l };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|simple_list1"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn simple_list2() -> Vec<u32> {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|simple_list2"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn simple_list4(l: &[&[u32]]) -> Vec<Vec<u32>> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            l: &'a [&'a [u32]],
        }
        let params = Params { l };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|simple_list4"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
