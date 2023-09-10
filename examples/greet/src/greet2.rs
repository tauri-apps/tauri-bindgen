#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod greet {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    pub async fn greet(my_name: &'_ str) -> ::tauri_bindgen_guest_rust::Streaming<Vec<u8>> {
        ::tauri_bindgen_guest_rust::open_stream("greet", "greet", &(my_name)).await.unwrap()
    }
}
