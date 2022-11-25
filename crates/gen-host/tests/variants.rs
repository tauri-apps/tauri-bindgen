#[allow(clippy::all)]
pub mod imports {
    pub const WORLD_HASH: &str = "d5901a6520084a85";
    #[repr(u8)]
    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    pub enum E1 {
        A,
    }
    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
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
    #[derive(
        Debug,
        Clone,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    pub enum V1 {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    pub enum Casts1 {
        A(i32),
        B(f32),
    }
    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    pub enum Casts2 {
        A(f64),
        B(f32),
    }
    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    pub enum Casts3 {
        A(f64),
        B(u64),
    }
    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    pub enum Casts4 {
        A(u32),
        B(i64),
    }
    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    pub enum Casts5 {
        A(f32),
        B(i64),
    }
    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    pub enum Casts6 {
        A((f32, u32)),
        B((u32, u32)),
    }
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, ::tauri_bindgen_host::serde::Serialize)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(
        Debug,
        Clone,
        PartialEq,
        ::tauri_bindgen_host::serde::Serialize,
        ::tauri_bindgen_host::serde::Deserialize,
    )]
    #[serde(rename_all = "camelCase")]
    pub struct IsClone {
        pub v1: V1,
    }
    pub trait Imports: Sized {
        fn e1_arg(&self, x: E1) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn e1_result(&self) -> ::tauri_bindgen_host::anyhow::Result<E1>;
        fn u1_arg(&self, x: U1) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn u1_result(&self) -> ::tauri_bindgen_host::anyhow::Result<U1>;
        fn v1_arg(&self, x: V1) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn v1_result(&self) -> ::tauri_bindgen_host::anyhow::Result<V1>;
        fn bool_arg(&self, x: bool) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn bool_result(&self) -> ::tauri_bindgen_host::anyhow::Result<bool>;
        fn option_arg(
            &self,
            a: Option<bool>,
            b: Option<()>,
            c: Option<u32>,
            d: Option<E1>,
            e: Option<f32>,
            f: Option<U1>,
            g: Option<Option<bool>>,
        ) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn option_result(
            &self,
        ) -> ::tauri_bindgen_host::anyhow::Result<(
            Option<bool>,
            Option<()>,
            Option<u32>,
            Option<E1>,
            Option<f32>,
            Option<U1>,
            Option<Option<bool>>,
        )>;
        fn casts(
            &self,
            a: Casts1,
            b: Casts2,
            c: Casts3,
            d: Casts4,
            e: Casts5,
            f: Casts6,
        ) -> ::tauri_bindgen_host::anyhow::Result<(Casts1, Casts2, Casts3, Casts4, Casts5, Casts6)>;
        fn result_arg(
            &self,
            a: Result<(), ()>,
            b: Result<(), E1>,
            c: Result<E1, ()>,
            d: Result<(), ()>,
            e: Result<u32, V1>,
            f: Result<String, Vec<u8>>,
        ) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn result_result(
            &self,
        ) -> ::tauri_bindgen_host::anyhow::Result<(
            Result<(), ()>,
            Result<(), E1>,
            Result<E1, ()>,
            Result<(), ()>,
            Result<u32, V1>,
            Result<String, Vec<u8>>,
        )>;
        fn return_result_sugar(&self)
            -> ::tauri_bindgen_host::anyhow::Result<Result<i32, MyErrno>>;
        fn return_result_sugar2(&self)
            -> ::tauri_bindgen_host::anyhow::Result<Result<(), MyErrno>>;
        fn return_result_sugar3(
            &self,
        ) -> ::tauri_bindgen_host::anyhow::Result<Result<MyErrno, MyErrno>>;
        fn return_result_sugar4(
            &self,
        ) -> ::tauri_bindgen_host::anyhow::Result<Result<(i32, u32), MyErrno>>;
        fn return_option_sugar(&self) -> ::tauri_bindgen_host::anyhow::Result<Option<i32>>;
        fn return_option_sugar2(&self) -> ::tauri_bindgen_host::anyhow::Result<Option<MyErrno>>;
        fn result_simple(&self) -> ::tauri_bindgen_host::anyhow::Result<Result<u32, i32>>;
        fn is_clone_arg(&self, a: IsClone) -> ::tauri_bindgen_host::anyhow::Result<()>;
        fn is_clone_return(&self) -> ::tauri_bindgen_host::anyhow::Result<IsClone>;
        fn return_named_option(&self) -> ::tauri_bindgen_host::anyhow::Result<Option<u8>>;
        fn return_named_result(&self) -> ::tauri_bindgen_host::anyhow::Result<Result<u8, MyErrno>>;
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
                "e1-arg" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "e1-arg",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "e1-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.e1_arg(x);

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "e1-result" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.e1_result();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "u1-arg" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "u1-arg",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "u1-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.u1_arg(x);

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "u1-result" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.u1_result();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "v1-arg" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "v1-arg",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "v1-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.v1_arg(x);

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "v1-result" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.v1_result();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "bool-arg" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "bool-arg",
                            key: "x",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "bool-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.bool_arg(x);

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "bool-result" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.bool_result();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "option-arg" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let a = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "option-arg",
                            key: "a",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "option-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let b = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "option-arg",
                            key: "b",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "option-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let c = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "option-arg",
                            key: "c",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "option-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let d = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "option-arg",
                            key: "d",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "option-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let e = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "option-arg",
                            key: "e",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "option-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let f = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "option-arg",
                            key: "f",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "option-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let g = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "option-arg",
                            key: "g",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "option-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.option_arg(a, b, c, d, e, f, g);

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "option-result" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.option_result();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "casts" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let a = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "casts",
                            key: "a",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "casts",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let b = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "casts",
                            key: "b",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "casts",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let c = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "casts",
                            key: "c",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "casts",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let d = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "casts",
                            key: "d",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "casts",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let e = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "casts",
                            key: "e",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "casts",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let f = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "casts",
                            key: "f",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "casts",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.casts(a, b, c, d, e, f);

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "result-arg" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let a = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "result-arg",
                            key: "a",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "result-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let b = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "result-arg",
                            key: "b",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "result-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let c = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "result-arg",
                            key: "c",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "result-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let d = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "result-arg",
                            key: "d",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "result-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let e = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "result-arg",
                            key: "e",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "result-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let f = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "result-arg",
                            key: "f",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "result-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.result_arg(a, b, c, d, e, f);

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "result-result" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.result_result();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "return-result-sugar" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.return_result_sugar();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "return-result-sugar2" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.return_result_sugar2();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "return-result-sugar3" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.return_result_sugar3();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "return-result-sugar4" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.return_result_sugar4();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "return-option-sugar" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.return_option_sugar();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "return-option-sugar2" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.return_option_sugar2();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "result-simple" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.result_simple();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "is-clone-arg" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let a = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                        ::tauri_bindgen_host::tauri::command::CommandItem {
                            name: "is-clone-arg",
                            key: "a",
                            message: &__tauri_message__,
                        },
                    ) {
                        Ok(arg) => arg,
                        Err(err) => {
                            ::tauri_bindgen_host::tracing::error!(
                                module = "imports",
                                function = "is-clone-arg",
                                "Invoke handler returned error {:?}",
                                err
                            );
                            return __tauri_resolver__.invoke_error(err);
                        }
                    };

                    let result = ctx.is_clone_arg(a);

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "is-clone-return" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.is_clone_return();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "return-named-option" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.return_named_option();

                    __tauri_resolver__.respond(
                        result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow),
                    );
                }
                "return-named-result" => {
                    #[allow(unused_variables)]
                    let ::tauri_bindgen_host::tauri::Invoke {
                        message: __tauri_message__,
                        resolver: __tauri_resolver__,
                    } = invoke;
                    let result = ctx.return_named_result();

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
