#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod integers {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    pub async fn a1(x: u8) {
        ::tauri_bindgen_guest_rust::invoke("integers", "a1", &(x)).await.unwrap()
    }
    pub async fn a2(x: i8) {
        ::tauri_bindgen_guest_rust::invoke("integers", "a2", &(x)).await.unwrap()
    }
    pub async fn a3(x: u16) {
        ::tauri_bindgen_guest_rust::invoke("integers", "a3", &(x)).await.unwrap()
    }
    pub async fn a4(x: i16) {
        ::tauri_bindgen_guest_rust::invoke("integers", "a4", &(x)).await.unwrap()
    }
    pub async fn a5(x: u32) {
        ::tauri_bindgen_guest_rust::invoke("integers", "a5", &(x)).await.unwrap()
    }
    pub async fn a6(x: i32) {
        ::tauri_bindgen_guest_rust::invoke("integers", "a6", &(x)).await.unwrap()
    }
    pub async fn a7(x: u64) {
        ::tauri_bindgen_guest_rust::invoke("integers", "a7", &(x)).await.unwrap()
    }
    pub async fn a8(x: i64) {
        ::tauri_bindgen_guest_rust::invoke("integers", "a8", &(x)).await.unwrap()
    }
    pub async fn a9(x: u128) {
        ::tauri_bindgen_guest_rust::invoke("integers", "a9", &(x)).await.unwrap()
    }
    pub async fn a10(x: i128) {
        ::tauri_bindgen_guest_rust::invoke("integers", "a10", &(x)).await.unwrap()
    }
    pub async fn a11(
        p1: u8,
        p2: i8,
        p3: u16,
        p4: i16,
        p5: u32,
        p6: i32,
        p7: u64,
        p8: i64,
        p9: u128,
        p10: i128,
    ) {
        ::tauri_bindgen_guest_rust::invoke(
                "integers",
                "a11",
                &(p1, p2, p3, p4, p5, p6, p7, p8, p9, p10),
            )
            .await
            .unwrap()
    }
    pub async fn r1() -> u8 {
        ::tauri_bindgen_guest_rust::invoke("integers", "r1", &()).await.unwrap()
    }
    pub async fn r2() -> i8 {
        ::tauri_bindgen_guest_rust::invoke("integers", "r2", &()).await.unwrap()
    }
    pub async fn r3() -> u16 {
        ::tauri_bindgen_guest_rust::invoke("integers", "r3", &()).await.unwrap()
    }
    pub async fn r4() -> i16 {
        ::tauri_bindgen_guest_rust::invoke("integers", "r4", &()).await.unwrap()
    }
    pub async fn r5() -> u32 {
        ::tauri_bindgen_guest_rust::invoke("integers", "r5", &()).await.unwrap()
    }
    pub async fn r6() -> i32 {
        ::tauri_bindgen_guest_rust::invoke("integers", "r6", &()).await.unwrap()
    }
    pub async fn r7() -> u64 {
        ::tauri_bindgen_guest_rust::invoke("integers", "r7", &()).await.unwrap()
    }
    pub async fn r8() -> i64 {
        ::tauri_bindgen_guest_rust::invoke("integers", "r8", &()).await.unwrap()
    }
    pub async fn r9() -> u128 {
        ::tauri_bindgen_guest_rust::invoke("integers", "r9", &()).await.unwrap()
    }
    pub async fn r10() -> i128 {
        ::tauri_bindgen_guest_rust::invoke("integers", "r10", &()).await.unwrap()
    }
    pub async fn pair_ret() -> (i64, u8) {
        ::tauri_bindgen_guest_rust::invoke("integers", "pair_ret", &()).await.unwrap()
    }
}
