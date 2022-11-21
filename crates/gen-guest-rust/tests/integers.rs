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

    pub async fn a1(x: u8) -> () {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u8,
        }
        let params = Params { x };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|a1"),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a2(x: i8) -> () {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i8,
        }
        let params = Params { x };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|a2"),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a3(x: u16) -> () {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u16,
        }
        let params = Params { x };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|a3"),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a4(x: i16) -> () {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i16,
        }
        let params = Params { x };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|a4"),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a5(x: u32) -> () {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u32,
        }
        let params = Params { x };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|a5"),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a6(x: i32) -> () {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i32,
        }
        let params = Params { x };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|a6"),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a7(x: u64) -> () {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u64,
        }
        let params = Params { x };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|a7"),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a8(x: i64) -> () {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i64,
        }
        let params = Params { x };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|a8"),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn a9(p1: u8, p2: i8, p3: u16, p4: i16, p5: u32, p6: i32, p7: u64, p8: i64) -> () {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
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
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|a9"),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r1() -> u8 {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|r1"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r2() -> i8 {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|r2"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r3() -> u16 {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|r3"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r4() -> i16 {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|r4"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r5() -> u32 {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|r5"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r6() -> i32 {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|r6"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r7() -> u64 {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|r7"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn r8() -> i64 {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|r8"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn pair_ret() -> (i64, u8) {
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|pair_ret"),
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
