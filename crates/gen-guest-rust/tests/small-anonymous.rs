#[allow(clippy::all, unused)]
pub mod imports {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, ::serde::Deserialize)]
    pub enum Error {
        Success,
        Failure,
    }
    pub async fn option_test() -> Result<Option<String>, Error> {
        ::tauri_bindgen_guest_rust::send("plugin:imports|option_test", ()).await
    }
}
