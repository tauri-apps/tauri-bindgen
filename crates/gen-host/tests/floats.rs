#[allow(unused_imports, unused_variables)]
pub mod floats {
    use ::tauri_bindgen_host::bitflags;
    use ::tauri_bindgen_host::serde;
    pub trait Floats: Sized {
        fn float32_param(&mut self, x: f32) -> ();
        fn float64_param(&mut self, x: f64) -> ();
        fn float32_result(&mut self) -> f32;
        fn float64_result(&mut self) -> f64;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Floats,
    {
        Ok(())
    }
}
