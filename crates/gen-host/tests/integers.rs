#[allow(clippy::all)]
pub mod imports {
    #[::tauri_bindgen_host::async_trait]
    pub trait Imports: Sized {
        async fn a1(&self, x: u8) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn a2(&self, x: i8) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn a3(&self, x: u16) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn a4(&self, x: i16) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn a5(&self, x: u32) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn a6(&self, x: i32) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn a7(&self, x: u64) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn a8(&self, x: i64) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn a9(
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
        async fn r1(&self) -> ::tauri_bindgen_host::anyhow::Result<u8>;
        async fn r2(&self) -> ::tauri_bindgen_host::anyhow::Result<i8>;
        async fn r3(&self) -> ::tauri_bindgen_host::anyhow::Result<u16>;
        async fn r4(&self) -> ::tauri_bindgen_host::anyhow::Result<i16>;
        async fn r5(&self) -> ::tauri_bindgen_host::anyhow::Result<u32>;
        async fn r6(&self) -> ::tauri_bindgen_host::anyhow::Result<i32>;
        async fn r7(&self) -> ::tauri_bindgen_host::anyhow::Result<u64>;
        async fn r8(&self) -> ::tauri_bindgen_host::anyhow::Result<i64>;
        async fn pair_ret(&self) -> ::tauri_bindgen_host::anyhow::Result<(i64, u8)>;
    }

    fn verfiy_idl_hash<'a, R: ::tauri_bindgen_host::tauri::Runtime>(
        item: ::tauri_bindgen_host::tauri::command::CommandItem<'a, R>,
    ) -> Result<(), ::tauri_bindgen_host::tauri::InvokeError> {
        let hash: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(item)?;

        if hash != "279b557e344c2e05" {
            return Err(::tauri_bindgen_host::tauri::InvokeError::from(
                "IDL version mismatch",
            ));
        }

        Ok(())
    }

    pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
    where
        U: Imports + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime + 'static,
    {
        move |invoke| match invoke.message.command() {
            "a1" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "a1", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a1",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a1",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a1",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.a1(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "a2" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "a2", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a2",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a2",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a2",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.a2(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "a3" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "a3", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a3",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a3",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a3",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.a3(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "a4" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "a4", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a4",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a4",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a4",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.a4(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "a5" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "a5", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a5",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a5",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a5",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.a5(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "a6" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "a6", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a6",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a6",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a6",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.a6(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "a7" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "a7", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a7",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a7",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a7",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.a7(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "a8" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "a8", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a8",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a8",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a8",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.a8(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "a9" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "a9", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a9",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let p1 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a9",
                        key: "p1",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a9",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                let p2 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a9",
                        key: "p2",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a9",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                let p3 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a9",
                        key: "p3",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a9",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                let p4 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a9",
                        key: "p4",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a9",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                let p5 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a9",
                        key: "p5",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a9",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                let p6 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a9",
                        key: "p6",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a9",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                let p7 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a9",
                        key: "p7",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a9",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                let p8 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a9",
                        key: "p8",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "a9",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.a9(p1, p2, p3, p4, p5, p6, p7, p8);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "r1" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "r1", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "r1",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.r1();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "r2" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "r2", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "r2",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.r2();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "r3" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "r3", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "r3",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.r3();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "r4" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "r4", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "r4",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.r4();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "r5" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "r5", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "r5",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.r5();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "r6" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "r6", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "r6",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.r6();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "r7" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "r7", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "r7",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.r7();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "r8" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "r8", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "r8",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.r8();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "pair-ret" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "pair-ret", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "pair-ret",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.pair_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            func_name => {
                ::tauri_bindgen_host::tracing::error!(
                    module = "imports",
                    function = func_name,
                    "Not Found"
                );
                invoke.resolver.reject("Not Found")
            }
        }
    }
}
