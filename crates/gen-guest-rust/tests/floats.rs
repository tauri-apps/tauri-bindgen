#[allow(clippy::all, unused)]
pub mod imports {
    pub async fn float32_param(x: f32) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: f32,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::send("plugin:imports|float32_param", &params).await
    }
    pub async fn float64_param(x: f64) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: f64,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::send("plugin:imports|float64_param", &params).await
    }
    pub async fn float32_result() -> f32 {
        ::tauri_bindgen_guest_rust::send("plugin:imports|float32_result", ()).await
    }
    pub async fn float64_result() -> f64 {
        ::tauri_bindgen_guest_rust::send("plugin:imports|float64_result", ()).await
    }
}
