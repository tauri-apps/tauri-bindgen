
#![allow(clippy::all, unused)]
pub mod unions{
  pub const WORLD_HASH: &str = "37da362e4966fe5b";
  /// A union of all of the integral types
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(::tauri_bindgen_host::serde::Deserialize, ::tauri_bindgen_host::serde::Serialize)]
  pub enum AllIntegers{
    /// Bool is equivalent to a 1 bit integer and is treated that way in some languages
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
  }
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(::tauri_bindgen_host::serde::Deserialize, ::tauri_bindgen_host::serde::Serialize)]
  pub enum AllFloats{
    F32(f32),
    F64(f64),
  }
  #[derive(Debug, Clone, PartialEq)]
  #[derive(::tauri_bindgen_host::serde::Deserialize, ::tauri_bindgen_host::serde::Serialize)]
  pub enum AllText{
    Char(char),
    String(String),
  }
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(::tauri_bindgen_host::serde::Deserialize, ::tauri_bindgen_host::serde::Serialize)]
  pub enum DuplicatedS32{
    /// The first s32
    I320(i32),
    /// The second s32
    I321(i32),
    /// The third s32
    I322(i32),
  }
  /// A type containing numeric types that are distinct in most languages
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(::tauri_bindgen_host::serde::Deserialize, ::tauri_bindgen_host::serde::Serialize)]
  pub enum DistinguishableNum{
    /// A Floating Point Number
    F64(f64),
    /// A Signed Integer
    I64(i64),
  }
  pub trait Unions: Sized {
    fn add_one_integer(&self,num: AllIntegers,) -> ::tauri_bindgen_host::anyhow::Result<AllIntegers>;
    fn add_one_float(&self,num: AllFloats,) -> ::tauri_bindgen_host::anyhow::Result<AllFloats>;
    fn replace_first_char(&self,text: AllText,letter: char,) -> ::tauri_bindgen_host::anyhow::Result<AllText>;
    fn identify_integer(&self,num: AllIntegers,) -> ::tauri_bindgen_host::anyhow::Result<u8>;
    fn identify_float(&self,num: AllFloats,) -> ::tauri_bindgen_host::anyhow::Result<u8>;
    fn identify_text(&self,text: AllText,) -> ::tauri_bindgen_host::anyhow::Result<u8>;
    fn add_one_duplicated(&self,num: DuplicatedS32,) -> ::tauri_bindgen_host::anyhow::Result<DuplicatedS32>;
    fn identify_duplicated(&self,num: DuplicatedS32,) -> ::tauri_bindgen_host::anyhow::Result<u8>;
    fn add_one_distinguishable_num(&self,num: DistinguishableNum,) -> ::tauri_bindgen_host::anyhow::Result<DistinguishableNum>;
    fn identify_distinguishable_num(&self,num: DistinguishableNum,) -> ::tauri_bindgen_host::anyhow::Result<u8>;
  }
  
  pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
  where
  U: Unions+ Send + Sync + 'static,
  R: ::tauri_bindgen_host::tauri::Runtime + 'static
  {
    
    move |invoke| {
      let span = ::tauri_bindgen_host::tracing::span!(
      ::tauri_bindgen_host::tracing::Level::TRACE,
      "tauri-bindgen invoke handler",
      module = "unions", function = invoke.message.command(), payload = ?invoke.message.payload()
      );
      let _enter = span.enter();
      
      match invoke.message.command() {
        "add-one-integer" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          let num= match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "add-one-integer",
            key: "num",
            message: &__tauri_message__,
          }) {
            Ok(arg) => arg,
            Err(err) => {
              ::tauri_bindgen_host::tracing::error!(module = "unions", function = "add-one-integer", "Invoke handler returned error {:?}", err);
              return __tauri_resolver__.invoke_error(err);
            },
          };
          
          let result = ctx.add_one_integer(
          num, );
          
          __tauri_resolver__.respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
          
        },
        "add-one-float" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          let num= match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "add-one-float",
            key: "num",
            message: &__tauri_message__,
          }) {
            Ok(arg) => arg,
            Err(err) => {
              ::tauri_bindgen_host::tracing::error!(module = "unions", function = "add-one-float", "Invoke handler returned error {:?}", err);
              return __tauri_resolver__.invoke_error(err);
            },
          };
          
          let result = ctx.add_one_float(
          num, );
          
          __tauri_resolver__.respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
          
        },
        "replace-first-char" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          let text= match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "replace-first-char",
            key: "text",
            message: &__tauri_message__,
          }) {
            Ok(arg) => arg,
            Err(err) => {
              ::tauri_bindgen_host::tracing::error!(module = "unions", function = "replace-first-char", "Invoke handler returned error {:?}", err);
              return __tauri_resolver__.invoke_error(err);
            },
          };
          
          let letter= match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "replace-first-char",
            key: "letter",
            message: &__tauri_message__,
          }) {
            Ok(arg) => arg,
            Err(err) => {
              ::tauri_bindgen_host::tracing::error!(module = "unions", function = "replace-first-char", "Invoke handler returned error {:?}", err);
              return __tauri_resolver__.invoke_error(err);
            },
          };
          
          let result = ctx.replace_first_char(
          text, letter, );
          
          __tauri_resolver__.respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
          
        },
        "identify-integer" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          let num= match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "identify-integer",
            key: "num",
            message: &__tauri_message__,
          }) {
            Ok(arg) => arg,
            Err(err) => {
              ::tauri_bindgen_host::tracing::error!(module = "unions", function = "identify-integer", "Invoke handler returned error {:?}", err);
              return __tauri_resolver__.invoke_error(err);
            },
          };
          
          let result = ctx.identify_integer(
          num, );
          
          __tauri_resolver__.respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
          
        },
        "identify-float" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          let num= match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "identify-float",
            key: "num",
            message: &__tauri_message__,
          }) {
            Ok(arg) => arg,
            Err(err) => {
              ::tauri_bindgen_host::tracing::error!(module = "unions", function = "identify-float", "Invoke handler returned error {:?}", err);
              return __tauri_resolver__.invoke_error(err);
            },
          };
          
          let result = ctx.identify_float(
          num, );
          
          __tauri_resolver__.respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
          
        },
        "identify-text" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          let text= match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "identify-text",
            key: "text",
            message: &__tauri_message__,
          }) {
            Ok(arg) => arg,
            Err(err) => {
              ::tauri_bindgen_host::tracing::error!(module = "unions", function = "identify-text", "Invoke handler returned error {:?}", err);
              return __tauri_resolver__.invoke_error(err);
            },
          };
          
          let result = ctx.identify_text(
          text, );
          
          __tauri_resolver__.respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
          
        },
        "add-one-duplicated" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          let num= match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "add-one-duplicated",
            key: "num",
            message: &__tauri_message__,
          }) {
            Ok(arg) => arg,
            Err(err) => {
              ::tauri_bindgen_host::tracing::error!(module = "unions", function = "add-one-duplicated", "Invoke handler returned error {:?}", err);
              return __tauri_resolver__.invoke_error(err);
            },
          };
          
          let result = ctx.add_one_duplicated(
          num, );
          
          __tauri_resolver__.respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
          
        },
        "identify-duplicated" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          let num= match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "identify-duplicated",
            key: "num",
            message: &__tauri_message__,
          }) {
            Ok(arg) => arg,
            Err(err) => {
              ::tauri_bindgen_host::tracing::error!(module = "unions", function = "identify-duplicated", "Invoke handler returned error {:?}", err);
              return __tauri_resolver__.invoke_error(err);
            },
          };
          
          let result = ctx.identify_duplicated(
          num, );
          
          __tauri_resolver__.respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
          
        },
        "add-one-distinguishable-num" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          let num= match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "add-one-distinguishable-num",
            key: "num",
            message: &__tauri_message__,
          }) {
            Ok(arg) => arg,
            Err(err) => {
              ::tauri_bindgen_host::tracing::error!(module = "unions", function = "add-one-distinguishable-num", "Invoke handler returned error {:?}", err);
              return __tauri_resolver__.invoke_error(err);
            },
          };
          
          let result = ctx.add_one_distinguishable_num(
          num, );
          
          __tauri_resolver__.respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
          
        },
        "identify-distinguishable-num" => {
          #[allow(unused_variables)]
          let ::tauri_bindgen_host::tauri::Invoke {
            message: __tauri_message__,
            resolver: __tauri_resolver__,
          } = invoke;
          let num= match ::tauri_bindgen_host::tauri::command::CommandArg::from_command(::tauri_bindgen_host::tauri::command::CommandItem {
            name: "identify-distinguishable-num",
            key: "num",
            message: &__tauri_message__,
          }) {
            Ok(arg) => arg,
            Err(err) => {
              ::tauri_bindgen_host::tracing::error!(module = "unions", function = "identify-distinguishable-num", "Invoke handler returned error {:?}", err);
              return __tauri_resolver__.invoke_error(err);
            },
          };
          
          let result = ctx.identify_distinguishable_num(
          num, );
          
          __tauri_resolver__.respond(result.map_err(::tauri_bindgen_host::tauri::InvokeError::from_anyhow));
          
        },
        func_name => {
          ::tauri_bindgen_host::tracing::error!(module = "unions", function = func_name, "Not Found");
          invoke.resolver.reject("Not Found");
        }
      }
    }
  }
  
}

