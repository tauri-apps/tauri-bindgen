#[allow(clippy::all, unused)]
pub mod imports {
    pub async fn a1(x: u8) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u8,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::send("plugin:imports|a1", &params).await
    }
    pub async fn a2(x: i8) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i8,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::send("plugin:imports|a2", &params).await
    }
    pub async fn a3(x: u16) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u16,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::send("plugin:imports|a3", &params).await
    }
    pub async fn a4(x: i16) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i16,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::send("plugin:imports|a4", &params).await
    }
    pub async fn a5(x: u32) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u32,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::send("plugin:imports|a5", &params).await
    }
    pub async fn a6(x: i32) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i32,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::send("plugin:imports|a6", &params).await
    }
    pub async fn a7(x: u64) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u64,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::send("plugin:imports|a7", &params).await
    }
    pub async fn a8(x: i64) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i64,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::send("plugin:imports|a8", &params).await
    }
    pub async fn a9(p1: u8, p2: i8, p3: u16, p4: i16, p5: u32, p6: i32, p7: u64, p8: i64) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            p1: u8,
            p2: i8,
            p3: u16,
            p4: i16,
            p5: u32,
            p6: i32,
            p7: u64,
            p8: i64,
        }
        let params = Params {
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
            p8,
        };
        ::tauri_bindgen_guest_rust::send("plugin:imports|a9", &params).await
    }
    pub async fn r1() -> u8 {
        ::tauri_bindgen_guest_rust::send("plugin:imports|r1", ()).await
    }
    pub async fn r2() -> i8 {
        ::tauri_bindgen_guest_rust::send("plugin:imports|r2", ()).await
    }
    pub async fn r3() -> u16 {
        ::tauri_bindgen_guest_rust::send("plugin:imports|r3", ()).await
    }
    pub async fn r4() -> i16 {
        ::tauri_bindgen_guest_rust::send("plugin:imports|r4", ()).await
    }
    pub async fn r5() -> u32 {
        ::tauri_bindgen_guest_rust::send("plugin:imports|r5", ()).await
    }
    pub async fn r6() -> i32 {
        ::tauri_bindgen_guest_rust::send("plugin:imports|r6", ()).await
    }
    pub async fn r7() -> u64 {
        ::tauri_bindgen_guest_rust::send("plugin:imports|r7", ()).await
    }
    pub async fn r8() -> i64 {
        ::tauri_bindgen_guest_rust::send("plugin:imports|r8", ()).await
    }
    pub async fn pair_ret() -> (i64, u8) {
        ::tauri_bindgen_guest_rust::send("plugin:imports|pair_ret", ()).await
    }
}
