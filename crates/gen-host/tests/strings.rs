#[allow(clippy::all)]
pub mod imports {
    #[::tauri_bindgen_host::async_trait]
    pub trait Imports: Sized {
        async fn a(&self, x: String) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn b(&self) -> ::tauri_bindgen_host::anyhow::Result<String>;
        async fn c(&self, a: String, b: String) -> ::tauri_bindgen_host::anyhow::Result<String>;
    }

    pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
    where
        U: Imports + Copy + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime,
    {
        move |invoke| match invoke.message.command() {
            "a" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "a",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.a(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "b" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.b();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "c" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let a = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "c",
                        key: "a",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                let b = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "c",
                        key: "b",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.c(a, b);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            _ => invoke.resolver.reject("Not Found"),
        }
    }
}
