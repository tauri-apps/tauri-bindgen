#[allow(clippy::all)]
pub mod imports {
    pub const WORLD_HASH: &str = "e6872cf01241a6f3";
    #[repr(C)]
    #[derive(
        Debug,
        Copy,
        Clone,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    #[serde(rename_all = "camelCase")]
    pub struct Empty {}
    /// A record containing two scalar fields
    /// that both have the same type
    #[repr(C)]
    #[derive(
        Debug,
        Copy,
        Clone,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    #[serde(rename_all = "camelCase")]
    pub struct Scalars {
        /// The first field, named a
        pub a: u32,
        /// The second field, named b
        pub b: u32,
    }
    /// A record that is really just flags
    /// All of the fields are bool
    #[repr(C)]
    #[derive(
        Debug,
        Copy,
        Clone,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    #[serde(rename_all = "camelCase")]
    pub struct ReallyFlags {
        pub a: bool,
        pub b: bool,
        pub c: bool,
        pub d: bool,
        pub e: bool,
        pub f: bool,
        pub g: bool,
        pub h: bool,
        pub i: bool,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    #[serde(rename_all = "camelCase")]
    pub struct Aggregates {
        pub a: Scalars,
        pub b: u32,
        pub c: Empty,
        pub d: String,
        pub e: ReallyFlags,
    }
    pub type IntTypedef = i32;
    pub type TupleTypedef2 = (IntTypedef,);
    pub trait Imports: Sized {
        fn tuple_arg(&self, x: (char, u32)) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn tuple_result(&self) -> ::tauri_bindgen_host::anyhow::Result<(char, u32)>;
        fn empty_arg(&self, x: Empty) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn empty_result(&self) -> ::tauri_bindgen_host::anyhow::Result<Empty>;
        fn scalar_arg(&self, x: Scalars) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn scalar_result(&self) -> ::tauri_bindgen_host::anyhow::Result<Scalars>;
        fn flags_arg(&self, x: ReallyFlags) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn flags_result(&self) -> ::tauri_bindgen_host::anyhow::Result<ReallyFlags>;
        fn aggregate_arg(&self, x: Aggregates) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn aggregate_result(&self) -> ::tauri_bindgen_host::anyhow::Result<Aggregates>;
        fn typedef_inout(&self, e: TupleTypedef2) -> ::tauri_bindgen_host::anyhow::Result<i32>;
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
                "tuple-arg" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "tuple-arg",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "tuple-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.tuple_arg(x);

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "tuple-result" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.tuple_result();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "empty-arg" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "empty-arg",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "empty-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.empty_arg(x);

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "empty-result" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.empty_result();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "scalar-arg" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "scalar-arg",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "scalar-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.scalar_arg(x);

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "scalar-result" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.scalar_result();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "flags-arg" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "flags-arg",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "flags-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.flags_arg(x);

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "flags-result" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.flags_result();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "aggregate-arg" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "aggregate-arg",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "aggregate-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.aggregate_arg(x);

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "aggregate-result" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.aggregate_result();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "typedef-inout" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let e = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "typedef-inout",
                            key: "e",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "typedef-inout",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.typedef_inout(e);

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
