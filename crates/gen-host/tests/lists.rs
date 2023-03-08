pub mod lists {
    use ::tauri_bindgen_host::bitflags;
    use ::tauri_bindgen_host::tauri_bindgen_abi;
    #[derive(tauri_bindgen_abi::Writable)]
    pub struct OtherRecord {
        a1: u32,
        a2: u64,
        a3: i32,
        a4: i64,
        b: String,
        c: Vec<u8>,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub struct SomeRecord {
        x: String,
        y: OtherRecord,
        z: Vec<OtherRecord>,
        c1: u32,
        c2: u64,
        c3: i32,
        c4: i64,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum OtherVariant {
        A,
        B(u32),
        C(String),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum SomeVariant {
        A(String),
        B,
        C(u32),
        D(Vec<OtherVariant>),
    }
    pub type LoadStoreAllSizes =
        Vec<(String, u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, char)>;
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
        fn load_store_everything(&mut self, a: LoadStoreAllSizes) -> LoadStoreAllSizes<'_>;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Lists,
    {
        router.func_wrap(
            "lists",
            "list_u8_param",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: Vec<u8>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.list_u8_param(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "list_u16_param",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: Vec<u16>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.list_u16_param(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "list_u32_param",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: Vec<u32>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.list_u32_param(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "list_u64_param",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: Vec<u64>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.list_u64_param(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "list_s8_param",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: Vec<i8>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.list_s8_param(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "list_s16_param",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: Vec<i16>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.list_s16_param(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "list_s32_param",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: Vec<i32>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.list_s32_param(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "list_s64_param",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: Vec<i64>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.list_s64_param(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "list_float32_param",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: Vec<f32>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.list_float32_param(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "list_float64_param",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: Vec<f64>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.list_float64_param(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "list_u8_ret",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> &'_ [u8] {
                let cx = get_cx(cx.data_mut());
                cx.list_u8_ret()
            },
        )?;
        router.func_wrap(
            "lists",
            "list_u16_ret",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> &'_ [u16] {
                let cx = get_cx(cx.data_mut());
                cx.list_u16_ret()
            },
        )?;
        router.func_wrap(
            "lists",
            "list_u32_ret",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> &'_ [u32] {
                let cx = get_cx(cx.data_mut());
                cx.list_u32_ret()
            },
        )?;
        router.func_wrap(
            "lists",
            "list_u64_ret",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> &'_ [u64] {
                let cx = get_cx(cx.data_mut());
                cx.list_u64_ret()
            },
        )?;
        router.func_wrap(
            "lists",
            "list_s8_ret",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> &'_ [i8] {
                let cx = get_cx(cx.data_mut());
                cx.list_s8_ret()
            },
        )?;
        router.func_wrap(
            "lists",
            "list_s16_ret",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> &'_ [i16] {
                let cx = get_cx(cx.data_mut());
                cx.list_s16_ret()
            },
        )?;
        router.func_wrap(
            "lists",
            "list_s32_ret",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> &'_ [i32] {
                let cx = get_cx(cx.data_mut());
                cx.list_s32_ret()
            },
        )?;
        router.func_wrap(
            "lists",
            "list_s64_ret",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> &'_ [i64] {
                let cx = get_cx(cx.data_mut());
                cx.list_s64_ret()
            },
        )?;
        router.func_wrap(
            "lists",
            "list_float32_ret",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> &'_ [f32] {
                let cx = get_cx(cx.data_mut());
                cx.list_float32_ret()
            },
        )?;
        router.func_wrap(
            "lists",
            "list_float64_ret",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> &'_ [f64] {
                let cx = get_cx(cx.data_mut());
                cx.list_float64_ret()
            },
        )?;
        router.func_wrap(
            "lists",
            "tuple_list",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                  x: Vec<(u8, i8)>|
                  -> &'_ [(i64, u32)] {
                let cx = get_cx(cx.data_mut());
                cx.tuple_list(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "string_list_arg",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, a: Vec<String>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.string_list_arg(a)
            },
        )?;
        router.func_wrap(
            "lists",
            "string_list_ret",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> &'_ [&'_ str] {
                let cx = get_cx(cx.data_mut());
                cx.string_list_ret()
            },
        )?;
        router.func_wrap(
            "lists",
            "tuple_string_list",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                  x: Vec<(u8, String)>|
                  -> &'_ [(&'_ str, u8)] {
                let cx = get_cx(cx.data_mut());
                cx.tuple_string_list(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "string_list",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                  x: Vec<String>|
                  -> &'_ [&'_ str] {
                let cx = get_cx(cx.data_mut());
                cx.string_list(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "record_list",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                  x: Vec<SomeRecord>|
                  -> &'_ [OtherRecord<'_>] {
                let cx = get_cx(cx.data_mut());
                cx.record_list(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "record_list_reverse",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                  x: Vec<OtherRecord>|
                  -> &'_ [SomeRecord<'_>] {
                let cx = get_cx(cx.data_mut());
                cx.record_list_reverse(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "variant_list",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                  x: Vec<SomeVariant>|
                  -> &'_ [OtherVariant<'_>] {
                let cx = get_cx(cx.data_mut());
                cx.variant_list(x)
            },
        )?;
        router.func_wrap(
            "lists",
            "load_store_everything",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                  a: LoadStoreAllSizes|
                  -> LoadStoreAllSizes<'_> {
                let cx = get_cx(cx.data_mut());
                cx.load_store_everything(a)
            },
        )?;
        Ok(())
    }
}
