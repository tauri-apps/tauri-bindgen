#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod many_arguments {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[derive(serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct BigStruct {
        pub a1: String,
        pub a2: String,
        pub a3: String,
        pub a4: String,
        pub a5: String,
        pub a6: String,
        pub a7: String,
        pub a8: String,
        pub a9: String,
        pub a10: String,
        pub a11: String,
        pub a12: String,
        pub a13: String,
        pub a14: String,
        pub a15: String,
        pub a16: String,
        pub a17: String,
        pub a18: String,
        pub a19: String,
        pub a20: String,
    }
    pub trait ManyArguments: Sized {
        fn many_args(
            &self,
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
        );
        fn big_argument(&self, x: BigStruct);
    }
    pub fn add_to_router<T, U, R: ::tauri_bindgen_host::tauri::Runtime>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T, R>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: ManyArguments + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "many_arguments",
                "many_args",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
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
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(
                        ctx
                            .many_args(
                                a1,
                                a2,
                                a3,
                                a4,
                                a5,
                                a6,
                                a7,
                                a8,
                                a9,
                                a10,
                                a11,
                                a12,
                                a13,
                                a14,
                                a15,
                                a16,
                            ),
                    )
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "many_arguments",
                "big_argument",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T, R>,
                    x: BigStruct,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.big_argument(x))
                },
            )?;
        Ok(())
    }
}
