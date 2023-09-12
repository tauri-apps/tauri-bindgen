#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod strings {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[::tauri_bindgen_host::async_trait]
    pub trait Strings: Sized {
        async fn a(&self, x: String);
        async fn b(&self) -> String;
        async fn c(&self, a: String, b: String) -> String;
    }
    pub fn add_to_router<T, U, R>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T, R>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: Strings + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "strings",
                "a",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: String| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.a(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "strings",
                "b",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.b().await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "strings",
                "c",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (String, String)|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.c(p.0, p.1).await)
                    })
                },
            )?;
        Ok(())
    }
}
