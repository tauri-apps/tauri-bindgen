#[allow(clippy::all, unused)]
pub mod imports {

    #[cfg(debug_assertions)]
    static START: ::std::sync::Once = ::std::sync::Once::new();
    #[cfg(debug_assertions)]
    fn check_idl_version() {
        ::tauri_bindgen_guest_rust::wasm_bindgen_futures::spawn_local(async {
            if ::tauri_bindgen_guest_rust::invoke::<_, ()>(
                "plugin:anon|bee731db80799df9a7eea6b7e0b7a0ce",
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

    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Deserialize)]
    pub enum Error {
        Success,
        Failure,
    }
    pub async fn option_test() -> Result<Option<String>, Error> {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:anon|option-test", ())
            .await
            .unwrap()
    }
}
