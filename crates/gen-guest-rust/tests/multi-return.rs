#[allow(clippy::all, unused)]
pub mod imports {
    pub async fn mra() -> () {
        ::tauri_bindgen_guest_rust::send("plugin:imports|mra", ()).await
    }
    pub async fn mrb() -> () {
        ::tauri_bindgen_guest_rust::send("plugin:imports|mrb", ()).await
    }
    pub async fn mrc() -> u32 {
        ::tauri_bindgen_guest_rust::send("plugin:imports|mrc", ()).await
    }
    pub async fn mrd() -> u32 {
        ::tauri_bindgen_guest_rust::send("plugin:imports|mrd", ()).await
    }
    pub async fn mre() -> (u32, f32) {
        ::tauri_bindgen_guest_rust::send("plugin:imports|mre", ()).await
    }
}
