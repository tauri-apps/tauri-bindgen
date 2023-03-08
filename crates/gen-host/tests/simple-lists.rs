pub mod simple_lists {
    use ::tauri_bindgen_host::tauri_bindgen_abi;
    use ::tauri_bindgen_host::bitflags;
    pub trait SimpleLists: Sized {
        fn simple_list1(&mut self, l: Vec<u32>) -> ();
        fn simple_list2(&mut self) -> &'_ [u32];
        fn simple_list3(&mut self, a: Vec<u32>, b: Vec<u32>) -> (&'_ [u32], &'_ [u32]);
        fn simple_list4(&mut self, l: Vec<Vec<u32>>) -> &'_ [&'_ [u32]];
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: SimpleLists,
    {
        router
            .func_wrap(
                "simple_lists",
                "simple_list1",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    l: Vec<u32>,
                | -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.simple_list1(l)
                },
            )?;
        router
            .func_wrap(
                "simple_lists",
                "simple_list2",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> &'_ [u32] {
                    let cx = get_cx(cx.data_mut());
                    cx.simple_list2()
                },
            )?;
        router
            .func_wrap(
                "simple_lists",
                "simple_list3",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: Vec<u32>,
                    b: Vec<u32>,
                | -> (&'_ [u32], &'_ [u32]) {
                    let cx = get_cx(cx.data_mut());
                    cx.simple_list3(a, b)
                },
            )?;
        router
            .func_wrap(
                "simple_lists",
                "simple_list4",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    l: Vec<Vec<u32>>,
                | -> &'_ [&'_ [u32]] {
                    let cx = get_cx(cx.data_mut());
                    cx.simple_list4(l)
                },
            )?;
        Ok(())
    }
}
