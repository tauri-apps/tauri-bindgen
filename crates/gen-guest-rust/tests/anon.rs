#[allow(clippy::all, unused)]
pub mod anon {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Deserialize)]
    pub enum Error {
        Success,
        Failure,
    }
    pub async fn option_test() -> Result<Option<String>, Error> {
        ::tauri_bindgen_guest_rust::invoke("plugin:9f005dd416978e86|option-test", ())
            .await
            .unwrap()
    }
}
