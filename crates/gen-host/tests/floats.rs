pub mod floats {
    use ::tauri_bindgen_host::bitflags;
    use ::tauri_bindgen_host::tauri_bindgen_abi;
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
        router.func_wrap(
            "floats",
            "float32_param",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: f32| -> () {
                let cx = get_cx(cx.data_mut());
                cx.float32_param(x)
            },
        )?;
        router.func_wrap(
            "floats",
            "float64_param",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: f64| -> () {
                let cx = get_cx(cx.data_mut());
                cx.float64_param(x)
            },
        )?;
        router.func_wrap(
            "floats",
            "float32_result",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> f32 {
                let cx = get_cx(cx.data_mut());
                cx.float32_result()
            },
        )?;
        router.func_wrap(
            "floats",
            "float64_result",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> f64 {
                let cx = get_cx(cx.data_mut());
                cx.float64_result()
            },
        )?;
        Ok(())
    }
}
