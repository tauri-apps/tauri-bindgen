#[allow(clippy::all, unused)]
pub mod imports {
    /// A function that accepts a character
    pub async fn take_char(x: char) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            #[serde(rename = "0")]
            x: char,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:chars|0", &params).await
    }
    /// A function that returns a character
    pub async fn return_char() -> char {
        ::tauri_bindgen_guest_rust::invoke("plugin:chars|1", ()).await
    }
}
