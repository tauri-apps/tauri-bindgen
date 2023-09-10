#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod streams {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait Streams: Sized {
        type StringStreamStream: ::tauri_bindgen_host::Stream<Item = String>
            + Send
            + 'static;
        ///A function that returns a stream of strings
        fn string_stream(&self) -> Self::StringStreamStream;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Streams + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "streams",
                "string_stream",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<String> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.string_stream())
                },
            )?;
        Ok(())
    }
}
