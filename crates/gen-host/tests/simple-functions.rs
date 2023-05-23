#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod simple_functions {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait SimpleFunctions: Sized {
        fn f1(&mut self);
        fn f2(&mut self, a: u32);
        fn f3(&mut self, a: u32, b: u32);
        fn f4(&mut self) -> u32;
        fn f5(&mut self) -> (u32, u32);
        fn f6(&mut self, a: u32, b: u32, c: u32) -> (u32, u32, u32);
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: SimpleFunctions + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "simple_functions",
                "f1",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.f1())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "simple_functions",
                "f2",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: u32,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.f2(a))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "simple_functions",
                "f3",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: u32,
                    b: u32,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.f3(a, b))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "simple_functions",
                "f4",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<u32> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.f4())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "simple_functions",
                "f5",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<(u32, u32)> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.f5())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "simple_functions",
                "f6",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: u32,
                    b: u32,
                    c: u32,
                | -> ::tauri_bindgen_host::anyhow::Result<(u32, u32, u32)> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.f6(a, b, c))
                },
            )?;
        Ok(())
    }
}
