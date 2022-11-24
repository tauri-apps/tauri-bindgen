#[allow(clippy::all, unused)]
pub mod imports {
    const IDL_HASH: &str = "678374cfb5cdb2b5";
    /// A function that accepts a character
    pub async fn take_char(x: char) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: char,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:chars|take-char", &params).await
    }
    /// A function that returns a character
    pub async fn return_char() -> char {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:chars|return-char", ()).await
    }
}
