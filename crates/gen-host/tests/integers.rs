#[allow(clippy::all)]
pub mod imports {
    pub trait Imports: Sized {
        fn a1(&self, x: u8) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn a2(&self, x: i8) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn a3(&self, x: u16) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn a4(&self, x: i16) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn a5(&self, x: u32) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn a6(&self, x: i32) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn a7(&self, x: u64) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn a8(&self, x: i64) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn a9(
            &self,
            p1: u8,
            p2: i8,
            p3: u16,
            p4: i16,
            p5: u32,
            p6: i32,
            p7: u64,
            p8: i64,
        ) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn r1(&self) -> ::tauri_bindgen_host::anyhow::Result<u8>;
        fn r2(&self) -> ::tauri_bindgen_host::anyhow::Result<i8>;
        fn r3(&self) -> ::tauri_bindgen_host::anyhow::Result<u16>;
        fn r4(&self) -> ::tauri_bindgen_host::anyhow::Result<i16>;
        fn r5(&self) -> ::tauri_bindgen_host::anyhow::Result<u32>;
        fn r6(&self) -> ::tauri_bindgen_host::anyhow::Result<i32>;
        fn r7(&self) -> ::tauri_bindgen_host::anyhow::Result<u64>;
        fn r8(&self) -> ::tauri_bindgen_host::anyhow::Result<i64>;
        fn pair_ret(&self) -> ::tauri_bindgen_host::anyhow::Result<(i64, u8)>;
    }

    pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
    where
        U: Imports + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime,
    {
        move |invoke| match invoke.message.command() {
            "a1" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.a1(
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a1",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                );

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "a2" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.a2(
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a2",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                );

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "a3" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.a3(
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a3",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                );

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "a4" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.a4(
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a4",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                );

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "a5" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.a5(
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a5",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                );

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "a6" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.a6(
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a6",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                );

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "a7" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.a7(
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a7",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                );

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "a8" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.a8(
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a8",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                );

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "a9" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.a9(
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a9",
                            key: "p1",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a9",
                            key: "p2",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a9",
                            key: "p3",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a9",
                            key: "p4",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a9",
                            key: "p5",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a9",
                            key: "p6",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a9",
                            key: "p7",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "a9",
                            key: "p8",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                );

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "r1" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.r1();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "r2" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.r2();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "r3" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.r3();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "r4" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.r4();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "r5" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.r5();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "r6" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.r6();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "r7" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.r7();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "r8" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.r8();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "pair-ret" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.pair_ret();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            _ => invoke.resolver.reject("Not Found"),
        }
    }
}
