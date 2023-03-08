pub mod conventions {
    use ::tauri_bindgen_host::bitflags;
    use ::tauri_bindgen_host::tauri_bindgen_abi;
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct LudicrousSpeed {
        how_fast_are_you_going: u32,
        i_am_going_extremely_slow: u64,
    }
    pub trait Conventions: Sized {
        fn kebab_case(&mut self) -> ();
        fn foo(&mut self, x: LudicrousSpeed) -> ();
        fn function_with_underscores(&mut self) -> ();
        fn function_with_no_weird_characters(&mut self) -> ();
        fn apple(&mut self) -> ();
        fn apple_pear(&mut self) -> ();
        fn apple_pear_grape(&mut self) -> ();
        fn a0(&mut self) -> ();
        fn is_xml(&mut self) -> ();
        fn explicit(&mut self) -> ();
        fn explicit_snake(&mut self) -> ();
        fn bool(&mut self) -> ();
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Conventions,
    {
        router.func_wrap(
            "conventions",
            "kebab_case",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.kebab_case()
            },
        )?;
        router.func_wrap(
            "conventions",
            "foo",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: LudicrousSpeed| -> () {
                let cx = get_cx(cx.data_mut());
                cx.foo(x)
            },
        )?;
        router.func_wrap(
            "conventions",
            "function_with_underscores",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.function_with_underscores()
            },
        )?;
        router.func_wrap(
            "conventions",
            "function_with_no_weird_characters",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.function_with_no_weird_characters()
            },
        )?;
        router.func_wrap(
            "conventions",
            "apple",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.apple()
            },
        )?;
        router.func_wrap(
            "conventions",
            "apple_pear",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.apple_pear()
            },
        )?;
        router.func_wrap(
            "conventions",
            "apple_pear_grape",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.apple_pear_grape()
            },
        )?;
        router.func_wrap(
            "conventions",
            "a0",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.a0()
            },
        )?;
        router.func_wrap(
            "conventions",
            "is_xml",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.is_xml()
            },
        )?;
        router.func_wrap(
            "conventions",
            "explicit",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.explicit()
            },
        )?;
        router.func_wrap(
            "conventions",
            "explicit_snake",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.explicit_snake()
            },
        )?;
        router.func_wrap(
            "conventions",
            "bool",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> () {
                let cx = get_cx(cx.data_mut());
                cx.bool()
            },
        )?;
        Ok(())
    }
}
