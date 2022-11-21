#[allow(clippy::all)]
pub mod imports {
    #[derive(Debug, Clone, ::tauri_bindgen_host::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
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
    pub trait Imports: Sized {
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
        ) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn big_argument(&self, x: BigStruct) -> ::tauri_bindgen_host::anyhow::Result<()>;
    }

    pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
    where
        U: Imports + Copy + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime,
    {
        move |invoke| match invoke.message.command() {
            "many-args" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let a1 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a1",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let a2 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a2",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let a3 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a3",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let a4 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a4",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let a5 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a5",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let a6 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a6",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let a7 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a7",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let a8 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a8",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let a9 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a9",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let a10 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a10",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let a11 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a11",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let a12 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a12",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let a13 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a13",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let a14 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a14",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let a15 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a15",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let a16 = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "many-args",
                        key: "a16",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.many_args(
                        a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16,
                    );

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "big-argument" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "big-argument",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.big_argument(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            _ => invoke.resolver.reject("Not Found"),
        }
    }
}
