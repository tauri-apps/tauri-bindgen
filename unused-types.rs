
#[allow(clippy::all)]
pub mod unused_types{
  pub const WORLD_HASH: &str = "cc0d95a0fa90d663";
  pub trait UnusedTypes: Sized {
  }
  
  pub fn invoke_handler<U, R>(ctx: U) -> impl Fn(::tauri_bindgen_host::tauri::Invoke<R>)
  where
  U: UnusedTypes+ Send + Sync + 'static,
  R: ::tauri_bindgen_host::tauri::Runtime + 'static
  {
    
    move |invoke| {
      match invoke.message.command() {
        func_name => {
          invoke.resolver.reject("Not Found")
        }
      }
    }
  }
  
}

