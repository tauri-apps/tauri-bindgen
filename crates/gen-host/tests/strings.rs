pub mod strings {
    use ::tauri_bindgen_host::tauri_bindgen_abi;
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
        router
            .func_wrap(
                "strings",
                "a",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: String,
                | -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.a(x)
                },
            )?;
        router
            .func_wrap(
                "strings",
                "b",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> &'_ str {
                    let cx = get_cx(cx.data_mut());
                    cx.b()
                },
            )?;
        router
            .func_wrap(
                "strings",
                "c",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: String,
                    b: String,
                | -> &'_ str {
                    let cx = get_cx(cx.data_mut());
                    cx.c(a, b)
                },
            )?;
        Ok(())
    }
}
