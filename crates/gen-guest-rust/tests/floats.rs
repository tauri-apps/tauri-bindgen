#[allow(clippy::all, unused)]
#[rustfmt::skip]
pub mod floats{
  use ::tauri_bindgen_guest_rust::tauri_bindgen_abi;
  pub async fn float32_param(x: f32,) -> () {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params {
      x : f32,
    }
    let params = Params {x,};
    ::tauri_bindgen_guest_rust::invoke("plugin:979575fda4ffb8b9|float32-param", &params).await.unwrap()
  }
  pub async fn float64_param(x: f64,) -> () {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params {
      x : f64,
    }
    let params = Params {x,};
    ::tauri_bindgen_guest_rust::invoke("plugin:979575fda4ffb8b9|float64-param", &params).await.unwrap()
  }
  pub async fn float32_result() -> f32 {
    ::tauri_bindgen_guest_rust::invoke("plugin:979575fda4ffb8b9|float32-result", ()).await.unwrap()
  }
  pub async fn float64_result() -> f64 {
    ::tauri_bindgen_guest_rust::invoke("plugin:979575fda4ffb8b9|float64-result", ()).await.unwrap()
  }
  
}
