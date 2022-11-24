#[allow(clippy::all, unused)]
pub mod imports {
    const IDL_HASH: &str = "16c3ebd2deefea81";
    pub async fn a(x: &str) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a str,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:strings|a", &params).await
    }
    pub async fn b() -> String {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:strings|b", ()).await
    }
    pub async fn c(a: &str, b: &str) -> String {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            a: &'a str,
            b: &'a str,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            a,
            b,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:strings|c", &params).await
    }
}
