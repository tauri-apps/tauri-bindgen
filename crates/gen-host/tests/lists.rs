#[allow(clippy::all)]
pub mod import_lists {
    #[derive(
        Debug,
        Clone,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    #[serde(rename_all = "camelCase")]
    pub struct SomeRecord {
        pub x: String,
        pub y: OtherRecord,
        pub z: Vec<OtherRecord>,
        pub c1: u32,
        pub c2: u64,
        pub c3: i32,
        pub c4: i64,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    #[serde(rename_all = "camelCase")]
    pub struct OtherRecord {
        pub a1: u32,
        pub a2: u64,
        pub a3: i32,
        pub a4: i64,
        pub b: String,
        pub c: Vec<u8>,
    }
    #[derive(Debug, Clone, PartialEq, ::tauri_bindgen_host::serde::Deserialize)]
    pub enum SomeVariant {
        A(String),
        B,
        C(u32),
        D(Vec<OtherVariant>),
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    pub enum OtherVariant {
        A,
        B(u32),
        C(String),
    }
    pub type LoadStoreAllSizes =
        Vec<(String, u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, char)>;
    #[::tauri_bindgen_host::async_trait]
    pub trait ImportLists: Sized {
        async fn list_u8_param(&self, x: Vec<u8>) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn list_u16_param(&self, x: Vec<u16>) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn list_u32_param(&self, x: Vec<u32>) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn list_u64_param(&self, x: Vec<u64>) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn list_s8_param(&self, x: Vec<i8>) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn list_s16_param(&self, x: Vec<i16>) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn list_s32_param(&self, x: Vec<i32>) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn list_s64_param(&self, x: Vec<i64>) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn list_float32_param(&self, x: Vec<f32>)
            -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn list_float64_param(&self, x: Vec<f64>)
            -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn list_u8_ret(&self) -> ::tauri_bindgen_host::anyhow::Result<Vec<u8>>;
        async fn list_u16_ret(&self) -> ::tauri_bindgen_host::anyhow::Result<Vec<u16>>;
        async fn list_u32_ret(&self) -> ::tauri_bindgen_host::anyhow::Result<Vec<u32>>;
        async fn list_u64_ret(&self) -> ::tauri_bindgen_host::anyhow::Result<Vec<u64>>;
        async fn list_s8_ret(&self) -> ::tauri_bindgen_host::anyhow::Result<Vec<i8>>;
        async fn list_s16_ret(&self) -> ::tauri_bindgen_host::anyhow::Result<Vec<i16>>;
        async fn list_s32_ret(&self) -> ::tauri_bindgen_host::anyhow::Result<Vec<i32>>;
        async fn list_s64_ret(&self) -> ::tauri_bindgen_host::anyhow::Result<Vec<i64>>;
        async fn list_float32_ret(&self) -> ::tauri_bindgen_host::anyhow::Result<Vec<f32>>;
        async fn list_float64_ret(&self) -> ::tauri_bindgen_host::anyhow::Result<Vec<f64>>;
        async fn tuple_list(
            &self,
            x: Vec<(u8, i8)>,
        ) -> ::tauri_bindgen_host::anyhow::Result<Vec<(i64, u32)>>;
        async fn string_list_arg(&self, a: Vec<String>)
            -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn string_list_ret(&self) -> ::tauri_bindgen_host::anyhow::Result<Vec<String>>;
        async fn tuple_string_list(
            &self,
            x: Vec<(u8, String)>,
        ) -> ::tauri_bindgen_host::anyhow::Result<Vec<(String, u8)>>;
        async fn string_list(
            &self,
            x: Vec<String>,
        ) -> ::tauri_bindgen_host::anyhow::Result<Vec<String>>;
        async fn record_list(
            &self,
            x: Vec<SomeRecord>,
        ) -> ::tauri_bindgen_host::anyhow::Result<Vec<OtherRecord>>;
        async fn record_list_reverse(
            &self,
            x: Vec<OtherRecord>,
        ) -> ::tauri_bindgen_host::anyhow::Result<Vec<SomeRecord>>;
        async fn variant_list(
            &self,
            x: Vec<SomeVariant>,
        ) -> ::tauri_bindgen_host::anyhow::Result<Vec<OtherVariant>>;
        async fn load_store_everything(
            &self,
            a: LoadStoreAllSizes,
        ) -> ::tauri_bindgen_host::anyhow::Result<LoadStoreAllSizes>;
    }

    fn verfiy_idl_hash<'a, R: ::tauri_bindgen_host::tauri::Runtime>(
        item: ::tauri_bindgen_host::tauri::command::CommandItem<'a, R>,
    ) -> Result<(), ::tauri_bindgen_host::tauri::InvokeError> {
        let hash: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(item)?;

        if hash != "a744d1c6fec40184" {
            return Err(::tauri_bindgen_host::tauri::InvokeError::from(
                "IDL version mismatch",
            ));
        }

        Ok(())
    }

    pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
    where
        U: ImportLists + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime + 'static,
    {
        move |invoke| match invoke.message.command() {
            "list-u8-param" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-u8-param", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u8-param",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u8-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "list-u8-param",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u8_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-u16-param" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-u16-param", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u16-param",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u16-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "list-u16-param",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u16_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-u32-param" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-u32-param", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u32-param",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u32-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "list-u32-param",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u32_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-u64-param" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-u64-param", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u64-param",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u64-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "list-u64-param",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u64_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s8-param" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-s8-param", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s8-param",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s8-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "list-s8-param",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s8_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s16-param" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-s16-param", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s16-param",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s16-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "list-s16-param",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s16_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s32-param" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-s32-param", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s32-param",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s32-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "list-s32-param",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s32_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s64-param" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-s64-param", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s64-param",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s64-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "list-s64-param",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s64_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-float32-param" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-float32-param", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-float32-param",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-float32-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "list-float32-param",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_float32_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-float64-param" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-float64-param", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-float64-param",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-float64-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "list-float64-param",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_float64_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-u8-ret" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-u8-ret", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u8-ret",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u8_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-u16-ret" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-u16-ret", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u16-ret",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u16_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-u32-ret" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-u32-ret", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u32-ret",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u32_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-u64-ret" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-u64-ret", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u64-ret",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u64_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s8-ret" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-s8-ret", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s8-ret",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s8_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s16-ret" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-s16-ret", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s16-ret",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s16_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s32-ret" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-s32-ret", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s32-ret",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s32_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s64-ret" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-s64-ret", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s64-ret",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s64_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-float32-ret" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-float32-ret", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-float32-ret",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_float32_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-float64-ret" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "list-float64-ret", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-float64-ret",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_float64_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "tuple-list" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "tuple-list", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "tuple-list",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "tuple-list",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "tuple-list",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.tuple_list(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "string-list-arg" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "string-list-arg", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "string-list-arg",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let a = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "string-list-arg",
                        key: "a",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "string-list-arg",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.string_list_arg(a);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "string-list-ret" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "string-list-ret", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "string-list-ret",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.string_list_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "tuple-string-list" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "tuple-string-list", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "tuple-string-list",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "tuple-string-list",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "tuple-string-list",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.tuple_string_list(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "string-list" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "string-list", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "string-list",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "string-list",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "string-list",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.string_list(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "record-list" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "record-list", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "record-list",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "record-list",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "record-list",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.record_list(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "record-list-reverse" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "record-list-reverse", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "record-list-reverse",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "record-list-reverse",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "record-list-reverse",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.record_list_reverse(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "variant-list" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "variant-list", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "variant-list",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "variant-list",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "variant-list",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.variant_list(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "load-store-everything" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "import-lists", function = "load-store-everything", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "load-store-everything",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
                let a = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "load-store-everything",
                        key: "a",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        ::tauri_bindgen_host::tracing::error!(
                            module = "import-lists",
                            function = "load-store-everything",
                            "Invoke handler returned error {:?}",
                            err
                        );
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.load_store_everything(a);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            func_name => {
                ::tauri_bindgen_host::tracing::error!(
                    module = "import-lists",
                    function = func_name,
                    "Not Found"
                );
                invoke.resolver.reject("Not Found")
            }
        }
    }
}
