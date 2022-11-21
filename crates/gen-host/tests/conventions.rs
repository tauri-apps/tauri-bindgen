#[allow(clippy::all)]
pub mod imports {
    #[repr(C)]
    #[derive(Debug, Copy, Clone, ::serde::Serialize, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LudicrousSpeed {
        pub how_fast_are_you_going: u32,
        pub i_am_going_extremely_slow: u64,
    }
    pub trait Imports: Sized {
        fn kebab_case(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn foo(&self, x: LudicrousSpeed) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn function_with_dashes(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn function_with_no_weird_characters(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn apple(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn apple_pear(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn apple_pear_grape(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn a0(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn is_xml(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn explicit(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn explicit_kebab(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn bool(&self) -> ::tauri_bindgen_host::anyhow::Result<()>;
    }

    pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
    where
        U: Imports + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime,
    {
        move |invoke| match invoke.message.command() {
            "kebab-case" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.kebab_case();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "foo" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.foo(
                    match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "foo",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => return __tauri_resolver__.invoke_error(err),
                    },
                );

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "function-with-dashes" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.function_with_dashes();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "function-with-no-weird-characters" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.function_with_no_weird_characters();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "apple" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.apple();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "apple-pear" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.apple_pear();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "apple-pear-grape" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.apple_pear_grape();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "a0" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.a0();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "is-XML" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.is_xml();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "explicit" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.explicit();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "explicit-kebab" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.explicit_kebab();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            "bool" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let result = ctx.bool();

                __tauri_resolver__
                    .respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
            }
            _ => invoke.resolver.reject("Not Found"),
        }
    }
}
