#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod floats {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait Floats: Sized {
        fn float32_param(&self, x: f32);
        fn float64_param(&self, x: f64);
        fn float32_result(&self) -> f32;
        fn float64_result(&self) -> f64;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: Floats + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "floats",
                "float32_param",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: f32| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.float32_param(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "floats",
                "float64_param",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: f64| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.float64_param(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "floats",
                "float32_result",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.float32_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "floats",
                "float64_result",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.float64_result())
                },
            )?;
        Ok(())
    }
}
