#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod chars {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait Chars: Sized {
        ///A function that accepts a character
        fn take_char(&self, x: char);
        ///A function that returns a character
        fn return_char(&self) -> char;
    }
    pub fn add_to_router<T, U, R>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T, R>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: Chars + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "chars",
                "take_char",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: char| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.take_char(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "chars",
                "return_char",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.return_char())
                },
            )?;
        Ok(())
    }
}
