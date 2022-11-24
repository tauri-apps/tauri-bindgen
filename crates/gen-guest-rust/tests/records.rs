#[allow(clippy::all, unused)]
pub mod imports {

    #[cfg(debug_assertions)]
    static START: ::std::sync::Once = ::std::sync::Once::new();
    #[cfg(debug_assertions)]
    fn check_idl_version() {
        ::tauri_bindgen_guest_rust::wasm_bindgen_futures::spawn_local(async {
            if ::tauri_bindgen_guest_rust::invoke::<_, ()>(
                "plugin:records|e6872cf01241a6f3e4c4bedaa609dbeb",
                (),
            )
            .await
            .is_err()
            {
                ::tauri_bindgen_guest_rust::console_warn("The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: This is a debug assertion and IDL versions will not be checked in release builds.
        ");
            }
        });
    }

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
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: (char, u32),
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|tuple-arg", &params)
            .await
            .unwrap()
    }
    pub async fn tuple_result() -> (char, u32) {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:records|tuple-result", ())
            .await
            .unwrap()
    }
    pub async fn empty_arg(x: Empty) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: Empty,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|empty-arg", &params)
            .await
            .unwrap()
    }
    pub async fn empty_result() -> Empty {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:records|empty-result", ())
            .await
            .unwrap()
    }
    pub async fn scalar_arg(x: Scalars) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: Scalars,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|scalar-arg", &params)
            .await
            .unwrap()
    }
    pub async fn scalar_result() -> Scalars {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:records|scalar-result", ())
            .await
            .unwrap()
    }
    pub async fn flags_arg(x: ReallyFlags) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: ReallyFlags,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|flags-arg", &params)
            .await
            .unwrap()
    }
    pub async fn flags_result() -> ReallyFlags {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:records|flags-result", ())
            .await
            .unwrap()
    }
    pub async fn aggregate_arg(x: AggregatesParam<'_>) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            x: AggregatesParam<'a>,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|aggregate-arg", &params)
            .await
            .unwrap()
    }
    pub async fn aggregate_result() -> AggregatesResult {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:records|aggregate-result", ())
            .await
            .unwrap()
    }
    pub async fn typedef_inout(e: TupleTypedef2) -> i32 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            e: TupleTypedef2,
        }
        let params = Params { e };
        ::tauri_bindgen_guest_rust::invoke("plugin:records|typedef-inout", &params)
            .await
            .unwrap()
    }
}
