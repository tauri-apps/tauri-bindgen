#[allow(unused_imports, unused_variables)]
pub mod lists {
    use ::tauri_bindgen_guest_rust::bitflags;
    use ::tauri_bindgen_guest_rust::serde;
    #[derive(serde::Serialize)]
    pub struct OtherRecordParam<'a> {
        a1: u32,
        a2: u64,
        a3: i32,
        a4: i64,
        b: &'a str,
        c: &'a [u8],
    }
    #[derive(serde::Deserialize)]
    pub struct OtherRecordResult {
        a1: u32,
        a2: u64,
        a3: i32,
        a4: i64,
        b: String,
        c: Vec<u8>,
    }
    #[derive(serde::Serialize)]
    pub struct SomeRecordParam<'a> {
        x: &'a str,
        #[serde(borrow)]
        y: OtherRecordParam<'a>,
        #[serde(borrow)]
        z: &'a [OtherRecordParam<'a>],
        c1: u32,
        c2: u64,
        c3: i32,
        c4: i64,
    }
    #[derive(serde::Deserialize)]
    pub struct SomeRecordResult {
        x: String,
        y: OtherRecordResult,
        z: Vec<OtherRecordResult>,
        c1: u32,
        c2: u64,
        c3: i32,
        c4: i64,
    }
    #[derive(serde::Serialize)]
    pub enum OtherVariantParam<'a> {
        A,
        B(u32),
        C(&'a str),
    }
    #[derive(serde::Deserialize)]
    pub enum OtherVariantResult {
        A,
        B(u32),
        C(String),
    }
    #[derive(serde::Serialize)]
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
    pub type LoadStoreAllSizesResult =
        Vec<(String, u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, char)>;
    pub async fn list_u8_param(x: &'_ [u8]) -> () {
        todo!()
    }
    pub async fn list_u16_param(x: &'_ [u16]) -> () {
        todo!()
    }
    pub async fn list_u32_param(x: &'_ [u32]) -> () {
        todo!()
    }
    pub async fn list_u64_param(x: &'_ [u64]) -> () {
        todo!()
    }
    pub async fn list_s8_param(x: &'_ [i8]) -> () {
        todo!()
    }
    pub async fn list_s16_param(x: &'_ [i16]) -> () {
        todo!()
    }
    pub async fn list_s32_param(x: &'_ [i32]) -> () {
        todo!()
    }
    pub async fn list_s64_param(x: &'_ [i64]) -> () {
        todo!()
    }
    pub async fn list_float32_param(x: &'_ [f32]) -> () {
        todo!()
    }
    pub async fn list_float64_param(x: &'_ [f64]) -> () {
        todo!()
    }
    pub async fn list_u8_ret() -> Vec<u8> {
        todo!()
    }
    pub async fn list_u16_ret() -> Vec<u16> {
        todo!()
    }
    pub async fn list_u32_ret() -> Vec<u32> {
        todo!()
    }
    pub async fn list_u64_ret() -> Vec<u64> {
        todo!()
    }
    pub async fn list_s8_ret() -> Vec<i8> {
        todo!()
    }
    pub async fn list_s16_ret() -> Vec<i16> {
        todo!()
    }
    pub async fn list_s32_ret() -> Vec<i32> {
        todo!()
    }
    pub async fn list_s64_ret() -> Vec<i64> {
        todo!()
    }
    pub async fn list_float32_ret() -> Vec<f32> {
        todo!()
    }
    pub async fn list_float64_ret() -> Vec<f64> {
        todo!()
    }
    pub async fn tuple_list(x: &'_ [(u8, i8)]) -> Vec<(i64, u32)> {
        todo!()
    }
    pub async fn string_list_arg(a: &'_ [&'_ str]) -> () {
        todo!()
    }
    pub async fn string_list_ret() -> Vec<String> {
        todo!()
    }
    pub async fn tuple_string_list(x: &'_ [(u8, &'_ str)]) -> Vec<(String, u8)> {
        todo!()
    }
    pub async fn string_list(x: &'_ [&'_ str]) -> Vec<String> {
        todo!()
    }
    pub async fn record_list(x: &'_ [SomeRecordParam<'_>]) -> Vec<OtherRecordResult> {
        todo!()
    }
    pub async fn record_list_reverse(x: &'_ [OtherRecordParam<'_>]) -> Vec<SomeRecordResult> {
        todo!()
    }
    pub async fn variant_list(x: &'_ [SomeVariant<'_>]) -> Vec<OtherVariantResult> {
        todo!()
    }
    pub async fn load_store_everything(a: LoadStoreAllSizesParam<'_>) -> LoadStoreAllSizesResult {
        todo!()
    }
}
