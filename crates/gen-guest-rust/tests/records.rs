#[allow(clippy::all, unused)]
pub mod imports {
    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Empty {}
    /// A record containing two scalar fields
    /// that both have the same type
    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
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
    #[derive(Debug, Copy, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
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
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AggregatesParam<'a> {
        pub a: Scalars,
        pub b: u32,
        pub c: Empty,
        pub d: &'a str,
        pub e: ReallyFlags,
    }
    #[derive(Debug, Clone, PartialEq, ::serde::Deserialize)]
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
        ::tauri_bindgen_guest_rust::invoke("plugin:e6872cf01241a6f3|tuple-arg", &params)
            .await
            .unwrap()
    }
    pub async fn tuple_result() -> (char, u32) {
        ::tauri_bindgen_guest_rust::invoke("plugin:e6872cf01241a6f3|tuple-result", ())
            .await
            .unwrap()
    }
    pub async fn empty_arg(x: Empty) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: Empty,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:e6872cf01241a6f3|empty-arg", &params)
            .await
            .unwrap()
    }
    pub async fn empty_result() -> Empty {
        ::tauri_bindgen_guest_rust::invoke("plugin:e6872cf01241a6f3|empty-result", ())
            .await
            .unwrap()
    }
    pub async fn scalar_arg(x: Scalars) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: Scalars,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:e6872cf01241a6f3|scalar-arg", &params)
            .await
            .unwrap()
    }
    pub async fn scalar_result() -> Scalars {
        ::tauri_bindgen_guest_rust::invoke("plugin:e6872cf01241a6f3|scalar-result", ())
            .await
            .unwrap()
    }
    pub async fn flags_arg(x: ReallyFlags) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: ReallyFlags,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:e6872cf01241a6f3|flags-arg", &params)
            .await
            .unwrap()
    }
    pub async fn flags_result() -> ReallyFlags {
        ::tauri_bindgen_guest_rust::invoke("plugin:e6872cf01241a6f3|flags-result", ())
            .await
            .unwrap()
    }
    pub async fn aggregate_arg(x: AggregatesParam<'_>) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            x: AggregatesParam<'a>,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:e6872cf01241a6f3|aggregate-arg", &params)
            .await
            .unwrap()
    }
    pub async fn aggregate_result() -> AggregatesResult {
        ::tauri_bindgen_guest_rust::invoke("plugin:e6872cf01241a6f3|aggregate-result", ())
            .await
            .unwrap()
    }
    pub async fn typedef_inout(e: TupleTypedef2) -> i32 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            e: TupleTypedef2,
        }
        let params = Params { e };
        ::tauri_bindgen_guest_rust::invoke("plugin:e6872cf01241a6f3|typedef-inout", &params)
            .await
            .unwrap()
    }
}
