#[allow(clippy::all)]
pub mod multi_return {
    pub const WORLD_HASH: &str = "def17a258c1e4f4d";
    pub trait MultiReturn: Sized {
        fn mra(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn mrb(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn mrc(&self) -> ::tauri_bindgen_host::anyhow::Result<u32>;
        fn mrd(&self) -> ::tauri_bindgen_host::anyhow::Result<u32>;
        fn mre(&self) -> ::tauri_bindgen_host::anyhow::Result<(u32, f32)>;
    }

    pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
    where
        U: MultiReturn + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime + 'static,
    {
        move |invoke| {
            let span = ::tauri_bindgen_host::tracing::span!(
            ::tauri_bindgen_host::tracing::Level::TRACE,
            "tauri-bindgen invoke handler",
            module = "multi-return", function = invoke.message.command(), payload = ?invoke.message.payload()
            );
            let _enter = span.enter();

            match invoke.message.command() {
                "mra" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.mra();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "mrb" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.mrb();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "mrc" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.mrc();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "mrd" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.mrd();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "mre" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.mre();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                func_name => {
                    ::tauri_bindgen_host::tracing::error!(
                        module = "multi-return",
                        function = func_name,
                        "Not Found"
                    );
                    invoke.resolver.reject("Not Found")
                }
            }
        }
    }
}
