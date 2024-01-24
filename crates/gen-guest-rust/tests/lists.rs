#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod lists {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    #[derive(serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct OtherRecordParam<'a> {
        pub a1: u32,
        pub a2: u64,
        pub a3: i32,
        pub a4: i64,
        pub b: &'a str,
        pub c: &'a [u8],
    }
    #[derive(serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct OtherRecordResult {
        pub a1: u32,
        pub a2: u64,
        pub a3: i32,
        pub a4: i64,
        pub b: String,
        pub c: Vec<u8>,
    }
    #[derive(serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct SomeRecordParam<'a> {
        pub x: &'a str,
        #[serde(borrow)]
        pub y: OtherRecordParam<'a>,
        #[serde(borrow)]
        pub z: &'a [OtherRecordParam<'a>],
        pub c1: u32,
        pub c2: u64,
        pub c3: i32,
        pub c4: i64,
    }
    #[derive(serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct SomeRecordResult {
        pub x: String,
        pub y: OtherRecordResult,
        pub z: Vec<OtherRecordResult>,
        pub c1: u32,
        pub c2: u64,
        pub c3: i32,
        pub c4: i64,
    }
    #[derive(serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum OtherVariantParam<'a> {
        A,
        B(u32),
        C(&'a str),
    }
    #[derive(serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum OtherVariantResult {
        A,
        B(u32),
        C(String),
    }
    #[derive(serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum SomeVariant<'a> {
        A(&'a str),
        B,
        C(u32),
        D(&'a [OtherVariantParam<'a>]),
    }
    pub type LoadStoreAllSizesParam<'a> = &'a [(
        &'a str,
        u8,
        i8,
        u16,
        i16,
        u32,
        i32,
        u64,
        i64,
        f32,
        f64,
        char,
    )];
    pub type LoadStoreAllSizesResult = Vec<
        (String, u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, char),
    >;
    pub async fn list_u8_param(x: &'_ [u8]) {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_u8_param", &(x)).await.unwrap()
    }
    pub async fn list_u16_param(x: &'_ [u16]) {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_u16_param", &(x))
            .await
            .unwrap()
    }
    pub async fn list_u32_param(x: &'_ [u32]) {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_u32_param", &(x))
            .await
            .unwrap()
    }
    pub async fn list_u64_param(x: &'_ [u64]) {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_u64_param", &(x))
            .await
            .unwrap()
    }
    pub async fn list_u128_param(x: &'_ [u128]) {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_u128_param", &(x))
            .await
            .unwrap()
    }
    pub async fn list_s8_param(x: &'_ [i8]) {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_s8_param", &(x)).await.unwrap()
    }
    pub async fn list_s16_param(x: &'_ [i16]) {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_s16_param", &(x))
            .await
            .unwrap()
    }
    pub async fn list_s32_param(x: &'_ [i32]) {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_s32_param", &(x))
            .await
            .unwrap()
    }
    pub async fn list_s64_param(x: &'_ [i64]) {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_s64_param", &(x))
            .await
            .unwrap()
    }
    pub async fn list_s128_param(x: &'_ [i128]) {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_s128_param", &(x))
            .await
            .unwrap()
    }
    pub async fn list_float32_param(x: &'_ [f32]) {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_float32_param", &(x))
            .await
            .unwrap()
    }
    pub async fn list_float64_param(x: &'_ [f64]) {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_float64_param", &(x))
            .await
            .unwrap()
    }
    pub async fn list_u8_ret() -> Vec<u8> {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_u8_ret", &()).await.unwrap()
    }
    pub async fn list_u16_ret() -> Vec<u16> {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_u16_ret", &()).await.unwrap()
    }
    pub async fn list_u32_ret() -> Vec<u32> {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_u32_ret", &()).await.unwrap()
    }
    pub async fn list_u64_ret() -> Vec<u64> {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_u64_ret", &()).await.unwrap()
    }
    pub async fn list_u128_ret() -> Vec<u128> {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_u128_ret", &()).await.unwrap()
    }
    pub async fn list_s8_ret() -> Vec<i8> {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_s8_ret", &()).await.unwrap()
    }
    pub async fn list_s16_ret() -> Vec<i16> {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_s16_ret", &()).await.unwrap()
    }
    pub async fn list_s32_ret() -> Vec<i32> {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_s32_ret", &()).await.unwrap()
    }
    pub async fn list_s64_ret() -> Vec<i64> {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_s64_ret", &()).await.unwrap()
    }
    pub async fn list_s128_ret() -> Vec<i128> {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_s128_ret", &()).await.unwrap()
    }
    pub async fn list_float32_ret() -> Vec<f32> {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_float32_ret", &())
            .await
            .unwrap()
    }
    pub async fn list_float64_ret() -> Vec<f64> {
        ::tauri_bindgen_guest_rust::invoke("lists", "list_float64_ret", &())
            .await
            .unwrap()
    }
    pub async fn tuple_list(x: &'_ [(u8, i8)]) -> Vec<(i64, u32)> {
        ::tauri_bindgen_guest_rust::invoke("lists", "tuple_list", &(x)).await.unwrap()
    }
    pub async fn string_list_arg(a: &'_ [&'_ str]) {
        ::tauri_bindgen_guest_rust::invoke("lists", "string_list_arg", &(a))
            .await
            .unwrap()
    }
    pub async fn string_list_ret() -> Vec<String> {
        ::tauri_bindgen_guest_rust::invoke("lists", "string_list_ret", &())
            .await
            .unwrap()
    }
    pub async fn tuple_string_list(x: &'_ [(u8, &'_ str)]) -> Vec<(String, u8)> {
        ::tauri_bindgen_guest_rust::invoke("lists", "tuple_string_list", &(x))
            .await
            .unwrap()
    }
    pub async fn string_list(x: &'_ [&'_ str]) -> Vec<String> {
        ::tauri_bindgen_guest_rust::invoke("lists", "string_list", &(x)).await.unwrap()
    }
    pub async fn record_list(x: &'_ [SomeRecordParam<'_>]) -> Vec<OtherRecordResult> {
        ::tauri_bindgen_guest_rust::invoke("lists", "record_list", &(x)).await.unwrap()
    }
    pub async fn record_list_reverse(
        x: &'_ [OtherRecordParam<'_>],
    ) -> Vec<SomeRecordResult> {
        ::tauri_bindgen_guest_rust::invoke("lists", "record_list_reverse", &(x))
            .await
            .unwrap()
    }
    pub async fn variant_list(x: &'_ [SomeVariant<'_>]) -> Vec<OtherVariantResult> {
        ::tauri_bindgen_guest_rust::invoke("lists", "variant_list", &(x)).await.unwrap()
    }
    pub async fn load_store_everything(
        a: LoadStoreAllSizesParam<'_>,
    ) -> LoadStoreAllSizesResult {
        ::tauri_bindgen_guest_rust::invoke("lists", "load_store_everything", &(a))
            .await
            .unwrap()
    }
}
