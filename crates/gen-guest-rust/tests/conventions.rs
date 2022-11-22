#[allow(clippy::all, unused)]
pub mod imports {
    #[repr(C)]
    #[derive(Debug, Copy, Clone, ::serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LudicrousSpeed {
        pub how_fast_are_you_going: u32,
        pub i_am_going_extremely_slow: u64,
    }
    pub async fn kebab_case() -> () {
        ::tauri_bindgen_guest_rust::send("plugin:imports|kebab_case", ()).await
    }
    pub async fn foo(x: LudicrousSpeed) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: LudicrousSpeed,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::send("plugin:imports|foo", &params).await
    }
    pub async fn function_with_dashes() -> () {
        ::tauri_bindgen_guest_rust::send("plugin:imports|function_with_dashes", ()).await
    }
    pub async fn function_with_no_weird_characters() -> () {
        ::tauri_bindgen_guest_rust::send("plugin:imports|function_with_no_weird_characters", ())
            .await
    }
    pub async fn apple() -> () {
        ::tauri_bindgen_guest_rust::send("plugin:imports|apple", ()).await
    }
    pub async fn apple_pear() -> () {
        ::tauri_bindgen_guest_rust::send("plugin:imports|apple_pear", ()).await
    }
    pub async fn apple_pear_grape() -> () {
        ::tauri_bindgen_guest_rust::send("plugin:imports|apple_pear_grape", ()).await
    }
    pub async fn a0() -> () {
        ::tauri_bindgen_guest_rust::send("plugin:imports|a0", ()).await
    }
    pub async fn is_xml() -> () {
        ::tauri_bindgen_guest_rust::send("plugin:imports|is_xml", ()).await
    }
    pub async fn explicit() -> () {
        ::tauri_bindgen_guest_rust::send("plugin:imports|explicit", ()).await
    }
    pub async fn explicit_kebab() -> () {
        ::tauri_bindgen_guest_rust::send("plugin:imports|explicit_kebab", ()).await
    }
    pub async fn bool() -> () {
        ::tauri_bindgen_guest_rust::send("plugin:imports|bool", ()).await
    }
}
