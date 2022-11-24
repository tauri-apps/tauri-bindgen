#[allow(clippy::all, unused)]
pub mod imports {

    #[cfg(debug_assertions)]
    static START: ::std::sync::Once = ::std::sync::Once::new();
    #[cfg(debug_assertions)]
    fn check_idl_version() {
        ::tauri_bindgen_guest_rust::wasm_bindgen_futures::spawn_local(async {
            if ::tauri_bindgen_guest_rust::invoke::<_, ()>(
                "plugin:multi_return|d238f57052cdcb9073d14f7a8059345b",
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

    pub async fn mra() -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:multi_return|mra", ())
            .await
            .unwrap()
    }
    pub async fn mrb() -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:multi_return|mrb", ())
            .await
            .unwrap()
    }
    pub async fn mrc() -> u32 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:multi_return|mrc", ())
            .await
            .unwrap()
    }
    pub async fn mrd() -> u32 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:multi_return|mrd", ())
            .await
            .unwrap()
    }
    pub async fn mre() -> (u32, f32) {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:multi_return|mre", ())
            .await
            .unwrap()
    }
}
