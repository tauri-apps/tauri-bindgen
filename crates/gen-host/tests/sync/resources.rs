#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod resources {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait A {
        fn f1(&self);
        fn f2(&self, a: u32);
        fn f3(&self, a: u32, b: u32);
    }
    pub trait B {
        type A: A;
        fn f1(&self) -> ::tauri_bindgen_host::ResourceId;
        fn f2(&self, x: ::tauri_bindgen_host::ResourceId) -> Result<u32, ()>;
        fn f3(
            &self,
            x: Option<Vec<::tauri_bindgen_host::ResourceId>>,
        ) -> Result<::tauri_bindgen_host::ResourceId, ()>;
    }
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
        fn constructor_a(&self) -> ::tauri_bindgen_host::ResourceId;
        fn constructor_b(&self) -> ::tauri_bindgen_host::ResourceId;
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
            .define(
                "resources",
                "constructor_a",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.constructor_a())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "resources",
                "constructor_b",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.constructor_b())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "resources::resource::a",
                "f1",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    this_rid: ::tauri_bindgen_host::ResourceId,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    let r = ctx.get_a(this_rid)?;
                    Ok(r.f1())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "resources::resource::a",
                "f2",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    this_rid: ::tauri_bindgen_host::ResourceId,
                    a: u32,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    let r = ctx.get_a(this_rid)?;
                    Ok(r.f2(a))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "resources::resource::a",
                "f3",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    this_rid: ::tauri_bindgen_host::ResourceId,
                    a: u32,
                    b: u32,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    let r = ctx.get_a(this_rid)?;
                    Ok(r.f3(a, b))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "resources::resource::b",
                "f1",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    this_rid: ::tauri_bindgen_host::ResourceId,
                | -> ::tauri_bindgen_host::anyhow::Result<
                    ::tauri_bindgen_host::ResourceId,
                > {
                    let ctx = get_cx(ctx.data());
                    let r = ctx.get_b(this_rid)?;
                    Ok(r.f1())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "resources::resource::b",
                "f2",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    this_rid: ::tauri_bindgen_host::ResourceId,
                    x: ::tauri_bindgen_host::ResourceId,
                | -> ::tauri_bindgen_host::anyhow::Result<Result<u32, ()>> {
                    let ctx = get_cx(ctx.data());
                    let r = ctx.get_b(this_rid)?;
                    Ok(r.f2(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "resources::resource::b",
                "f3",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    this_rid: ::tauri_bindgen_host::ResourceId,
                    x: Option<Vec<::tauri_bindgen_host::ResourceId>>,
                | -> ::tauri_bindgen_host::anyhow::Result<
                    Result<::tauri_bindgen_host::ResourceId, ()>,
                > {
                    let ctx = get_cx(ctx.data());
                    let r = ctx.get_b(this_rid)?;
                    Ok(r.f3(x))
                },
            )?;
        Ok(())
    }
}
