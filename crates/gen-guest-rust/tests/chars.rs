#![allow(clippy::all, unused)]
#[rustfmt::skip]
pub mod chars{
  /// A function that accepts a character
  pub async fn take_char(x: char,) -> () {
    #[derive(::serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Params {
      x : char,
    }
    let params = Params {x,};
    ::tauri_bindgen_guest_rust::invoke("plugin:58d944fc9a2c8431|take-char", &params).await.unwrap()
  }
  /// A function that returns a character
  pub async fn return_char() -> char {
    ::tauri_bindgen_guest_rust::invoke("plugin:58d944fc9a2c8431|return-char", ()).await.unwrap()
  }
  
}
