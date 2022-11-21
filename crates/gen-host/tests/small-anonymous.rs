#[allow(clippy::all)]
pub mod imports {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, ::tauri_bindgen_host::serde::Serialize)]
    pub enum Error {
        Success,
        Failure,
    }
    #[::tauri_bindgen_host::async_trait]
    pub trait Imports: Sized {
        async fn option_test(
            &self,
        ) -> ::tauri_bindgen_host::anyhow::Result<Result<Option<String>, Error>>;
    }

    pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
    where
        U: Imports + Copy + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime,
    {
        move |invoke| match invoke.message.command() {
            "option-test" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.option_test();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            _ => invoke.resolver.reject("Not Found"),
        }
    }
}
