#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod simple_functions {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait SimpleFunctions: Sized {
        fn f1(&mut self) -> ();
        fn f2(&mut self, a: u32) -> ();
        fn f3(&mut self, a: u32, b: u32) -> ();
        fn f4(&mut self) -> u32;
        fn f5(&mut self) -> (u32, u32);
        fn f6(&mut self, a: u32, b: u32, c: u32) -> (u32, u32, u32);
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: SimpleFunctions,
    {
        Ok(())
    }
}
