#[allow(clippy::all, unused)]
pub mod imports {
    pub async fn float32_param(x: f32) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: f32,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:b2ded0ef970e6596|float32-param", &params)
            .await
            .unwrap()
    }
    pub async fn float64_param(x: f64) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: f64,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:b2ded0ef970e6596|float64-param", &params)
            .await
            .unwrap()
    }
    pub async fn float32_result() -> f32 {
        ::tauri_bindgen_guest_rust::invoke("plugin:b2ded0ef970e6596|float32-result", ())
            .await
            .unwrap()
    }
    pub async fn float64_result() -> f64 {
        ::tauri_bindgen_guest_rust::invoke("plugin:b2ded0ef970e6596|float64-result", ())
            .await
            .unwrap()
    }
}
