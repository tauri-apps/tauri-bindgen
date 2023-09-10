#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod streams {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    ///A function that returns a stream of strings
    pub async fn string_stream() -> ::tauri_bindgen_guest_rust::Streaming<String> {
        ::tauri_bindgen_guest_rust::start_stream("streams", "string_stream", &())
            .await
            .unwrap()
    }
}
