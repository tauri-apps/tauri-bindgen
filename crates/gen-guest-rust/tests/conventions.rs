#[allow(clippy::all, unused)]
pub mod imports {

    #[cfg(debug_assertions)]
    static START: ::std::sync::Once = ::std::sync::Once::new();
    #[cfg(debug_assertions)]
    fn check_idl_version() {
        ::tauri_bindgen_guest_rust::wasm_bindgen_futures::spawn_local(async {
            if ::tauri_bindgen_guest_rust::invoke::<_, ()>(
                "plugin:conventions|48646a1b1c089063e7b03a4c1dd9f5ad",
                (),
            )
            .await
            .is_err()
            {
                ::tauri_bindgen_guest_rust::console_warn("The Host bindings were generated from a different version of the definitions file. This usually means your Guest bindings are out-of-date. For more details see https://github.com/tauri-apps/tauri-bindgen#version-check.\nNote: This is a debug assertion and IDL versions will not be checked in release builds.
        ");
            }
        });
    }

    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq, ::serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LudicrousSpeed {
        pub how_fast_are_you_going: u32,
        pub i_am_going_extremely_slow: u64,
    }
    pub async fn kebab_case() -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:conventions|kebab-case", ())
            .await
            .unwrap()
    }
    pub async fn foo(x: LudicrousSpeed) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: LudicrousSpeed,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:conventions|foo", &params)
            .await
            .unwrap()
    }
    pub async fn function_with_dashes() -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:conventions|function-with-dashes", ())
            .await
            .unwrap()
    }
    pub async fn function_with_no_weird_characters() -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke(
            "plugin:conventions|function-with-no-weird-characters",
            (),
        )
        .await
        .unwrap()
    }
    pub async fn apple() -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:conventions|apple", ())
            .await
            .unwrap()
    }
    pub async fn apple_pear() -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:conventions|apple-pear", ())
            .await
            .unwrap()
    }
    pub async fn apple_pear_grape() -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:conventions|apple-pear-grape", ())
            .await
            .unwrap()
    }
    pub async fn a0() -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:conventions|a0", ())
            .await
            .unwrap()
    }
    pub async fn is_xml() -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:conventions|is-XML", ())
            .await
            .unwrap()
    }
    pub async fn explicit() -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:conventions|explicit", ())
            .await
            .unwrap()
    }
    pub async fn explicit_kebab() -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:conventions|explicit-kebab", ())
            .await
            .unwrap()
    }
    pub async fn bool() -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:conventions|bool", ())
            .await
            .unwrap()
    }
}
