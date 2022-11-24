#[allow(clippy::all, unused)]
pub mod imports {
    const IDL_HASH: &str = "ebb2d6f0441e00a0";
    pub async fn f1() -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:simple|f1", ()).await
    }
    pub async fn f2(a: u32) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            a: u32,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            a,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:simple|f2", &params).await
    }
    pub async fn f3(a: u32, b: u32) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            a: u32,
            b: u32,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            a,
            b,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:simple|f3", &params).await
    }
    pub async fn f4() -> u32 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:simple|f4", ()).await
    }
    pub async fn f5() -> (u32, u32) {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:simple|f5", ()).await
    }
    pub async fn f6(a: u32, b: u32, c: u32) -> (u32, u32, u32) {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            a: u32,
            b: u32,
            c: u32,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            a,
            b,
            c,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:simple|f6", &params).await
    }
}
