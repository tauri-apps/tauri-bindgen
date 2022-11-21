#[allow(clippy::all)]
pub mod import_lists {
    #[derive(
        Debug,
        Clone,
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
    #[derive(Debug, Clone, ::tauri_bindgen_host::serde::Deserialize)]
    pub enum SomeVariant {
        A(String),
        B,
        C(u32),
        D(Vec<OtherVariant>),
    }
    #[derive(
        Debug,
        Clone,
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

    pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
    where
        U: ImportLists + Copy + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime,
    {
        move |invoke| match invoke.message.command() {
            "list-u8-param" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u8-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u8_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-u16-param" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u16-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u16_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-u32-param" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u32-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u32_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-u64-param" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-u64-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u64_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s8-param" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s8-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s8_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s16-param" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s16-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s16_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s32-param" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s32-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s32_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s64-param" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-s64-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s64_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-float32-param" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-float32-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_float32_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-float64-param" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "list-float64-param",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_float64_param(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-u8-ret" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u8_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-u16-ret" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u16_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-u32-ret" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u32_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-u64-ret" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_u64_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s8-ret" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s8_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s16-ret" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s16_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s32-ret" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s32_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-s64-ret" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_s64_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-float32-ret" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_float32_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "list-float64-ret" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.list_float64_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "tuple-list" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "tuple-list",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.tuple_list(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "string-list-arg" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let a = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "string-list-arg",
                        key: "a",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.string_list_arg(a);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "string-list-ret" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.string_list_ret();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "tuple-string-list" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "tuple-string-list",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.tuple_string_list(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "string-list" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "string-list",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.string_list(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "record-list" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "record-list",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.record_list(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "record-list-reverse" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "record-list-reverse",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.record_list_reverse(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "variant-list" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "variant-list",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.variant_list(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "load-store-everything" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let a = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "load-store-everything",
                        key: "a",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.load_store_everything(a);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            _ => invoke.resolver.reject("Not Found"),
        }
    }
}
