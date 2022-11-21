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

    pub async fn a1(x: u8) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u8,
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|a1"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a2(x: i8) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i8,
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|a2"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a3(x: u16) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u16,
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|a3"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a4(x: i16) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i16,
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|a4"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a5(x: u32) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u32,
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|a5"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a6(x: i32) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i32,
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|a6"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a7(x: u64) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u64,
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|a7"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a8(x: i64) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i64,
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|a8"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a9(p1: u8, p2: i8, p3: u16, p4: i16, p5: u32, p6: i32, p7: u64, p8: i64) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            p1: u8,
            p2: i8,
            p3: u16,
            p4: i16,
            p5: u32,
            p6: i32,
            p7: u64,
            p8: i64,
        }
        let params = Params {
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
            p8,
        };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|a9"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r1() -> u8 {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|r1"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r2() -> i8 {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|r2"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r3() -> u16 {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|r3"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r4() -> i16 {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|r4"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r5() -> u32 {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|r5"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r6() -> i32 {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|r6"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r7() -> u64 {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|r7"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r8() -> i64 {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|r8"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn pair_ret() -> (i64, u8) {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|pair_ret"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
