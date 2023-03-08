pub mod small_anonymous {
    use ::tauri_bindgen_guest_rust::tauri_bindgen_abi;
    use ::tauri_bindgen_guest_rust::bitflags;
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum Error {
        Success,
        Failure,
    }
    pub async fn option_test() -> Result<Option<String>, Error> {
        todo!()
    }
}
