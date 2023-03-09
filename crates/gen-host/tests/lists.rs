#[allow(unused_imports, unused_variables)]
pub mod lists {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct OtherRecord<'a> {
        a1: u32,
        a2: u64,
        a3: i32,
        a4: i64,
        b: &'a str,
        c: &'a [u8],
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct SomeRecord<'a> {
        x: &'a str,
        #[serde(borrow)]
        y: OtherRecord<'a>,
        #[serde(borrow)]
        z: Vec<OtherRecord<'a>>,
        c1: u32,
        c2: u64,
        c3: i32,
        c4: i64,
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    pub enum OtherVariant<'a> {
        A,
        B(u32),
        C(&'a str),
    }
    #[derive(serde::Deserialize)]
    pub enum SomeVariant<'a> {
        A(&'a str),
        B,
        C(u32),
        D(Vec<OtherVariant<'a>>),
    }
    pub type LoadStoreAllSizes<'a> = Vec<
        (&'a str, u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, char),
    >;
    pub trait Lists: Sized {
        fn list_u8_param(&mut self, x: Vec<u8>) -> ();
        fn list_u16_param(&mut self, x: Vec<u16>) -> ();
        fn list_u32_param(&mut self, x: Vec<u32>) -> ();
        fn list_u64_param(&mut self, x: Vec<u64>) -> ();
        fn list_s8_param(&mut self, x: Vec<i8>) -> ();
        fn list_s16_param(&mut self, x: Vec<i16>) -> ();
        fn list_s32_param(&mut self, x: Vec<i32>) -> ();
        fn list_s64_param(&mut self, x: Vec<i64>) -> ();
        fn list_float32_param(&mut self, x: Vec<f32>) -> ();
        fn list_float64_param(&mut self, x: Vec<f64>) -> ();
        fn list_u8_ret(&mut self) -> &'_ [u8];
        fn list_u16_ret(&mut self) -> &'_ [u16];
        fn list_u32_ret(&mut self) -> &'_ [u32];
        fn list_u64_ret(&mut self) -> &'_ [u64];
        fn list_s8_ret(&mut self) -> &'_ [i8];
        fn list_s16_ret(&mut self) -> &'_ [i16];
        fn list_s32_ret(&mut self) -> &'_ [i32];
        fn list_s64_ret(&mut self) -> &'_ [i64];
        fn list_float32_ret(&mut self) -> &'_ [f32];
        fn list_float64_ret(&mut self) -> &'_ [f64];
        fn tuple_list(&mut self, x: Vec<(u8, i8)>) -> &'_ [(i64, u32)];
        fn string_list_arg(&mut self, a: Vec<String>) -> ();
        fn string_list_ret(&mut self) -> &'_ [&'_ str];
        fn tuple_string_list(&mut self, x: Vec<(u8, String)>) -> &'_ [(&'_ str, u8)];
        fn string_list(&mut self, x: Vec<String>) -> &'_ [&'_ str];
        fn record_list(&mut self, x: Vec<SomeRecord>) -> &'_ [OtherRecord<'_>];
        fn record_list_reverse(&mut self, x: Vec<OtherRecord>) -> &'_ [SomeRecord<'_>];
        fn variant_list(&mut self, x: Vec<SomeVariant>) -> &'_ [OtherVariant<'_>];
        fn load_store_everything(
            &mut self,
            a: LoadStoreAllSizes,
        ) -> LoadStoreAllSizes<'_>;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Lists,
    {
        Ok(())
    }
}
