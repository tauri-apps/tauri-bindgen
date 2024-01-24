#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod simple_functions {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    pub async fn f1() {
        ::tauri_bindgen_guest_rust::invoke("simple_functions", "f1", &()).await.unwrap()
    }
    pub async fn f2(a: u32) {
        ::tauri_bindgen_guest_rust::invoke("simple_functions", "f2", &(a)).await.unwrap()
    }
    pub async fn f3(a: u32, b: u32) {
        ::tauri_bindgen_guest_rust::invoke("simple_functions", "f3", &(a, b))
            .await
            .unwrap()
    }
    pub async fn f4() -> u32 {
        ::tauri_bindgen_guest_rust::invoke("simple_functions", "f4", &()).await.unwrap()
    }
    pub async fn f5() -> (u32, u32) {
        ::tauri_bindgen_guest_rust::invoke("simple_functions", "f5", &()).await.unwrap()
    }
    pub async fn f6(a: u32, b: u32, c: u32) -> (u32, u32, u32) {
        ::tauri_bindgen_guest_rust::invoke("simple_functions", "f6", &(a, b, c))
            .await
            .unwrap()
    }
}
