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
    #[::tauri_bindgen_host::async_trait]
    pub trait ManyArguments: Sized {
        async fn many_args(
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
        async fn big_argument(&self, x: BigStruct);
    }
    pub fn add_to_router<T, U, R>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T, R>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: ManyArguments + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "many_arguments",
                "many_args",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (
                        u64,
                        u64,
                        u64,
                        u64,
                        u64,
                        u64,
                        u64,
                        u64,
                        u64,
                        u64,
                        u64,
                        u64,
                        u64,
                        u64,
                        u64,
                        u64,
                    )|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(
                            ctx
                                .many_args(
                                    p.0,
                                    p.1,
                                    p.2,
                                    p.3,
                                    p.4,
                                    p.5,
                                    p.6,
                                    p.7,
                                    p.8,
                                    p.9,
                                    p.10,
                                    p.11,
                                    p.12,
                                    p.13,
                                    p.14,
                                    p.15,
                                )
                                .await,
                        )
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "many_arguments",
                "big_argument",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: BigStruct|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.big_argument(p).await)
                    })
                },
            )?;
        Ok(())
    }
}
