#[allow(unused_imports, unused_variables)]
pub mod small_anonymous {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    #[derive(serde::Deserialize)]
    pub enum Error {
        Success,
        Failure,
    }
    pub async fn option_test() -> Result<Option<String>, Error> {
        todo!()
    }
}
