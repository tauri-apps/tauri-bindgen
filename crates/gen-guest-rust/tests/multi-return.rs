
#![allow(clippy::all, unused)]
pub mod multi_return{
  pub async fn mra() -> () {
    ::tauri_bindgen_guest_rust::invoke("plugin:def17a258c1e4f4d|mra", ()).await.unwrap()
  }
  pub async fn mrb() -> () {
    ::tauri_bindgen_guest_rust::invoke("plugin:def17a258c1e4f4d|mrb", ()).await.unwrap()
  }
  pub async fn mrc() -> u32 {
    ::tauri_bindgen_guest_rust::invoke("plugin:def17a258c1e4f4d|mrc", ()).await.unwrap()
  }
  pub async fn mrd() -> u32 {
    ::tauri_bindgen_guest_rust::invoke("plugin:def17a258c1e4f4d|mrd", ()).await.unwrap()
  }
  pub async fn mre() -> (u32, f32, ) {
    ::tauri_bindgen_guest_rust::invoke("plugin:def17a258c1e4f4d|mre", ()).await.unwrap()
  }
  
}

