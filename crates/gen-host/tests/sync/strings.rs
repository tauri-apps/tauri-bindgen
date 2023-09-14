#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod strings {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait Strings: Sized {
        fn a(&self, x: String);
        fn b(&self) -> String;
        fn c(&self, a: String, b: String) -> String;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: Strings + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "strings",
                "a",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: String| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.a(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "strings",
                "b",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.b())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "strings",
                "c",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (String, String)|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.c(p.0, p.1))
                },
            )?;
        Ok(())
    }
}
