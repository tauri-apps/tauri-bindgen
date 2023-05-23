#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod strings {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait Strings: Sized {
        fn a(&mut self, x: String);
        fn b(&mut self) -> String;
        fn c(&mut self, a: String, b: String) -> String;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Strings + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "strings",
                "a",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: String,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.a(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "strings",
                "b",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<String> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.b())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "strings",
                "c",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: String,
                    b: String,
                | -> ::tauri_bindgen_host::anyhow::Result<String> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.c(a, b))
                },
            )?;
        Ok(())
    }
}
