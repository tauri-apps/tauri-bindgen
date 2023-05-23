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
        fn list_u8_param(&mut self, x: Vec<u8>);
        fn list_u16_param(&mut self, x: Vec<u16>);
        fn list_u32_param(&mut self, x: Vec<u32>);
        fn list_u64_param(&mut self, x: Vec<u64>);
        fn list_s8_param(&mut self, x: Vec<i8>);
        fn list_s16_param(&mut self, x: Vec<i16>);
        fn list_s32_param(&mut self, x: Vec<i32>);
        fn list_s64_param(&mut self, x: Vec<i64>);
        fn list_float32_param(&mut self, x: Vec<f32>);
        fn list_float64_param(&mut self, x: Vec<f64>);
        fn list_u8_ret(&mut self) -> Vec<u8>;
        fn list_u16_ret(&mut self) -> Vec<u16>;
        fn list_u32_ret(&mut self) -> Vec<u32>;
        fn list_u64_ret(&mut self) -> Vec<u64>;
        fn list_s8_ret(&mut self) -> Vec<i8>;
        fn list_s16_ret(&mut self) -> Vec<i16>;
        fn list_s32_ret(&mut self) -> Vec<i32>;
        fn list_s64_ret(&mut self) -> Vec<i64>;
        fn list_float32_ret(&mut self) -> Vec<f32>;
        fn list_float64_ret(&mut self) -> Vec<f64>;
        fn tuple_list(&mut self, x: Vec<(u8, i8)>) -> Vec<(i64, u32)>;
        fn string_list_arg(&mut self, a: Vec<String>);
        fn string_list_ret(&mut self) -> Vec<String>;
        fn tuple_string_list(&mut self, x: Vec<(u8, String)>) -> Vec<(String, u8)>;
        fn string_list(&mut self, x: Vec<String>) -> Vec<String>;
        fn record_list(&mut self, x: Vec<SomeRecord>) -> Vec<OtherRecord>;
        fn record_list_reverse(&mut self, x: Vec<OtherRecord>) -> Vec<SomeRecord>;
        fn variant_list(&mut self, x: Vec<SomeVariant>) -> Vec<OtherVariant>;
        fn load_store_everything(&mut self, a: LoadStoreAllSizes) -> LoadStoreAllSizes;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Lists + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_u8_param",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<u8>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.list_u8_param(x);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_u16_param",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<u16>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.list_u16_param(x);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_u32_param",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<u32>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.list_u32_param(x);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_u64_param",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<u64>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.list_u64_param(x);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_s8_param",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<i8>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.list_s8_param(x);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_s16_param",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<i16>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.list_s16_param(x);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_s32_param",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<i32>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.list_s32_param(x);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_s64_param",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<i64>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.list_s64_param(x);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_float32_param",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<f32>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.list_float32_param(x);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_float64_param",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<f64>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.list_float64_param(x);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_u8_ret",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<u8>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.list_u8_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_u16_ret",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<u16>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.list_u16_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_u32_ret",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<u32>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.list_u32_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_u64_ret",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<u64>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.list_u64_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_s8_ret",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<i8>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.list_s8_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_s16_ret",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<i16>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.list_s16_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_s32_ret",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<i32>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.list_s32_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_s64_ret",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<i64>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.list_s64_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_float32_ret",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<f32>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.list_float32_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "list_float64_ret",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<f64>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.list_float64_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "tuple_list",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<(u8, i8)>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<(i64, u32)>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.tuple_list(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "string_list_arg",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: Vec<String>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.string_list_arg(a);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "string_list_ret",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<String>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.string_list_ret())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "tuple_string_list",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<(u8, String)>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<(String, u8)>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.tuple_string_list(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "string_list",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<String>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<String>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.string_list(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "record_list",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<SomeRecord>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<OtherRecord>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.record_list(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "record_list_reverse",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<OtherRecord>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<SomeRecord>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.record_list_reverse(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "variant_list",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: Vec<SomeVariant>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<OtherVariant>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.variant_list(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "lists",
                "load_store_everything",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: LoadStoreAllSizes,
                | -> ::tauri_bindgen_host::anyhow::Result<LoadStoreAllSizes> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.load_store_everything(a))
                },
            )?;
        Ok(())
    }
}
