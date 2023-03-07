pub mod multi_return {
    use ::tauri_bindgen_host::bitflags;
    use ::tauri_bindgen_host::tauri_bindgen_abi;
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
        router.func_wrap(
            "multi_return",
            "mra",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.mra()
            },
        )?;
        router.func_wrap(
            "multi_return",
            "mrb",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| {
                let cx = get_cx(cx.data_mut());
                cx.mrb()
            },
        )?;
        router.func_wrap(
            "multi_return",
            "mrc",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> u32 {
                let cx = get_cx(cx.data_mut());
                cx.mrc()
            },
        )?;
        router.func_wrap(
            "multi_return",
            "mrd",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> u32 {
                let cx = get_cx(cx.data_mut());
                cx.mrd()
            },
        )?;
        router.func_wrap(
            "multi_return",
            "mre",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> (u32, f32) {
                let cx = get_cx(cx.data_mut());
                cx.mre()
            },
        )?;
        Ok(())
    }
}
