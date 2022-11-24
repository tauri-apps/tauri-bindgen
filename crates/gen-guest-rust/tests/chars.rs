#[allow(clippy::all, unused)]
pub mod imports {

    #[cfg(debug_assertions)]
    static START: ::std::sync::Once = ::std::sync::Once::new();
    #[cfg(debug_assertions)]
    fn check_idl_version() {
        ::tauri_bindgen_guest_rust::wasm_bindgen_futures::spawn_local(async {
            if ::tauri_bindgen_guest_rust::invoke::<_, ()>(
                "plugin:chars|678374cfb5cdb2b5ba845e4b559f402a",
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

    /// A function that accepts a character
    pub async fn take_char(x: char) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: char,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:chars|take-char", &params)
            .await
            .unwrap()
    }
    /// A function that returns a character
    pub async fn return_char() -> char {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:chars|return-char", ())
            .await
            .unwrap()
    }
}
