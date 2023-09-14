#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod resources {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[::tauri_bindgen_host::async_trait]
    pub trait A {
        async fn f1(&self);
        async fn f2(&self, a: u32);
        async fn f3(&self, a: u32, b: u32);
    }
    #[::tauri_bindgen_host::async_trait]
    pub trait B {
        type A: A;
        async fn f1(&self) -> ::tauri_bindgen_host::ResourceId;
        async fn f2(&self, x: ::tauri_bindgen_host::ResourceId) -> Result<u32, ()>;
        async fn f3(
            &self,
            x: Option<Vec<::tauri_bindgen_host::ResourceId>>,
        ) -> Result<::tauri_bindgen_host::ResourceId, ()>;
    }
    #[::tauri_bindgen_host::async_trait]
    pub trait Resources: Sized {
        type A: A + Send + Sync;
        fn get_a(
            &self,
            id: ::tauri_bindgen_host::ResourceId,
        ) -> ::tauri_bindgen_host::Result<::std::sync::Arc<Self::A>>;
        type B: B + Send + Sync;
        fn get_b(
            &self,
            id: ::tauri_bindgen_host::ResourceId,
        ) -> ::tauri_bindgen_host::Result<::std::sync::Arc<Self::B>>;
        async fn constructor_a(&self) -> ::tauri_bindgen_host::ResourceId;
        async fn constructor_b(&self) -> ::tauri_bindgen_host::ResourceId;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: Resources + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "resources",
                "constructor_a",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.constructor_a().await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "resources",
                "constructor_b",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.constructor_b().await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "resources::resource::a",
                "f1",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (::tauri_bindgen_host::ResourceId,)|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        let r = ctx.get_a(p.0)?;
                        Ok(r.f1().await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "resources::resource::a",
                "f2",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (::tauri_bindgen_host::ResourceId, u32)|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        let r = ctx.get_a(p.0)?;
                        Ok(r.f2(p.1).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "resources::resource::a",
                "f3",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (::tauri_bindgen_host::ResourceId, u32, u32)|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        let r = ctx.get_a(p.0)?;
                        Ok(r.f3(p.1, p.2).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "resources::resource::b",
                "f1",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (::tauri_bindgen_host::ResourceId,)|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        let r = ctx.get_b(p.0)?;
                        Ok(r.f1().await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "resources::resource::b",
                "f2",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (
                        ::tauri_bindgen_host::ResourceId,
                        ::tauri_bindgen_host::ResourceId,
                    )|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        let r = ctx.get_b(p.0)?;
                        Ok(r.f2(p.1).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "resources::resource::b",
                "f3",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (
                        ::tauri_bindgen_host::ResourceId,
                        Option<Vec<::tauri_bindgen_host::ResourceId>>,
                    )|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        let r = ctx.get_b(p.0)?;
                        Ok(r.f3(p.1).await)
                    })
                },
            )?;
        Ok(())
    }
}
