#[allow(unused_imports, unused_variables)]
pub mod multi_return {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait MultiReturn: Sized {
        fn mra(&mut self) -> ();
        fn mrb(&mut self);
        fn mrc(&mut self) -> u32;
        fn mrd(&mut self) -> u32;
        fn mre(&mut self) -> (u32, f32);
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: MultiReturn,
    {
        Ok(())
    }
}
