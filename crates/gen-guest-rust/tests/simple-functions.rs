#[allow(clippy::all, unused)]
pub mod imports {
    pub async fn f1() -> () {
        ::tauri_bindgen_guest_rust::invoke("plugin:ebb2d6f0|f1", ())
            .await
            .unwrap()
    }
    pub async fn f2(a: u32) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            a: u32,
        }
        let params = Params { a };
        ::tauri_bindgen_guest_rust::invoke("plugin:ebb2d6f0|f2", &params)
            .await
            .unwrap()
    }
    pub async fn f3(a: u32, b: u32) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            a: u32,
            b: u32,
        }
        let params = Params { a, b };
        ::tauri_bindgen_guest_rust::invoke("plugin:ebb2d6f0|f3", &params)
            .await
            .unwrap()
    }
    pub async fn f4() -> u32 {
        ::tauri_bindgen_guest_rust::invoke("plugin:ebb2d6f0|f4", ())
            .await
            .unwrap()
    }
    pub async fn f5() -> (u32, u32) {
        ::tauri_bindgen_guest_rust::invoke("plugin:ebb2d6f0|f5", ())
            .await
            .unwrap()
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
        ::tauri_bindgen_guest_rust::invoke("plugin:ebb2d6f0|f6", &params)
            .await
            .unwrap()
    }
}
