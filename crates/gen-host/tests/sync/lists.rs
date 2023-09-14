#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod lists {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct OtherRecord {
        pub a1: u32,
        pub a2: u64,
        pub a3: i32,
        pub a4: i64,
        pub b: String,
        pub c: Vec<u8>,
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct SomeRecord {
        pub x: String,
        pub y: OtherRecord,
        pub z: Vec<OtherRecord>,
        pub c1: u32,
        pub c2: u64,
        pub c3: i32,
        pub c4: i64,
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum OtherVariant {
        A,
        B(u32),
        C(String),
    }
    #[derive(serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum SomeVariant {
        A(String),
        B,
        C(u32),
        D(Vec<OtherVariant>),
    }
    pub type LoadStoreAllSizes = Vec<
        (String, u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, char),
    >;
    pub trait Lists: Sized {
        fn list_u8_param(&self, x: Vec<u8>);
        fn list_u16_param(&self, x: Vec<u16>);
        fn list_u32_param(&self, x: Vec<u32>);
        fn list_u64_param(&self, x: Vec<u64>);
        fn list_u128_param(&self, x: Vec<u128>);
        fn list_s8_param(&self, x: Vec<i8>);
        fn list_s16_param(&self, x: Vec<i16>);
        fn list_s32_param(&self, x: Vec<i32>);
        fn list_s64_param(&self, x: Vec<i64>);
        fn list_s128_param(&self, x: Vec<i128>);
        fn list_float32_param(&self, x: Vec<f32>);
        fn list_float64_param(&self, x: Vec<f64>);
        fn list_u8_ret(&self) -> Vec<u8>;
        fn list_u16_ret(&self) -> Vec<u16>;
        fn list_u32_ret(&self) -> Vec<u32>;
        fn list_u64_ret(&self) -> Vec<u64>;
        fn list_u128_ret(&self) -> Vec<u128>;
        fn list_s8_ret(&self) -> Vec<i8>;
        fn list_s16_ret(&self) -> Vec<i16>;
        fn list_s32_ret(&self) -> Vec<i32>;
        fn list_s64_ret(&self) -> Vec<i64>;
        fn list_s128_ret(&self) -> Vec<i128>;
        fn list_float32_ret(&self) -> Vec<f32>;
        fn list_float64_ret(&self) -> Vec<f64>;
        fn tuple_list(&self, x: Vec<(u8, i8)>) -> Vec<(i64, u32)>;
        fn string_list_arg(&self, a: Vec<String>);
        fn string_list_ret(&self) -> Vec<String>;
        fn tuple_string_list(&self, x: Vec<(u8, String)>) -> Vec<(String, u8)>;
        fn string_list(&self, x: Vec<String>) -> Vec<String>;
        fn record_list(&self, x: Vec<SomeRecord>) -> Vec<OtherRecord>;
        fn record_list_reverse(&self, x: Vec<OtherRecord>) -> Vec<SomeRecord>;
        fn variant_list(&self, x: Vec<SomeVariant>) -> Vec<OtherVariant>;
        fn load_store_everything(&self, a: LoadStoreAllSizes) -> LoadStoreAllSizes;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: Lists + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_u8_param",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Vec<u8>| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_u8_param(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_u16_param",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Vec<u16>| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_u16_param(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_u32_param",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Vec<u32>| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_u32_param(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_u64_param",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Vec<u64>| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_u64_param(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_u128_param",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: Vec<u128>|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_u128_param(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_s8_param",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Vec<i8>| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_s8_param(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_s16_param",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Vec<i16>| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_s16_param(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_s32_param",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Vec<i32>| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_s32_param(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_s64_param",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Vec<i64>| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_s64_param(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_s128_param",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: Vec<i128>|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_s128_param(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_float32_param",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Vec<f32>| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_float32_param(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_float64_param",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Vec<f64>| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_float64_param(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_u8_ret",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_u8_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_u16_ret",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_u16_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_u32_ret",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_u32_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_u64_ret",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_u64_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_u128_ret",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_u128_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_s8_ret",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_s8_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_s16_ret",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_s16_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_s32_ret",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_s32_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_s64_ret",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_s64_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_s128_ret",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_s128_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_float32_ret",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_float32_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "list_float64_ret",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.list_float64_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "tuple_list",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: Vec<(u8, i8)>|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.tuple_list(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "string_list_arg",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: Vec<String>|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.string_list_arg(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "string_list_ret",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.string_list_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "tuple_string_list",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: Vec<(u8, String)>|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.tuple_string_list(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "string_list",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: Vec<String>|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.string_list(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "record_list",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: Vec<SomeRecord>|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.record_list(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "record_list_reverse",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: Vec<OtherRecord>|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.record_list_reverse(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "variant_list",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: Vec<SomeVariant>|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.variant_list(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "lists",
                "load_store_everything",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: LoadStoreAllSizes|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.load_store_everything(p))
                },
            )?;
        Ok(())
    }
}
