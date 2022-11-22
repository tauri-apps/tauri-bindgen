#[allow(clippy::all, unused)]
pub mod imports {
    pub async fn f1() -> () {
        ::tauri_bindgen_guest_rust::send("plugin:imports|f1", ()).await
    }
    pub async fn f2(a: u32) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            a: u32,
        }
        let params = Params { a };
        ::tauri_bindgen_guest_rust::send("plugin:imports|f2", &params).await
    }
    pub async fn f3(a: u32, b: u32) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            a: u32,
            b: u32,
        }
        let params = Params { a, b };
        ::tauri_bindgen_guest_rust::send("plugin:imports|f3", &params).await
    }
    pub async fn f4() -> u32 {
        ::tauri_bindgen_guest_rust::send("plugin:imports|f4", ()).await
    }
    pub async fn f5() -> (u32, u32) {
        ::tauri_bindgen_guest_rust::send("plugin:imports|f5", ()).await
    }
    pub async fn f6(a: u32, b: u32, c: u32) -> (u32, u32, u32) {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            a: u32,
            b: u32,
            c: u32,
        }
        let params = Params { a, b, c };
        ::tauri_bindgen_guest_rust::send("plugin:imports|f6", &params).await
    }
}
