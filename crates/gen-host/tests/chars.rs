#[allow(clippy::all, unused)]
#[rustfmt::skip]
pub mod chars{
  use ::tauri_bindgen_host::tauri_bindgen_abi;
  pub const WORLD_HASH: &str = "58d944fc9a2c8431";
  pub trait Chars: Sized {
    /// A function that accepts a character
    fn take_char(&self,x: char,) -> ::tauri_bindgen_host::anyhow::Result<()>;
    /// A function that returns a character
    fn return_char(&self,) -> ::tauri_bindgen_host::anyhow::Result<char>;
  }
  
  pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
  where
  U: Chars+ Send + Sync + 'static,
  R: ::tauri_bindgen_host::tauri::Runtime + 'static
  {
    
    move |invoke| {
      
      let span = ::tauri_bindgen_host::tracing::span!(
      ::tauri_bindgen_host::tracing::Level::TRACE,
      "tauri-bindgen invoke handler",
      module = "chars", function = invoke.message.command(), payload = ?invoke.message.payload()
      );
      let _enter = span.enter();
      
      match invoke.message.command() {
        "take-char" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
            x : char,
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "take-char",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.take_char(
          params.x, );
          
          __tauri_resolver__.respond(result
          .map(|ref val| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(val).unwrap()))
          .map_err(|ref err| ::tauri_bindgen_host::encode_base64(&::tauri_bindgen_host::tauri_bindgen_abi::to_bytes(&err.to_string()).unwrap()).into())
          );
        },
        "return-char" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          #[derive(tauri_bindgen_abi::Readable)]
          struct Params {
          }
          
          let message: String = ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "return-char",
            key: "encoded",
            message: &__tauri_message__,
          }).unwrap();
          let message = ::tauri_bindgen_host::decode_base64(&message);
          let params: Params = ::tauri_bindgen_host::tauri_bindgen_abi::from_slice(&message).unwrap();
          
          let result = ctx.return_char(
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