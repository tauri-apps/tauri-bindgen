pub mod lists {
    use ::tauri_bindgen_guest_rust::tauri_bindgen_abi;
    use ::tauri_bindgen_guest_rust::bitflags;
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct OtherRecord<'a> {
        a1: u32,
        a2: u64,
        a3: i32,
        a4: i64,
        b: &'a str,
        c: &'a [u8],
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct SomeRecord<'a> {
        x: &'a str,
        y: OtherRecord<'a>,
        z: &'a [OtherRecord<'a>],
        c1: u32,
        c2: u64,
        c3: i32,
        c4: i64,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum OtherVariant<'a> {
        A,
        B(u32),
        C(&'a str),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum SomeVariant<'a> {
        A(&'a str),
        B,
        C(u32),
        D(&'a [OtherVariant<'a>]),
    }
    pub type LoadStoreAllSizes<'a> = &'a [(
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
    pub async fn record_list(x: &'_ [SomeRecord<'_>]) -> Vec<OtherRecord> {
        todo!()
    }
    pub async fn record_list_reverse(x: &'_ [OtherRecord<'_>]) -> Vec<SomeRecord> {
        todo!()
    }
    pub async fn variant_list(x: &'_ [SomeVariant<'_>]) -> Vec<OtherVariant> {
        todo!()
    }
    pub async fn load_store_everything(a: LoadStoreAllSizes<'_>) -> LoadStoreAllSizes {
        todo!()
    }
}
