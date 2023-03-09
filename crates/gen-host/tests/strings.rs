#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod strings {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait Strings: Sized {
        fn a(&mut self, x: String) -> ();
        fn b(&mut self) -> &'_ str;
        fn c(&mut self, a: String, b: String) -> &'_ str;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Strings,
    {
        Ok(())
    }
}
