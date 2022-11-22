#[allow(clippy::all, unused)]
pub mod imports {
    /// A function that accepts a character
    pub async fn take_char(x: char) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: char,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::send("plugin:imports|take_char", &params).await
    }
    /// A function that returns a character
    pub async fn return_char() -> char {
        ::tauri_bindgen_guest_rust::send("plugin:imports|return_char", ()).await
    }
}
