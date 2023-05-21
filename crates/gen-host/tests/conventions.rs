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
        fn kebab_case(&mut self);
        fn foo(&mut self, x: LudicrousSpeed);
        fn function_with_underscores(&mut self);
        fn function_with_no_weird_characters(&mut self);
        fn apple(&mut self);
        fn apple_pear(&mut self);
        fn apple_pear_grape(&mut self);
        fn a0(&mut self);
        fn is_xml(&mut self);
        fn explicit(&mut self);
        fn explicit_snake(&mut self);
        fn bool(&mut self);
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + 'static,
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
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.kebab_case())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "foo",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: LudicrousSpeed,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.foo(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "function_with_underscores",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.function_with_underscores())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "function_with_no_weird_characters",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.function_with_no_weird_characters())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "apple",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.apple())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "apple_pear",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.apple_pear())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "apple_pear_grape",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.apple_pear_grape())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "a0",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.a0())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "is_xml",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.is_xml())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "explicit",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.explicit())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "explicit_snake",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.explicit_snake())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "conventions",
                "bool",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.bool())
                },
            )?;
        Ok(())
    }
}
