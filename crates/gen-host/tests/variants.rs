#[allow(clippy::all, unused)]
#[rustfmt::skip]
pub mod variants{
  use ::tauri_bindgen_host::tauri_bindgen_abi;
  pub const WORLD_HASH: &str = "8178d1f91285bbc1";
  #[repr(u8)]
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
  pub enum E1{
    A,
  }
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
  pub enum U1{
    U32(u32),
    F32(f32),
  }
  #[repr(C)]
  #[derive(Debug, Copy, Clone, PartialEq)]
  #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
  pub struct Empty {
  }
  #[derive(Debug, Clone, PartialEq)]
  #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
  pub enum V1{
    A,
    B(U1),
    C(E1),
    D(String),
    E(Empty),
    F,
    G(u32),
  }
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
  pub enum Casts1{
    A(i32),
    B(f32),
  }
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
  pub enum Casts2{
    A(f64),
    B(f32),
  }
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
  pub enum Casts3{
    A(f64),
    B(u64),
  }
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
  pub enum Casts4{
    A(u32),
    B(i64),
  }
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
  pub enum Casts5{
    A(f32),
    B(i64),
  }
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
  pub enum Casts6{
    A((f32,u32,)),
    B((u32,u32,)),
  }
  #[repr(u8)]
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(tauri_bindgen_abi::Writable)]
  pub enum MyErrno{
    Bad1,
    Bad2,
  }
  #[derive(Debug, Clone, PartialEq)]
  #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
  pub struct IsClone {
    pub v1: V1,
  }
  pub trait Variants: Sized {
    fn e1_arg(&self,x: E1,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn e1_result(&self,) -> ::tauri_bindgen_host::anyhow::Result<E1>;
    fn u1_arg(&self,x: U1,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn u1_result(&self,) -> ::tauri_bindgen_host::anyhow::Result<U1>;
    fn v1_arg(&self,x: V1,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn v1_result(&self,) -> ::tauri_bindgen_host::anyhow::Result<V1>;
    fn bool_arg(&self,x: bool,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn bool_result(&self,) -> ::tauri_bindgen_host::anyhow::Result<bool>;
    fn option_arg(&self,a: Option<bool>,b: Option<()>,c: Option<u32>,d: Option<E1>,e: Option<f32>,f: Option<U1>,g: Option<Option<bool>>,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn option_result(&self,) -> ::tauri_bindgen_host::anyhow::Result<(Option<bool>,Option<()>,Option<u32>,Option<E1>,Option<f32>,Option<U1>,Option<Option<bool>>,)>;
    fn casts(&self,a: Casts1,b: Casts2,c: Casts3,d: Casts4,e: Casts5,f: Casts6,) -> ::tauri_bindgen_host::anyhow::Result<(Casts1,Casts2,Casts3,Casts4,Casts5,Casts6,)>;
    fn result_arg(&self,a: Result<(),()>,b: Result<(),E1>,c: Result<E1,()>,d: Result<(),()>,e: Result<u32,V1>,f: Result<String,Vec<u8>>,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn result_result(&self,) -> ::tauri_bindgen_host::anyhow::Result<(Result<(),()>,Result<(),E1>,Result<E1,()>,Result<(),()>,Result<u32,V1>,Result<String,Vec<u8>>,)>;
    fn return_result_sugar(&self,) -> ::tauri_bindgen_host::anyhow::Result<Result<i32,MyErrno>>;
    fn return_result_sugar2(&self,) -> ::tauri_bindgen_host::anyhow::Result<Result<(),MyErrno>>;
    fn return_result_sugar3(&self,) -> ::tauri_bindgen_host::anyhow::Result<Result<MyErrno,MyErrno>>;
    fn return_result_sugar4(&self,) -> ::tauri_bindgen_host::anyhow::Result<Result<(i32,u32,),MyErrno>>;
    fn return_option_sugar(&self,) -> ::tauri_bindgen_host::anyhow::Result<Option<i32>>;
    fn return_option_sugar2(&self,) -> ::tauri_bindgen_host::anyhow::Result<Option<MyErrno>>;
    fn result_simple(&self,) -> ::tauri_bindgen_host::anyhow::Result<Result<u32,i32>>;
    fn is_clone_arg(&self,a: IsClone,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn is_clone_return(&self,) -> ::tauri_bindgen_host::anyhow::Result<IsClone>;
    fn return_named_option(&self,) -> ::tauri_bindgen_host::anyhow::Result<Option<u8>>;
    fn return_named_result(&self,) -> ::tauri_bindgen_host::anyhow::Result<Result<u8,MyErrno>>;
  }
  
  pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
  where
  U: Variants+ Send + Sync + 'static,
  R: ::tauri_bindgen_host::tauri::Runtime + 'static
  {
    
    move |invoke| {
      
      let span = ::tauri_bindgen_host::tracing::span!(
      ::tauri_bindgen_host::tracing::Level::TRACE,
      "tauri-bindgen invoke handler",
      module = "variants", function = invoke.message.command(), payload = ?invoke.message.payload()
      );
      let _enter = span.enter();
      
      match invoke.message.command() {
        "e1-arg" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            x : E1,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "e1-arg",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.e1_arg(
          params.x, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "e1-result" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "e1-result",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.e1_result(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "u1-arg" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            x : U1,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "u1-arg",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.u1_arg(
          params.x, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "u1-result" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "u1-result",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.u1_result(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "v1-arg" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            x : V1,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "v1-arg",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.v1_arg(
          params.x, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "v1-result" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "v1-result",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.v1_result(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "bool-arg" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            x : bool,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "bool-arg",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.bool_arg(
          params.x, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "bool-result" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "bool-result",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.bool_result(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "option-arg" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            a : Option<bool>,
            b : Option<()>,
            c : Option<u32>,
            d : Option<E1>,
            e : Option<f32>,
            f : Option<U1>,
            g : Option<Option<bool>>,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "option-arg",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.option_arg(
          params.a, params.b, params.c, params.d, params.e, params.f, params.g, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "option-result" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "option-result",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.option_result(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "casts" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            a : Casts1,
            b : Casts2,
            c : Casts3,
            d : Casts4,
            e : Casts5,
            f : Casts6,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "casts",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.casts(
          params.a, params.b, params.c, params.d, params.e, params.f, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "result-arg" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            a : Result<(),()>,
            b : Result<(),E1>,
            c : Result<E1,()>,
            d : Result<(),()>,
            e : Result<u32,V1>,
            f : Result<String,Vec<u8>>,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "result-arg",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.result_arg(
          params.a, params.b, params.c, params.d, params.e, params.f, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "result-result" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "result-result",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.result_result(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "return-result-sugar" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "return-result-sugar",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.return_result_sugar(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "return-result-sugar2" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "return-result-sugar2",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.return_result_sugar2(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "return-result-sugar3" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "return-result-sugar3",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.return_result_sugar3(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "return-result-sugar4" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "return-result-sugar4",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.return_result_sugar4(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "return-option-sugar" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "return-option-sugar",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.return_option_sugar(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "return-option-sugar2" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "return-option-sugar2",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.return_option_sugar2(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "result-simple" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "result-simple",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.result_simple(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "is-clone-arg" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            a : IsClone,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "is-clone-arg",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.is_clone_arg(
          params.a, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "is-clone-return" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "is-clone-return",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.is_clone_return(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "return-named-option" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "return-named-option",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.return_named_option(
          );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "return-named-result" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "return-named-result",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.return_named_result(
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
