#[allow(clippy::all)]
pub mod imports {
    #[::tauri_bindgen_host::async_trait]
    pub trait Imports: Sized {
        async fn f1(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn f2(&self, a: u32) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn f3(&self, a: u32, b: u32) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn f4(&self) -> ::tauri_bindgen_host::anyhow::Result<u32>;
        async fn f5(&self) -> ::tauri_bindgen_host::anyhow::Result<(u32, u32)>;
        async fn f6(
            &self,
            a: u32,
            b: u32,
            c: u32,
        ) -> ::tauri_bindgen_host::anyhow::Result<(u32, u32, u32)>;
    }

    fn verfiy_idl_hash<'a, R: ::tauri_bindgen_host::tauri::Runtime>(
        item: ::tauri_bindgen_host::tauri::command::CommandItem<'a, R>,
    ) -> Result<(), ::tauri_bindgen_host::tauri::InvokeError> {
        let hash: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(item)?;

        if hash != "ebb2d6f0441e00a0" {
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
            "f1" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "f1", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "f1",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.f1();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "f2" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "f2", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "f2",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let a = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "f2",
                        key: "a",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "f2",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.f2(a);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "f3" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "f3", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "f3",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let a = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "f3",
                        key: "a",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "f3",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                let b = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "f3",
                        key: "b",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "f3",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.f3(a, b);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "f4" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "f4", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "f4",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.f4();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "f5" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "f5", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "f5",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.f5();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "f6" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "f6", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "f6",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let a = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "f6",
                        key: "a",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "f6",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                let b = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "f6",
                        key: "b",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "f6",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                let c = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "f6",
                        key: "c",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "imports",
                            function = "f6",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.f6(a, b, c);

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
