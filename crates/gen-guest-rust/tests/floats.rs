#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod floats {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    pub async fn float32_param(x: f32) {
        ::tauri_bindgen_guest_rust::invoke("floats", "float32_param", &(x))
            .await
            .unwrap()
    }
    pub async fn float64_param(x: f64) {
        ::tauri_bindgen_guest_rust::invoke("floats", "float64_param", &(x))
            .await
            .unwrap()
    }
    pub async fn float32_result() -> f32 {
        ::tauri_bindgen_guest_rust::invoke("floats", "float32_result", &())
            .await
            .unwrap()
    }
    pub async fn float64_result() -> f64 {
        ::tauri_bindgen_guest_rust::invoke("floats", "float64_result", &())
            .await
            .unwrap()
    }
}
