#[allow(clippy::all)]
pub mod imports {
    pub const WORLD_HASH: &str = "d40a3203";
    pub trait Imports: Sized {
        fn simple_list1(&self, l: Vec<u32>) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn simple_list2(&self) -> ::tauri_bindgen_host::anyhow::Result<Vec<u32>>;
        fn simple_list4(
            &self,
            l: Vec<Vec<u32>>,
        ) -> ::tauri_bindgen_host::anyhow::Result<Vec<Vec<u32>>>;
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
                "simple-list1" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let l = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "simple-list1",
                            key: "l",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "simple-list1",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.simple_list1(l);

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "simple-list2" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.simple_list2();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "simple-list4" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let l = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "simple-list4",
                            key: "l",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "simple-list4",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.simple_list4(l);

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
