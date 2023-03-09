#[allow(unused_imports, unused_variables)]
#[rustfmt::skip]
pub mod chars {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
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
        Ok(())
    }
}
