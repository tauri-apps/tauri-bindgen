#[allow(clippy::all)]
pub mod import_flags {
    ::tauri_bindgen_host::bitflags::bitflags! {
      #[derive(::tauri_bindgen_host::serde::Serialize, ::tauri_bindgen_host::serde::Deserialize)]
      pub struct Flag1: u8 {
        const B0 = 1 << 0;
      }
    }
    ::tauri_bindgen_host::bitflags::bitflags! {
      #[derive(::tauri_bindgen_host::serde::Serialize, ::tauri_bindgen_host::serde::Deserialize)]
      pub struct Flag2: u8 {
        const B0 = 1 << 0;
        const B1 = 1 << 1;
      }
    }
    ::tauri_bindgen_host::bitflags::bitflags! {
      #[derive(::tauri_bindgen_host::serde::Serialize, ::tauri_bindgen_host::serde::Deserialize)]
      pub struct Flag4: u8 {
        const B0 = 1 << 0;
        const B1 = 1 << 1;
        const B2 = 1 << 2;
        const B3 = 1 << 3;
      }
    }
    ::tauri_bindgen_host::bitflags::bitflags! {
      #[derive(::tauri_bindgen_host::serde::Serialize, ::tauri_bindgen_host::serde::Deserialize)]
      pub struct Flag8: u8 {
        const B0 = 1 << 0;
        const B1 = 1 << 1;
        const B2 = 1 << 2;
        const B3 = 1 << 3;
        const B4 = 1 << 4;
        const B5 = 1 << 5;
        const B6 = 1 << 6;
        const B7 = 1 << 7;
      }
    }
    ::tauri_bindgen_host::bitflags::bitflags! {
      #[derive(::tauri_bindgen_host::serde::Serialize, ::tauri_bindgen_host::serde::Deserialize)]
      pub struct Flag16: u16 {
        const B0 = 1 << 0;
        const B1 = 1 << 1;
        const B2 = 1 << 2;
        const B3 = 1 << 3;
        const B4 = 1 << 4;
        const B5 = 1 << 5;
        const B6 = 1 << 6;
        const B7 = 1 << 7;
        const B8 = 1 << 8;
        const B9 = 1 << 9;
        const B10 = 1 << 10;
        const B11 = 1 << 11;
        const B12 = 1 << 12;
        const B13 = 1 << 13;
        const B14 = 1 << 14;
        const B15 = 1 << 15;
      }
    }
    ::tauri_bindgen_host::bitflags::bitflags! {
      #[derive(::tauri_bindgen_host::serde::Serialize, ::tauri_bindgen_host::serde::Deserialize)]
      pub struct Flag32: u32 {
        const B0 = 1 << 0;
        const B1 = 1 << 1;
        const B2 = 1 << 2;
        const B3 = 1 << 3;
        const B4 = 1 << 4;
        const B5 = 1 << 5;
        const B6 = 1 << 6;
        const B7 = 1 << 7;
        const B8 = 1 << 8;
        const B9 = 1 << 9;
        const B10 = 1 << 10;
        const B11 = 1 << 11;
        const B12 = 1 << 12;
        const B13 = 1 << 13;
        const B14 = 1 << 14;
        const B15 = 1 << 15;
        const B16 = 1 << 16;
        const B17 = 1 << 17;
        const B18 = 1 << 18;
        const B19 = 1 << 19;
        const B20 = 1 << 20;
        const B21 = 1 << 21;
        const B22 = 1 << 22;
        const B23 = 1 << 23;
        const B24 = 1 << 24;
        const B25 = 1 << 25;
        const B26 = 1 << 26;
        const B27 = 1 << 27;
        const B28 = 1 << 28;
        const B29 = 1 << 29;
        const B30 = 1 << 30;
        const B31 = 1 << 31;
      }
    }
    ::tauri_bindgen_host::bitflags::bitflags! {
      #[derive(::tauri_bindgen_host::serde::Serialize, ::tauri_bindgen_host::serde::Deserialize)]
      pub struct Flag64: u64 {
        const B0 = 1 << 0;
        const B1 = 1 << 1;
        const B2 = 1 << 2;
        const B3 = 1 << 3;
        const B4 = 1 << 4;
        const B5 = 1 << 5;
        const B6 = 1 << 6;
        const B7 = 1 << 7;
        const B8 = 1 << 8;
        const B9 = 1 << 9;
        const B10 = 1 << 10;
        const B11 = 1 << 11;
        const B12 = 1 << 12;
        const B13 = 1 << 13;
        const B14 = 1 << 14;
        const B15 = 1 << 15;
        const B16 = 1 << 16;
        const B17 = 1 << 17;
        const B18 = 1 << 18;
        const B19 = 1 << 19;
        const B20 = 1 << 20;
        const B21 = 1 << 21;
        const B22 = 1 << 22;
        const B23 = 1 << 23;
        const B24 = 1 << 24;
        const B25 = 1 << 25;
        const B26 = 1 << 26;
        const B27 = 1 << 27;
        const B28 = 1 << 28;
        const B29 = 1 << 29;
        const B30 = 1 << 30;
        const B31 = 1 << 31;
        const B32 = 1 << 32;
        const B33 = 1 << 33;
        const B34 = 1 << 34;
        const B35 = 1 << 35;
        const B36 = 1 << 36;
        const B37 = 1 << 37;
        const B38 = 1 << 38;
        const B39 = 1 << 39;
        const B40 = 1 << 40;
        const B41 = 1 << 41;
        const B42 = 1 << 42;
        const B43 = 1 << 43;
        const B44 = 1 << 44;
        const B45 = 1 << 45;
        const B46 = 1 << 46;
        const B47 = 1 << 47;
        const B48 = 1 << 48;
        const B49 = 1 << 49;
        const B50 = 1 << 50;
        const B51 = 1 << 51;
        const B52 = 1 << 52;
        const B53 = 1 << 53;
        const B54 = 1 << 54;
        const B55 = 1 << 55;
        const B56 = 1 << 56;
        const B57 = 1 << 57;
        const B58 = 1 << 58;
        const B59 = 1 << 59;
        const B60 = 1 << 60;
        const B61 = 1 << 61;
        const B62 = 1 << 62;
        const B63 = 1 << 63;
      }
    }
    #[::tauri_bindgen_host::async_trait]
    pub trait ImportFlags: Sized {
        async fn roundtrip_flag1(&self, x: Flag1) -> ::tauri_bindgen_host::anyhow::Result<Flag1>;
        async fn roundtrip_flag2(&self, x: Flag2) -> ::tauri_bindgen_host::anyhow::Result<Flag2>;
        async fn roundtrip_flag4(&self, x: Flag4) -> ::tauri_bindgen_host::anyhow::Result<Flag4>;
        async fn roundtrip_flag8(&self, x: Flag8) -> ::tauri_bindgen_host::anyhow::Result<Flag8>;
        async fn roundtrip_flag16(&self, x: Flag16)
            -> ::tauri_bindgen_host::anyhow::Result<Flag16>;
        async fn roundtrip_flag32(&self, x: Flag32)
            -> ::tauri_bindgen_host::anyhow::Result<Flag32>;
        async fn roundtrip_flag64(&self, x: Flag64)
            -> ::tauri_bindgen_host::anyhow::Result<Flag64>;
    }

    pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
    where
        U: ImportFlags + Copy + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime,
    {
        move |invoke| match invoke.message.command() {
            "roundtrip-flag1" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "roundtrip-flag1",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.roundtrip_flag1(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "roundtrip-flag2" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "roundtrip-flag2",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.roundtrip_flag2(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "roundtrip-flag4" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "roundtrip-flag4",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.roundtrip_flag4(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "roundtrip-flag8" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "roundtrip-flag8",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.roundtrip_flag8(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "roundtrip-flag16" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "roundtrip-flag16",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.roundtrip_flag16(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "roundtrip-flag32" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "roundtrip-flag32",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.roundtrip_flag32(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            "roundtrip-flag64" => {
                #[allow(unused_variables)]
                let ::tauri_bindgen_host::tauri::Invoke {
                    message: __tauri_message__,
                    resolver: __tauri_resolver__,
                } = invoke;

                let x = match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(
                    ::tauri_bindgen_host::tauri::command::CommandItem {
                        name: "roundtrip-flag64",
                        key: "x",
                        message: &__tauri_message__,
                    },
                ) {
                    Ok(arg) => arg,
                    Err(err) => return __tauri_resolver__.invoke_error(err),
                };

                __tauri_resolver__.respond_async(async move {
                    let result = ctx.roundtrip_flag64(x);

                    result
                        .await
                        .map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow)
                });
            }
            _ => invoke.resolver.reject("Not Found"),
        }
    }
}
