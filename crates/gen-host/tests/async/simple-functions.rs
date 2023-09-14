#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod simple_functions {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[::tauri_bindgen_host::async_trait]
    pub trait SimpleFunctions: Sized {
        async fn f1(&self);
        async fn f2(&self, a: u32);
        async fn f3(&self, a: u32, b: u32);
        async fn f4(&self) -> u32;
        async fn f5(&self) -> (u32, u32);
        async fn f6(&self, a: u32, b: u32, c: u32) -> (u32, u32, u32);
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: SimpleFunctions + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "simple_functions",
                "f1",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.f1().await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "simple_functions",
                "f2",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: u32| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.f2(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "simple_functions",
                "f3",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (u32, u32)|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.f3(p.0, p.1).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "simple_functions",
                "f4",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.f4().await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "simple_functions",
                "f5",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.f5().await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "simple_functions",
                "f6",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (u32, u32, u32)|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.f6(p.0, p.1, p.2).await)
                    })
                },
            )?;
        Ok(())
    }
}
