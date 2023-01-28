#[allow(clippy::all, unused)]
#[rustfmt::skip]
pub mod small_anonymous{
  use ::tauri_bindgen_guest_rust::tauri_bindgen_abi;
  #[repr(u8)]
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(tauri_bindgen_abi::Readable)]
  pub enum Error{
    Success,
    Failure,
  }
  pub async fn option_test() -> Result<Option<String>,Error> {
    ::tauri_bindgen_guest_rust::invoke("plugin:f831ebf42dd49cbb|option-test", ()).await.unwrap()
  }
  
}
