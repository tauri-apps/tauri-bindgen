#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod records {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct Empty {}
    /**A record containing two scalar fields
that both have the same type*/
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct Scalars {
        ///The first field, named a
        pub a: u32,
        ///The second field, named b
        pub b: u32,
    }
    /**A record that is really just flags
All of the fields are bool*/
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
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
    #[derive(serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct AggregatesParam<'a> {
        pub a: Scalars,
        pub b: u32,
        pub c: Empty,
        pub d: &'a str,
        pub e: ReallyFlags,
    }
    #[derive(serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct AggregatesResult {
        pub a: Scalars,
        pub b: u32,
        pub c: Empty,
        pub d: String,
        pub e: ReallyFlags,
    }
    pub type IntTypedef = i32;
    pub type TupleTypedef2 = (IntTypedef,);
    pub async fn tuple_arg(x: (char, u32)) {
        ::tauri_bindgen_guest_rust::invoke("records", "tuple_arg", &(x)).await.unwrap()
    }
    pub async fn tuple_result() -> (char, u32) {
        ::tauri_bindgen_guest_rust::invoke("records", "tuple_result", &()).await.unwrap()
    }
    pub async fn empty_arg(x: Empty) {
        ::tauri_bindgen_guest_rust::invoke("records", "empty_arg", &(x)).await.unwrap()
    }
    pub async fn empty_result() -> Empty {
        ::tauri_bindgen_guest_rust::invoke("records", "empty_result", &()).await.unwrap()
    }
    pub async fn scalar_arg(x: Scalars) {
        ::tauri_bindgen_guest_rust::invoke("records", "scalar_arg", &(x)).await.unwrap()
    }
    pub async fn scalar_result() -> Scalars {
        ::tauri_bindgen_guest_rust::invoke("records", "scalar_result", &())
            .await
            .unwrap()
    }
    pub async fn flags_arg(x: ReallyFlags) {
        ::tauri_bindgen_guest_rust::invoke("records", "flags_arg", &(x)).await.unwrap()
    }
    pub async fn flags_result() -> ReallyFlags {
        ::tauri_bindgen_guest_rust::invoke("records", "flags_result", &()).await.unwrap()
    }
    pub async fn aggregate_arg(x: AggregatesParam<'_>) {
        ::tauri_bindgen_guest_rust::invoke("records", "aggregate_arg", &(x))
            .await
            .unwrap()
    }
    pub async fn aggregate_result() -> AggregatesResult {
        ::tauri_bindgen_guest_rust::invoke("records", "aggregate_result", &())
            .await
            .unwrap()
    }
    pub async fn typedef_inout(e: TupleTypedef2) -> i32 {
        ::tauri_bindgen_guest_rust::invoke("records", "typedef_inout", &(e))
            .await
            .unwrap()
    }
}
