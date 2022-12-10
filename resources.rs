#[allow(clippy::all)]
pub mod resources {
    pub const WORLD_HASH: &str = "52ed1e51326e63d9";
    /// regular resource
    pub trait C: Sized {
        /// regular function with empty signature
        fn foo(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
        /// regular function with primitives
        fn bar(name: String) -> ::tauri_bindgen_host::anyhow::Result<String>;
        /// static function
        fn fiz() -> ::tauri_bindgen_host::anyhow::Result<String>;
    }
    /// regular resource without static functions that appears in function (this type will be generated)
    pub trait F: Sized {
        /// regular function with empty signature
        fn foo(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
    }
    pub trait Resources: Sized {
        /// regular resource
        type C: C;
        /// regular resource without static functions that appears in function (this type will be generated)
        type F: F;
        fn roundtrip(&self, x: Self::F) -> ::tauri_bindgen_host::anyhow::Result<Self::F>;
    }

    pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
    where
        U: Resources + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime + 'static,
    {
        move |invoke| match invoke.message.command() {
            "roundtrip" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "roundtrip",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => {
                        return __tauri_resolver__.invoke_error(err);
                    }
                };

                let result = ctx.roundtrip(x);

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "c.foo" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                let resource_id =
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "c.foo",
                            key: "__id",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };
            }
            "c.bar" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                let resource_id =
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "c.bar",
                            key: "__id",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };
            }
            "c.fiz" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                let resource_id =
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "c.fiz",
                            key: "__id",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };
            }
            "f.foo" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;
                let resource_id =
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "f.foo",
                            key: "__id",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };
            }
            func_name => invoke.resolver.reject("Not Found"),
        }
    }
}
