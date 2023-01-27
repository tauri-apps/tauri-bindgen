#![allow(clippy::all, unused)]
#[rustfmt::skip]
pub mod simple_lists{
  pub async fn simple_list1(l: &[u32],) -> () {
    #[derive(::serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Params<'a,> {
      l : &'a [u32],
    }
    let params = Params {l,};
    ::tauri_bindgen_guest_rust::invoke("plugin:e8600e8d0423cbdb|simple-list1", &params).await.unwrap()
  }
  pub async fn simple_list2() -> Vec<u32> {
    ::tauri_bindgen_guest_rust::invoke("plugin:e8600e8d0423cbdb|simple-list2", ()).await.unwrap()
  }
  pub async fn simple_list4(l: &[&[u32]],) -> Vec<Vec<u32>> {
    #[derive(::serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Params<'a,> {
      l : &'a [&'a [u32]],
    }
    let params = Params {l,};
    ::tauri_bindgen_guest_rust::invoke("plugin:e8600e8d0423cbdb|simple-list4", &params).await.unwrap()
  }
  
}
