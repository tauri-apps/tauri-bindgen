
#![allow(clippy::all, unused)]
pub mod empty{
  pub const WORLD_HASH: &str = "882ecd0a3d152e5d";
  pub trait Empty: Sized {
  }
  
  pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
  where
  U: Empty+ Send + Sync + 'static,
  R: ::tauri_bindgen_host::tauri::Runtime + 'static
  {
    
    move |invoke| {
      let span = ::tauri_bindgen_host::tracing::span!(
      ::tauri_bindgen_host::tracing::Level::TRACE,
      "tauri-bindgen invoke handler",
      module = "empty", function = invoke.message.command(), payload = ?invoke.message.payload()
      );
      let _enter = span.enter();
      
      match invoke.message.command() {
        func_name => {
          ::tauri_bindgen_host::tracing::error!(module = "empty", function = func_name, "Not Found");
          invoke.resolver.reject("Not Found");
        }
      }
    }
  }
  
}

