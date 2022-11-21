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

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, ::serde::Deserialize)]
    pub enum Error {
        Success,
        Failure,
    }
    pub async fn option_test() -> Result<Option<String>, Error> {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|option_test"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
