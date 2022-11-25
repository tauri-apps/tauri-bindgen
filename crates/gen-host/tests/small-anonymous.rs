#[allow(clippy::all)]
pub mod imports {
    pub const WORLD_HASH: &str = "bee731db";
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, ::tauri_bindgen_host::serde::Serialize)]
    pub enum Error {
        Success,
        Failure,
    }
    pub trait Imports: Sized {
        fn option_test(
            &self,
        ) -> ::tauri_bindgen_host::anyhow::Result<Result<Option<String>, Error>>;
    }

    pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
    where
        U: Imports + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime + 'static,
    {
        move |invoke| {
            let span = ::tauri_bindgen_host::tracing::span!(
            ::tauri_bindgen_host::tracing::Level::TRACE,
            "tauri-bindgen invoke handler",
            module = "imports", function = invoke.message.command(), payload = ?invoke.message.payload()
            );
            let _enter = span.enter();

            match invoke.message.command() {
                "option-test" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.option_test();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
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
}
