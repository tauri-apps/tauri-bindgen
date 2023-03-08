pub mod empty {
    use ::tauri_bindgen_host::bitflags;
    use ::tauri_bindgen_host::tauri_bindgen_abi;
    pub trait Empty: Sized {}
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Empty,
    {
        Ok(())
    }
}
