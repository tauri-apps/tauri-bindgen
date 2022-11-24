#[allow(clippy::all, unused)]
pub mod imports {
    const IDL_HASH: &str = "e6872cf01241a6f3";
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
        struct Params<'a> {
            idl_hash: &'a str,
            x: (char, u32),
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|tuple-arg", &params).await
    }
    pub async fn tuple_result() -> (char, u32) {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|tuple-result", ()).await
    }
    pub async fn empty_arg(x: Empty) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: Empty,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|empty-arg", &params).await
    }
    pub async fn empty_result() -> Empty {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|empty-result", ()).await
    }
    pub async fn scalar_arg(x: Scalars) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: Scalars,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|scalar-arg", &params).await
    }
    pub async fn scalar_result() -> Scalars {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|scalar-result", ()).await
    }
    pub async fn flags_arg(x: ReallyFlags) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: ReallyFlags,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|flags-arg", &params).await
    }
    pub async fn flags_result() -> ReallyFlags {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|flags-result", ()).await
    }
    pub async fn aggregate_arg(x: AggregatesParam<'_>) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: AggregatesParam<'a>,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|aggregate-arg", &params).await
    }
    pub async fn aggregate_result() -> AggregatesResult {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|aggregate-result", ()).await
    }
    pub async fn typedef_inout(e: TupleTypedef2) -> i32 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            e: TupleTypedef2,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            e,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|typedef-inout", &params).await
    }
}
