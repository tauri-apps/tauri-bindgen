#[allow(clippy::all)]
pub mod imports {
    #[::tauri_bindgen_host::async_trait]
    pub trait Imports: Sized {
        async fn simple_list1(&self, l: Vec<u32>) -> ::tauri_bindgen_host::anyhow::Result<()>;
        async fn simple_list2(&self) -> ::tauri_bindgen_host::anyhow::Result<Vec<u32>>;
        async fn simple_list4(
            &self,
            l: Vec<Vec<u32>>,
        ) -> ::tauri_bindgen_host::anyhow::Result<Vec<Vec<u32>>>;
    }

    fn verfiy_idl_hash<'a, R: ::tauri_bindgen_host::tauri::Runtime>(
        item: ::tauri_bindgen_host::tauri::command::CommandItem<'a, R>,
    ) -> Result<(), ::tauri_bindgen_host::tauri::InvokeError> {
        let hash: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(item)?;

        if hash != "d40a3203ef48115d" {
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
            "simple-list1" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "simple-list1", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "simple-list1",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
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

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.simple_list1(l);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "simple-list2" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "simple-list2", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "simple-list2",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.simple_list2();

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "simple-list4" => {
                let span = ::tauri_bindgen_host::tracing::span!(
                ::tauri_bindgen_host::tracing::Level::TRACE,
                "tauri-bindgen invoke handler",
                module = "imports", function = "simple-list4", payload = ?invoke.message.payload()
                );
                let _enter = span.enter();

                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                if let Err(err) =
                    verfiy_idl_hash(::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "simple-list4",
                        key: "idlHash",
                        message: &__tauri_message__,
                    })
                {
                    return __tauri_resolver__.invoke_error(err);
                }
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

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.simple_list4(l);

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
