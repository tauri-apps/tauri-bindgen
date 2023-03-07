pub mod records {
    use ::tauri_bindgen_host::bitflags;
    use ::tauri_bindgen_host::tauri_bindgen_abi;
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct AggregatesParam {
        a: Scalars,
        b: u32,
        c: Empty,
        d: String,
        e: ReallyFlags,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub struct AggregatesResult {
        a: Scalars,
        b: u32,
        c: Empty,
        d: String,
        e: ReallyFlags,
    }
    #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
    pub struct Empty {}
    pub type IntTypedef = i32;
    /**A record that is really just flags
    All of the fields are bool*/
    #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
    pub struct ReallyFlags {
        a: bool,
        b: bool,
        c: bool,
        d: bool,
        e: bool,
        f: bool,
        g: bool,
        h: bool,
        i: bool,
    }
    /**A record containing two scalar fields
    that both have the same type*/
    #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
    pub struct Scalars {
        ///The first field, named a
        a: u32,
        ///The second field, named b
        b: u32,
    }
    pub type TupleTypedef2 = (IntTypedef);
    pub trait Records: Sized {
        fn tuple_arg(&mut self, x: (char, u32)) -> ();
        fn tuple_result(&mut self) -> (char, u32);
        fn empty_arg(&mut self, x: Empty) -> ();
        fn empty_result(&mut self) -> Empty;
        fn scalar_arg(&mut self, x: Scalars) -> ();
        fn scalar_result(&mut self) -> Scalars;
        fn flags_arg(&mut self, x: ReallyFlags) -> ();
        fn flags_result(&mut self) -> ReallyFlags;
        fn aggregate_arg(&mut self, x: AggregatesResult) -> ();
        fn aggregate_result(&mut self) -> AggregatesParam<'_>;
        fn typedef_inout(&mut self, e: TupleTypedef2) -> i32;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Records,
    {
        router.func_wrap(
            "records",
            "tuple_arg",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: (char, u32)| -> () {
                let cx = get_cx(cx.data_mut());
                cx.tuple_arg(x)
            },
        )?;
        router.func_wrap(
            "records",
            "tuple_result",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> (char, u32) {
                let cx = get_cx(cx.data_mut());
                cx.tuple_result()
            },
        )?;
        router.func_wrap(
            "records",
            "empty_arg",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: Empty| -> () {
                let cx = get_cx(cx.data_mut());
                cx.empty_arg(x)
            },
        )?;
        router.func_wrap(
            "records",
            "empty_result",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> Empty {
                let cx = get_cx(cx.data_mut());
                cx.empty_result()
            },
        )?;
        router.func_wrap(
            "records",
            "scalar_arg",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: Scalars| -> () {
                let cx = get_cx(cx.data_mut());
                cx.scalar_arg(x)
            },
        )?;
        router.func_wrap(
            "records",
            "scalar_result",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> Scalars {
                let cx = get_cx(cx.data_mut());
                cx.scalar_result()
            },
        )?;
        router.func_wrap(
            "records",
            "flags_arg",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: ReallyFlags| -> () {
                let cx = get_cx(cx.data_mut());
                cx.flags_arg(x)
            },
        )?;
        router.func_wrap(
            "records",
            "flags_result",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> ReallyFlags {
                let cx = get_cx(cx.data_mut());
                cx.flags_result()
            },
        )?;
        router.func_wrap(
            "records",
            "aggregate_arg",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: AggregatesResult| -> () {
                let cx = get_cx(cx.data_mut());
                cx.aggregate_arg(x)
            },
        )?;
        router.func_wrap(
            "records",
            "aggregate_result",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> AggregatesParam<'_> {
                let cx = get_cx(cx.data_mut());
                cx.aggregate_result()
            },
        )?;
        router.func_wrap(
            "records",
            "typedef_inout",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, e: TupleTypedef2| -> i32 {
                let cx = get_cx(cx.data_mut());
                cx.typedef_inout(e)
            },
        )?;
        Ok(())
    }
}
