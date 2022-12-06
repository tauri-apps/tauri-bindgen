#[allow(clippy::all, unused)]
pub mod small_anonymous {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Deserialize)]
    pub enum Error {
        Success,
        Failure,
    }
    pub async fn option_test() -> Result<Option<String>, Error> {
        ::tauri_bindgen_guest_rust::invoke("plugin:f831ebf42dd49cbb|option-test", ())
            .await
            .unwrap()
    }
}
