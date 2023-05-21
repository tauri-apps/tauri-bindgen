#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod strings {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    pub async fn a(x: &'_ str) {
        ::tauri_bindgen_guest_rust::invoke("strings", "a", &(x)).await.unwrap()
    }
    pub async fn b() -> String {
        ::tauri_bindgen_guest_rust::invoke("strings", "b", &()).await.unwrap()
    }
    pub async fn c(a: &'_ str, b: &'_ str) -> String {
        ::tauri_bindgen_guest_rust::invoke("strings", "c", &(a, b)).await.unwrap()
    }
}
