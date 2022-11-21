#[allow(clippy::all)]
pub mod imports {
    #[::tauri_bindgen_host::async_trait]
    pub trait Imports: Sized {
        /// A function that accepts a character
        async fn take_char(&self, x: char) -> ::tauri_bindgen_host::anyhow::Result<()>;
        /// A function that returns a character
        async fn return_char(&self) -> ::tauri_bindgen_host::anyhow::Result<char>;
    }

    pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
    where
        U: Imports + Copy + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime,
    {
        move |invoke| match invoke.message.command() {
            "take-char" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "take-char",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.take_char(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "return-char" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.return_char();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            _ => invoke.resolver.reject("Not Found"),
        }
    }
}
