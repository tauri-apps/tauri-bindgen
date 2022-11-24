#[allow(clippy::all, unused)]
pub mod imports {
    const IDL_HASH: &str = "d238f57052cdcb90";
    pub async fn mra() -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:multi_return|mra", ()).await
    }
    pub async fn mrb() -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:multi_return|mrb", ()).await
    }
    pub async fn mrc() -> u32 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:multi_return|mrc", ()).await
    }
    pub async fn mrd() -> u32 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:multi_return|mrd", ()).await
    }
    pub async fn mre() -> (u32, f32) {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:multi_return|mre", ()).await
    }
}
