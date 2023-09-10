#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod integers {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait Integers: Sized {
        fn a1(&self, x: u8);
        fn a2(&self, x: i8);
        fn a3(&self, x: u16);
        fn a4(&self, x: i16);
        fn a5(&self, x: u32);
        fn a6(&self, x: i32);
        fn a7(&self, x: u64);
        fn a8(&self, x: i64);
        fn a9(&self, x: u128);
        fn a10(&self, x: i128);
        fn a11(
            &self,
            p1: u8,
            p2: i8,
            p3: u16,
            p4: i16,
            p5: u32,
            p6: i32,
            p7: u64,
            p8: i64,
            p9: u128,
            p10: i128,
        );
        fn r1(&self) -> u8;
        fn r2(&self) -> i8;
        fn r3(&self) -> u16;
        fn r4(&self) -> i16;
        fn r5(&self) -> u32;
        fn r6(&self) -> i32;
        fn r7(&self) -> u64;
        fn r8(&self) -> i64;
        fn r9(&self) -> u128;
        fn r10(&self) -> i128;
        fn pair_ret(&self) -> (i64, u8);
    }
    pub fn add_to_router<T, U, R: ::tauri_bindgen_host::tauri::Runtime>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T, R>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Integers + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a1",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                    x: u8,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a1(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a2",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                    x: i8,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a2(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a3",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                    x: u16,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a3(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a4",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                    x: i16,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a4(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a5",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                    x: u32,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a5(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a6",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                    x: i32,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a6(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a7",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                    x: u64,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a7(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a8",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                    x: i64,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a8(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a9",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                    x: u128,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a9(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a10",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                    x: i128,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a10(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a11",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                    p1: u8,
                    p2: i8,
                    p3: u16,
                    p4: i16,
                    p5: u32,
                    p6: i32,
                    p7: u64,
                    p8: i64,
                    p9: u128,
                    p10: i128,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a11(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r1",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<u8> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r1())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r2",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<i8> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r2())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r3",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<u16> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r3())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r4",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<i16> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r4())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r5",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<u32> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r5())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r6",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<i32> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r6())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r7",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<u64> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r7())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r8",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<i64> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r8())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r9",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<u128> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r9())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r10",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<i128> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r10())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "pair_ret",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<(i64, u8)> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.pair_ret())
                },
            )?;
        Ok(())
    }
}
