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
    #[derive(Debug, Copy, Clone, ::serde::Serialize, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Empty {}
    /// A record containing two scalar fields
    /// that both have the same type
    #[repr(C)]
    #[derive(Debug, Copy, Clone, ::serde::Serialize, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Scalars {
        /// The first field, named a
        pub a: u32,
        /// The second field, named b
        pub b: u32,
    }
    /// A record that is really just flags
    /// All of the fields are bool
    #[repr(C)]
    #[derive(Debug, Copy, Clone, ::serde::Serialize, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ReallyFlags {
        pub a: bool,
        pub b: bool,
        pub c: bool,
        pub d: bool,
        pub e: bool,
        pub f: bool,
        pub g: bool,
        pub h: bool,
        pub i: bool,
    }
    #[derive(Debug, Clone, ::serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AggregatesParam<'a> {
        pub a: Scalars,
        pub b: u32,
        pub c: Empty,
        pub d: &'a str,
        pub e: ReallyFlags,
    }
    #[derive(Debug, Clone, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AggregatesResult {
        pub a: Scalars,
        pub b: u32,
        pub c: Empty,
        pub d: String,
        pub e: ReallyFlags,
    }
    pub type IntTypedef = i32;
    pub type TupleTypedef2 = (IntTypedef,);
    pub async fn tuple_arg(x: (char, u32)) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: (char, u32),
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|tuple_arg"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn tuple_result() -> (char, u32) {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|tuple_result"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn empty_arg(x: Empty) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: Empty,
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|empty_arg"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn empty_result() -> Empty {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|empty_result"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn scalar_arg(x: Scalars) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: Scalars,
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|scalar_arg"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn scalar_result() -> Scalars {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|scalar_result"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn flags_arg(x: ReallyFlags) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: ReallyFlags,
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|flags_arg"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn flags_result() -> ReallyFlags {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|flags_result"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn aggregate_arg(x: AggregatesParam<'_>) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            x: AggregatesParam<'a>,
        }
        let params = Params { x };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|aggregate_arg"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn aggregate_result() -> AggregatesResult {
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|aggregate_result"),
            ::wasm_bindgen::JsValue::NULL,
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn typedef_inout(e: TupleTypedef2) -> i32 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            e: TupleTypedef2,
        }
        let params = Params { e };
        let raw = invoke(
            ::wasm_bindgen::JsValue::from_str("plugin:imports|typedef_inout"),
            ::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
