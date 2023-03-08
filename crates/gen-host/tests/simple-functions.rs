pub mod simple_functions {
    use ::tauri_bindgen_host::bitflags;
    use ::tauri_bindgen_host::tauri_bindgen_abi;
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
        router.func_wrap(
            "simple_functions",
            "f1",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.f1()
            },
        )?;
        router.func_wrap(
            "simple_functions",
            "f2",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, a: u32| -> () {
                let cx = get_cx(cx.data_mut());
                cx.f2(a)
            },
        )?;
        router.func_wrap(
            "simple_functions",
            "f3",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, a: u32, b: u32| -> () {
                let cx = get_cx(cx.data_mut());
                cx.f3(a, b)
            },
        )?;
        router.func_wrap(
            "simple_functions",
            "f4",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> u32 {
                let cx = get_cx(cx.data_mut());
                cx.f4()
            },
        )?;
        router.func_wrap(
            "simple_functions",
            "f5",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> (u32, u32) {
                let cx = get_cx(cx.data_mut());
                cx.f5()
            },
        )?;
        router.func_wrap(
            "simple_functions",
            "f6",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                  a: u32,
                  b: u32,
                  c: u32|
                  -> (u32, u32, u32) {
                let cx = get_cx(cx.data_mut());
                cx.f6(a, b, c)
            },
        )?;
        Ok(())
    }
}
