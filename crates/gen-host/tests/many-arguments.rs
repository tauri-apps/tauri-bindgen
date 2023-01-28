#[allow(clippy::all, unused)]
#[rustfmt::skip]
pub mod many_arguments{
  use ::tauri_bindgen_host::tauri_bindgen_abi;
  pub const WORLD_HASH: &str = "b26e5107ff225c6b";
  #[derive(Debug, Clone, PartialEq)]
  #[derive(tauri_bindgen_abi::Readable)]
  pub struct BigStruct {
    pub a1: String,
    pub a2: String,
    pub a3: String,
    pub a4: String,
    pub a5: String,
    pub a6: String,
    pub a7: String,
    pub a8: String,
    pub a9: String,
    pub a10: String,
    pub a11: String,
    pub a12: String,
    pub a13: String,
    pub a14: String,
    pub a15: String,
    pub a16: String,
    pub a17: String,
    pub a18: String,
    pub a19: String,
    pub a20: String,
  }
  pub trait ManyArguments: Sized {
    fn many_args(&self,a1: u64,a2: u64,a3: u64,a4: u64,a5: u64,a6: u64,a7: u64,a8: u64,a9: u64,a10: u64,a11: u64,a12: u64,a13: u64,a14: u64,a15: u64,a16: u64,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    fn big_argument(&self,x: BigStruct,) -> ::tauri_bindgen_host::anyhow::Result<()>;
  }
  
  pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
  where
  U: ManyArguments+ Send + Sync + 'static,
  R: ::tauri_bindgen_host::tauri::Runtime + 'static
  {
    
    move |invoke| {
      
      let span = ::tauri_bindgen_host::tracing::span!(
      ::tauri_bindgen_host::tracing::Level::TRACE,
      "tauri-bindgen invoke handler",
      module = "many-arguments", function = invoke.message.command(), payload = ?invoke.message.payload()
      );
      let _enter = span.enter();
      
      match invoke.message.command() {
        "many-args" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            a1 : u64,
            a2 : u64,
            a3 : u64,
            a4 : u64,
            a5 : u64,
            a6 : u64,
            a7 : u64,
            a8 : u64,
            a9 : u64,
            a10 : u64,
            a11 : u64,
            a12 : u64,
            a13 : u64,
            a14 : u64,
            a15 : u64,
            a16 : u64,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "many-args",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.many_args(
          params.a1, params.a2, params.a3, params.a4, params.a5, params.a6, params.a7, params.a8, params.a9, params.a10, params.a11, params.a12, params.a13, params.a14, params.a15, params.a16, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "big-argument" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            x : BigStruct,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "big-argument",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.big_argument(
          params.x, );
          
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
