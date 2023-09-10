#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod conventions {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[derive(serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct LudicrousSpeed {
        pub how_fast_are_you_going: u32,
        pub i_am_going_extremely_slow: u64,
    }
    pub trait Conventions: Sized {
        fn kebab_case(&self);
        fn foo(&self, x: LudicrousSpeed);
        fn function_with_underscores(&self);
        fn function_with_no_weird_characters(&self);
        fn apple(&self);
        fn apple_pear(&self);
        fn apple_pear_grape(&self);
        fn a0(&self);
        fn is_xml(&self);
        fn explicit(&self);
        fn explicit_snake(&self);
        fn bool(&self);
    }
    pub fn add_to_router<T, U, R: ::tauri_bindgen_host::tauri::Runtime>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T, R>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Conventions + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "kebab_case",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.kebab_case())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "foo",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                    x: LudicrousSpeed,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.foo(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "function_with_underscores",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.function_with_underscores())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "function_with_no_weird_characters",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.function_with_no_weird_characters())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "apple",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.apple())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "apple_pear",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.apple_pear())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "apple_pear_grape",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.apple_pear_grape())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "a0",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a0())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "is_xml",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.is_xml())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "explicit",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.explicit())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "explicit_snake",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.explicit_snake())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "bool",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.bool())
                },
            )?;
        Ok(())
    }
}
