#[allow(clippy::all, unused)]
pub mod imports {
    pub async fn mra() -> () {
        ::tauri_bindgen_guest_rust::invoke("plugin:d238f570|mra", ())
            .await
            .unwrap()
    }
    pub async fn mrb() -> () {
        ::tauri_bindgen_guest_rust::invoke("plugin:d238f570|mrb", ())
            .await
            .unwrap()
    }
    pub async fn mrc() -> u32 {
        ::tauri_bindgen_guest_rust::invoke("plugin:d238f570|mrc", ())
            .await
            .unwrap()
    }
    pub async fn mrd() -> u32 {
        ::tauri_bindgen_guest_rust::invoke("plugin:d238f570|mrd", ())
            .await
            .unwrap()
    }
    pub async fn mre() -> (u32, f32) {
        ::tauri_bindgen_guest_rust::invoke("plugin:d238f570|mre", ())
            .await
            .unwrap()
    }
}
