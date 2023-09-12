#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod small_anonymous {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[derive(serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Error {
        Success,
        Failure,
    }
    #[::tauri_bindgen_host::async_trait]
    pub trait SmallAnonymous: Sized {
        async fn option_test(&self) -> Result<Option<String>, Error>;
    }
    pub fn add_to_router<T, U, R>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T, R>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: SmallAnonymous + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "small_anonymous",
                "option_test",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.option_test().await)
                    })
                },
            )?;
        Ok(())
    }
}
