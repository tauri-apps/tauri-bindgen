#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod small_anonymous {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    #[derive(serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Error {
        Success,
        Failure,
    }
    pub async fn option_test() -> Result<Option<String>, Error> {
        ::tauri_bindgen_guest_rust::invoke("small_anonymous", "option_test", &())
            .await
            .unwrap()
    }
}
