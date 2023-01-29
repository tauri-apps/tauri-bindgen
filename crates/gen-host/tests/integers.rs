#[allow(clippy::all, unused)]
#[rustfmt::skip]
pub mod integers{
  use ::tauri_bindgen_host::tauri_bindgen_abi;
  pub const WORLD_HASH: &str = "0c0ef23cf9639264";
  pub trait Integers: Sized {
    fn a1(&self,x: u8,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn a2(&self,x: i8,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn a3(&self,x: u16,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn a4(&self,x: i16,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn a5(&self,x: u32,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn a6(&self,x: i32,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn a7(&self,x: u64,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn a8(&self,x: i64,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn a9(&self,p1: u8,p2: i8,p3: u16,p4: i16,p5: u32,p6: i32,p7: u64,p8: i64,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn r1(&self,) -> ::tauri_bindgen_host::anyhow::Result<u8>;
    fn r2(&self,) -> ::tauri_bindgen_host::anyhow::Result<i8>;
    fn r3(&self,) -> ::tauri_bindgen_host::anyhow::Result<u16>;
    fn r4(&self,) -> ::tauri_bindgen_host::anyhow::Result<i16>;
    fn r5(&self,) -> ::tauri_bindgen_host::anyhow::Result<u32>;
    fn r6(&self,) -> ::tauri_bindgen_host::anyhow::Result<i32>;
    fn r7(&self,) -> ::tauri_bindgen_host::anyhow::Result<u64>;
    fn r8(&self,) -> ::tauri_bindgen_host::anyhow::Result<i64>;
    fn pair_ret(&self,) -> ::tauri_bindgen_host::anyhow::Result<(i64,u8,)>;
  }
  
  pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
  where
  U: Integers+ Send + Sync + 'static,
  R: ::tauri_bindgen_host::tauri::Runtime + 'static
  {
    
    move |invoke| {
      
      let span = ::tauri_bindgen_host::tracing::span!(
      ::tauri_bindgen_host::tracing::Level::TRACE,
      "tauri-bindgen invoke handler",
      module = "integers", function = invoke.message.command(), payload = ?invoke.message.payload()
      );
      let _enter = span.enter();
      
      match invoke.message.command() {
        "a1" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            x : u8,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "a1",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.a1(
          params.x, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "a2" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            x : i8,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "a2",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.a2(
          params.x, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "a3" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            x : u16,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "a3",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.a3(
          params.x, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "a4" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            x : i16,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "a4",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.a4(
          params.x, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "a5" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            x : u32,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "a5",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.a5(
          params.x, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "a6" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            x : i32,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "a6",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.a6(
          params.x, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "a7" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            x : u64,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "a7",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.a7(
          params.x, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "a8" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            x : i64,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "a8",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.a8(
          params.x, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "a9" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            p1 : u8,
            p2 : i8,
            p3 : u16,
            p4 : i16,
            p5 : u32,
            p6 : i32,
            p7 : u64,
            p8 : i64,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "a9",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.a9(
          params.p1, params.p2, params.p3, params.p4, params.p5, params.p6, params.p7, params.p8, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "r1" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "r1",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.r1(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "r2" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "r2",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.r2(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "r3" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "r3",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.r3(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "r4" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "r4",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.r4(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "r5" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "r5",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.r5(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "r6" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "r6",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.r6(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "r7" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "r7",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.r7(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "r8" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "r8",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.r8(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "pair-ret" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "pair-ret",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.pair_ret(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        _ => todo!(),
      }
    }
  }
  
}
