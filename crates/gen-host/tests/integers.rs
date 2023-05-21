#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod integers {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait Integers: Sized {
        fn a1(&mut self, x: u8);
        fn a2(&mut self, x: i8);
        fn a3(&mut self, x: u16);
        fn a4(&mut self, x: i16);
        fn a5(&mut self, x: u32);
        fn a6(&mut self, x: i32);
        fn a7(&mut self, x: u64);
        fn a8(&mut self, x: i64);
        fn a9(
            &mut self,
            p1: u8,
            p2: i8,
            p3: u16,
            p4: i16,
            p5: u32,
            p6: i32,
            p7: u64,
            p8: i64,
        );
        fn r1(&mut self) -> u8;
        fn r2(&mut self) -> i8;
        fn r3(&mut self) -> u16;
        fn r4(&mut self) -> i16;
        fn r5(&mut self) -> u32;
        fn r6(&mut self) -> i32;
        fn r7(&mut self) -> u64;
        fn r8(&mut self) -> i64;
        fn pair_ret(&mut self) -> (i64, u8);
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + 'static,
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
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: u8,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.a1(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a2",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: i8,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.a2(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a3",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: u16,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.a3(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a4",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: i16,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.a4(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a5",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: u32,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.a5(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a6",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: i32,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.a6(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a7",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: u64,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.a7(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a8",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: i64,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.a8(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "a9",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p1: u8,
                    p2: i8,
                    p3: u16,
                    p4: i16,
                    p5: u32,
                    p6: i32,
                    p7: u64,
                    p8: i64,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.a9(p1, p2, p3, p4, p5, p6, p7, p8))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r1",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<u8> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.r1())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r2",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<i8> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.r2())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r3",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<u16> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.r3())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r4",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<i16> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.r4())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r5",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<u32> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.r5())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r6",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<i32> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.r6())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r7",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<u64> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.r7())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "r8",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<i64> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.r8())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "integers",
                "pair_ret",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<(i64, u8)> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.pair_ret())
                },
            )?;
        Ok(())
    }
}
