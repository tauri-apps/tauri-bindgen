pub mod integers {
    use ::tauri_bindgen_host::tauri_bindgen_abi;
    use ::tauri_bindgen_host::bitflags;
    pub trait Integers: Sized {
        fn a1(&mut self, x: u8) -> ();
        fn a2(&mut self, x: i8) -> ();
        fn a3(&mut self, x: u16) -> ();
        fn a4(&mut self, x: i16) -> ();
        fn a5(&mut self, x: u32) -> ();
        fn a6(&mut self, x: i32) -> ();
        fn a7(&mut self, x: u64) -> ();
        fn a8(&mut self, x: i64) -> ();
        fn a9(
            &mut self,
            p1: u8,
            p2: i8,
            p3: u16,
            p4: i16,
            p5: u32,
            p6: i32,
            p7: u64,
            p8: i64,
        ) -> ();
        fn r1(&mut self) -> u8;
        fn r2(&mut self) -> i8;
        fn r3(&mut self) -> u16;
        fn r4(&mut self) -> i16;
        fn r5(&mut self) -> u32;
        fn r6(&mut self) -> i32;
        fn r7(&mut self) -> u64;
        fn r8(&mut self) -> i64;
        fn pair_ret(&mut self) -> (i64, u8);
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Integers,
    {
        router
            .func_wrap(
                "integers",
                "a1",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: u8| -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.a1(x)
                },
            )?;
        router
            .func_wrap(
                "integers",
                "a2",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: i8| -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.a2(x)
                },
            )?;
        router
            .func_wrap(
                "integers",
                "a3",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: u16| -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.a3(x)
                },
            )?;
        router
            .func_wrap(
                "integers",
                "a4",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: i16| -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.a4(x)
                },
            )?;
        router
            .func_wrap(
                "integers",
                "a5",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: u32| -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.a5(x)
                },
            )?;
        router
            .func_wrap(
                "integers",
                "a6",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: i32| -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.a6(x)
                },
            )?;
        router
            .func_wrap(
                "integers",
                "a7",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: u64| -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.a7(x)
                },
            )?;
        router
            .func_wrap(
                "integers",
                "a8",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: i64| -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.a8(x)
                },
            )?;
        router
            .func_wrap(
                "integers",
                "a9",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p1: u8,
                    p2: i8,
                    p3: u16,
                    p4: i16,
                    p5: u32,
                    p6: i32,
                    p7: u64,
                    p8: i64,
                | -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.a9(p1, p2, p3, p4, p5, p6, p7, p8)
                },
            )?;
        router
            .func_wrap(
                "integers",
                "r1",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> u8 {
                    let cx = get_cx(cx.data_mut());
                    cx.r1()
                },
            )?;
        router
            .func_wrap(
                "integers",
                "r2",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> i8 {
                    let cx = get_cx(cx.data_mut());
                    cx.r2()
                },
            )?;
        router
            .func_wrap(
                "integers",
                "r3",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> u16 {
                    let cx = get_cx(cx.data_mut());
                    cx.r3()
                },
            )?;
        router
            .func_wrap(
                "integers",
                "r4",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> i16 {
                    let cx = get_cx(cx.data_mut());
                    cx.r4()
                },
            )?;
        router
            .func_wrap(
                "integers",
                "r5",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> u32 {
                    let cx = get_cx(cx.data_mut());
                    cx.r5()
                },
            )?;
        router
            .func_wrap(
                "integers",
                "r6",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> i32 {
                    let cx = get_cx(cx.data_mut());
                    cx.r6()
                },
            )?;
        router
            .func_wrap(
                "integers",
                "r7",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> u64 {
                    let cx = get_cx(cx.data_mut());
                    cx.r7()
                },
            )?;
        router
            .func_wrap(
                "integers",
                "r8",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> i64 {
                    let cx = get_cx(cx.data_mut());
                    cx.r8()
                },
            )?;
        router
            .func_wrap(
                "integers",
                "pair_ret",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> (i64, u8) {
                    let cx = get_cx(cx.data_mut());
                    cx.pair_ret()
                },
            )?;
        Ok(())
    }
}
