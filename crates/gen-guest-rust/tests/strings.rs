#[allow(clippy::all, unused)]
#[rustfmt::skip]
pub mod strings{
  use ::tauri_bindgen_guest_rust::tauri_bindgen_abi;
  pub async fn a(x: &str,) -> () {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params<'a,> {
      x : &'a str,
    }
    let params = Params {x,};
    ::tauri_bindgen_guest_rust::invoke("plugin:4883b53925a5f618|a", &params).await.unwrap()
  }
  pub async fn b() -> String {
    ::tauri_bindgen_guest_rust::invoke("plugin:4883b53925a5f618|b", ()).await.unwrap()
  }
  pub async fn c(a: &str,b: &str,) -> String {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params<'a,> {
      a : &'a str,
      b : &'a str,
    }
    let params = Params {a,b,};
    ::tauri_bindgen_guest_rust::invoke("plugin:4883b53925a5f618|c", &params).await.unwrap()
  }
  
}
