pub mod chars {
    use ::tauri_bindgen_host::bitflags;
    use ::tauri_bindgen_host::tauri_bindgen_abi;
    pub trait Chars: Sized {
        ///A function that accepts a character
        fn take_char(&mut self, x: char) -> ();
        ///A function that returns a character
        fn return_char(&mut self) -> char;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Chars,
    {
        router.func_wrap(
            "chars",
            "take_char",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: char| -> () {
                let cx = get_cx(cx.data_mut());
                cx.take_char(x)
            },
        )?;
        router.func_wrap(
            "chars",
            "return_char",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> char {
                let cx = get_cx(cx.data_mut());
                cx.return_char()
            },
        )?;
        Ok(())
    }
}
