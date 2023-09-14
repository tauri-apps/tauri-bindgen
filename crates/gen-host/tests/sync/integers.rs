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
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: Integers + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "a1",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: u8| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a1(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "a2",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: i8| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a2(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "a3",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: u16| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a3(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "a4",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: i16| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a4(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "a5",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: u32| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a5(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "a6",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: i32| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a6(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "a7",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: u64| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a7(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "a8",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: i64| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a8(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "a9",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: u128| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a9(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "a10",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: i128| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a10(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "a11",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (u8, i8, u16, i16, u32, i32, u64, i64, u128, i128)|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a11(p.0, p.1, p.2, p.3, p.4, p.5, p.6, p.7, p.8, p.9))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "r1",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r1())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "r2",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r2())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "r3",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r3())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "r4",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r4())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "r5",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r5())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "r6",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r6())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "r7",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r7())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "r8",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r8())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "r9",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r9())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "r10",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.r10())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "integers",
                "pair_ret",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.pair_ret())
                },
            )?;
        Ok(())
    }
}
