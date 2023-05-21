#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod conventions {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    #[derive(serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct LudicrousSpeed {
        pub how_fast_are_you_going: u32,
        pub i_am_going_extremely_slow: u64,
    }
    pub async fn kebab_case() {
        ::tauri_bindgen_guest_rust::invoke("conventions", "kebab_case", &())
            .await
            .unwrap()
    }
    pub async fn foo(x: LudicrousSpeed) {
        ::tauri_bindgen_guest_rust::invoke("conventions", "foo", &(x)).await.unwrap()
    }
    pub async fn function_with_underscores() {
        ::tauri_bindgen_guest_rust::invoke(
                "conventions",
                "function_with_underscores",
                &(),
            )
            .await
            .unwrap()
    }
    pub async fn function_with_no_weird_characters() {
        ::tauri_bindgen_guest_rust::invoke(
                "conventions",
                "function_with_no_weird_characters",
                &(),
            )
            .await
            .unwrap()
    }
    pub async fn apple() {
        ::tauri_bindgen_guest_rust::invoke("conventions", "apple", &()).await.unwrap()
    }
    pub async fn apple_pear() {
        ::tauri_bindgen_guest_rust::invoke("conventions", "apple_pear", &())
            .await
            .unwrap()
    }
    pub async fn apple_pear_grape() {
        ::tauri_bindgen_guest_rust::invoke("conventions", "apple_pear_grape", &())
            .await
            .unwrap()
    }
    pub async fn a0() {
        ::tauri_bindgen_guest_rust::invoke("conventions", "a0", &()).await.unwrap()
    }
    pub async fn is_xml() {
        ::tauri_bindgen_guest_rust::invoke("conventions", "is_xml", &()).await.unwrap()
    }
    pub async fn explicit() {
        ::tauri_bindgen_guest_rust::invoke("conventions", "explicit", &()).await.unwrap()
    }
    pub async fn explicit_snake() {
        ::tauri_bindgen_guest_rust::invoke("conventions", "explicit_snake", &())
            .await
            .unwrap()
    }
    pub async fn bool() {
        ::tauri_bindgen_guest_rust::invoke("conventions", "bool", &()).await.unwrap()
    }
}
