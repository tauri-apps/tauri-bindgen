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
    pub trait SmallAnonymous: Sized {
        fn option_test(&self) -> Result<Option<String>, Error>;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: SmallAnonymous + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "small_anonymous",
                "option_test",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Result<Option<String>, Error>> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.option_test())
                },
            )?;
        Ok(())
    }
}
