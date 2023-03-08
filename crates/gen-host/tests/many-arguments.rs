pub mod many_arguments {
    use ::tauri_bindgen_host::bitflags;
    use ::tauri_bindgen_host::tauri_bindgen_abi;
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct BigStruct {
        a1: String,
        a2: String,
        a3: String,
        a4: String,
        a5: String,
        a6: String,
        a7: String,
        a8: String,
        a9: String,
        a10: String,
        a11: String,
        a12: String,
        a13: String,
        a14: String,
        a15: String,
        a16: String,
        a17: String,
        a18: String,
        a19: String,
        a20: String,
    }
    pub trait ManyArguments: Sized {
        fn many_args(
            &mut self,
            a1: u64,
            a2: u64,
            a3: u64,
            a4: u64,
            a5: u64,
            a6: u64,
            a7: u64,
            a8: u64,
            a9: u64,
            a10: u64,
            a11: u64,
            a12: u64,
            a13: u64,
            a14: u64,
            a15: u64,
            a16: u64,
        ) -> ();
        fn big_argument(&mut self, x: BigStruct) -> ();
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: ManyArguments,
    {
        router.func_wrap(
            "many_arguments",
            "many_args",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                  a1: u64,
                  a2: u64,
                  a3: u64,
                  a4: u64,
                  a5: u64,
                  a6: u64,
                  a7: u64,
                  a8: u64,
                  a9: u64,
                  a10: u64,
                  a11: u64,
                  a12: u64,
                  a13: u64,
                  a14: u64,
                  a15: u64,
                  a16: u64|
                  -> () {
                let cx = get_cx(cx.data_mut());
                cx.many_args(
                    a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16,
                )
            },
        )?;
        router.func_wrap(
            "many_arguments",
            "big_argument",
            move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: BigStruct| -> () {
                let cx = get_cx(cx.data_mut());
                cx.big_argument(x)
            },
        )?;
        Ok(())
    }
}
