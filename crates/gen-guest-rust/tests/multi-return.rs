#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod multi_return {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    pub async fn mra() {
        ::tauri_bindgen_guest_rust::invoke("multi_return", "mra", &()).await.unwrap()
    }
    pub async fn mrb() {
        ::tauri_bindgen_guest_rust::invoke("multi_return", "mrb", &()).await.unwrap()
    }
    pub async fn mrc() -> u32 {
        ::tauri_bindgen_guest_rust::invoke("multi_return", "mrc", &()).await.unwrap()
    }
    pub async fn mrd() -> u32 {
        ::tauri_bindgen_guest_rust::invoke("multi_return", "mrd", &()).await.unwrap()
    }
    pub async fn mre() -> (u32, f32) {
        ::tauri_bindgen_guest_rust::invoke("multi_return", "mre", &()).await.unwrap()
    }
}
