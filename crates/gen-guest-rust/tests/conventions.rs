#[allow(clippy::all, unused)]
pub mod imports {
    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq, ::serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LudicrousSpeed {
        pub how_fast_are_you_going: u32,
        pub i_am_going_extremely_slow: u64,
    }
    pub async fn kebab_case() -> () {
        ::tauri_bindgen_guest_rust::invoke("plugin:48646a1b|kebab-case", ())
            .await
            .unwrap()
    }
    pub async fn foo(x: LudicrousSpeed) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: LudicrousSpeed,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:48646a1b|foo", &params)
            .await
            .unwrap()
    }
    pub async fn function_with_dashes() -> () {
        ::tauri_bindgen_guest_rust::invoke("plugin:48646a1b|function-with-dashes", ())
            .await
            .unwrap()
    }
    pub async fn function_with_no_weird_characters() -> () {
        ::tauri_bindgen_guest_rust::invoke("plugin:48646a1b|function-with-no-weird-characters", ())
            .await
            .unwrap()
    }
    pub async fn apple() -> () {
        ::tauri_bindgen_guest_rust::invoke("plugin:48646a1b|apple", ())
            .await
            .unwrap()
    }
    pub async fn apple_pear() -> () {
        ::tauri_bindgen_guest_rust::invoke("plugin:48646a1b|apple-pear", ())
            .await
            .unwrap()
    }
    pub async fn apple_pear_grape() -> () {
        ::tauri_bindgen_guest_rust::invoke("plugin:48646a1b|apple-pear-grape", ())
            .await
            .unwrap()
    }
    pub async fn a0() -> () {
        ::tauri_bindgen_guest_rust::invoke("plugin:48646a1b|a0", ())
            .await
            .unwrap()
    }
    pub async fn is_xml() -> () {
        ::tauri_bindgen_guest_rust::invoke("plugin:48646a1b|is-XML", ())
            .await
            .unwrap()
    }
    pub async fn explicit() -> () {
        ::tauri_bindgen_guest_rust::invoke("plugin:48646a1b|explicit", ())
            .await
            .unwrap()
    }
    pub async fn explicit_kebab() -> () {
        ::tauri_bindgen_guest_rust::invoke("plugin:48646a1b|explicit-kebab", ())
            .await
            .unwrap()
    }
    pub async fn bool() -> () {
        ::tauri_bindgen_guest_rust::invoke("plugin:48646a1b|bool", ())
            .await
            .unwrap()
    }
}
