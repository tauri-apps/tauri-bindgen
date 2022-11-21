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

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, ::serde::Serialize, ::serde::Deserialize)]
    pub enum E1 {
        A,
    }
    #[derive(Debug, Clone, Copy, ::serde::Serialize, ::serde::Deserialize)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, ::serde::Serialize, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Empty {}
    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
    pub enum V1Param<'a> {
        A,
        B(U1),
        C(E1),
        D(&'a str),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
    pub enum V1Result {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(Debug, Clone, Copy, ::serde::Serialize, ::serde::Deserialize)]
    pub enum Casts1 {
        A(i32),
        B(f32),
    }
    #[derive(Debug, Clone, Copy, ::serde::Serialize, ::serde::Deserialize)]
    pub enum Casts2 {
        A(f64),
        B(f32),
    }
    #[derive(Debug, Clone, Copy, ::serde::Serialize, ::serde::Deserialize)]
    pub enum Casts3 {
        A(f64),
        B(u64),
    }
    #[derive(Debug, Clone, Copy, ::serde::Serialize, ::serde::Deserialize)]
    pub enum Casts4 {
        A(u32),
        B(i64),
    }
    #[derive(Debug, Clone, Copy, ::serde::Serialize, ::serde::Deserialize)]
    pub enum Casts5 {
        A(f32),
        B(i64),
    }
    #[derive(Debug, Clone, Copy, ::serde::Serialize, ::serde::Deserialize)]
    pub enum Casts6 {
        A((f32, u32)),
        B((u32, u32)),
    }
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, ::serde::Serialize, ::serde::Deserialize)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct IsCloneParam<'a> {
        #[serde(borrow)]
        pub v1: V1Param<'a>,
    }
    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct IsCloneResult {
        pub v1: V1Result,
    }
    pub async fn e1_arg(x: E1) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: E1,
        }
        let params = Params { x };
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|e1_arg"),
            ::guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn e1_result() -> E1 {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|e1_result"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn u1_arg(x: U1) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: U1,
        }
        let params = Params { x };
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|u1_arg"),
            ::guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn u1_result() -> U1 {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|u1_result"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn v1_arg(x: V1Param<'_>) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            x: V1Param<'a>,
        }
        let params = Params { x };
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|v1_arg"),
            ::guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn v1_result() -> V1Result {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|v1_result"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn bool_arg(x: bool) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: bool,
        }
        let params = Params { x };
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|bool_arg"),
            ::guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn bool_result() -> bool {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|bool_result"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn option_arg(
        a: Option<bool>,
        b: Option<()>,
        c: Option<u32>,
        d: Option<E1>,
        e: Option<f32>,
        f: Option<U1>,
        g: Option<Option<bool>>,
    ) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            a: Option<bool>,
            b: Option<()>,
            c: Option<u32>,
            d: Option<E1>,
            e: Option<f32>,
            f: Option<U1>,
            g: Option<Option<bool>>,
        }
        let params = Params {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
        };
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|option_arg"),
            ::guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn option_result() -> (
        Option<bool>,
        Option<()>,
        Option<u32>,
        Option<E1>,
        Option<f32>,
        Option<U1>,
        Option<Option<bool>>,
    ) {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|option_result"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn casts(
        a: Casts1,
        b: Casts2,
        c: Casts3,
        d: Casts4,
        e: Casts5,
        f: Casts6,
    ) -> (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6) {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            a: Casts1,
            b: Casts2,
            c: Casts3,
            d: Casts4,
            e: Casts5,
            f: Casts6,
        }
        let params = Params { a, b, c, d, e, f };
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|casts"),
            ::guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn result_arg(
        a: Result<(), ()>,
        b: Result<(), E1>,
        c: Result<E1, ()>,
        d: Result<(), ()>,
        e: Result<u32, V1Param<'_>>,
        f: Result<&str, &[u8]>,
    ) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            a: Result<(), ()>,
            b: Result<(), E1>,
            c: Result<E1, ()>,
            d: Result<(), ()>,
            e: Result<u32, V1Param<'a>>,
            f: Result<&'a str, &'a [u8]>,
        }
        let params = Params { a, b, c, d, e, f };
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|result_arg"),
            ::guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn result_result() -> (
        Result<(), ()>,
        Result<(), E1>,
        Result<E1, ()>,
        Result<(), ()>,
        Result<u32, V1Result>,
        Result<String, Vec<u8>>,
    ) {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|result_result"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn return_result_sugar() -> Result<i32, MyErrno> {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|return_result_sugar"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn return_result_sugar2() -> Result<(), MyErrno> {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|return_result_sugar2"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn return_result_sugar3() -> Result<MyErrno, MyErrno> {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|return_result_sugar3"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn return_result_sugar4() -> Result<(i32, u32), MyErrno> {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|return_result_sugar4"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn return_option_sugar() -> Option<i32> {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|return_option_sugar"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn return_option_sugar2() -> Option<MyErrno> {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|return_option_sugar2"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn result_simple() -> Result<u32, i32> {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|result_simple"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn is_clone_arg(a: IsCloneParam<'_>) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            a: IsCloneParam<'a>,
        }
        let params = Params { a };
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|is_clone_arg"),
            ::guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn is_clone_return() -> IsCloneResult {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|is_clone_return"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn return_named_option() -> Option<u8> {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|return_named_option"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn return_named_result() -> Result<u8, MyErrno> {
        let raw = invoke(
            ::guest_rust::wasm_bindgen::JsValue::from_str("plugin:imports|return_named_result"),
            ::guest_rust::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
