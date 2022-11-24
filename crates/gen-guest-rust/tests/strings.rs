#[allow(clippy::all, unused)]
pub mod imports {

    #[cfg(debug_assertions)]
    static START: ::std::sync::Once = ::std::sync::Once::new();
    #[cfg(debug_assertions)]
    fn check_idl_version() {
        ::tauri_bindgen_guest_rust::wasm_bindgen_futures::spawn_local(async {
            if ::tauri_bindgen_guest_rust::invoke::<_, ()>(
                "plugin:strings|16c3ebd2deefea81065e2001501951a6",
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

    pub async fn a(x: &str) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            x: &'a str,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:strings|a", &params)
            .await
            .unwrap()
    }
    pub async fn b() -> String {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:strings|b", ())
            .await
            .unwrap()
    }
    pub async fn c(a: &str, b: &str) -> String {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            a: &'a str,
            b: &'a str,
        }
        let params = Params { a, b };
        ::tauri_bindgen_guest_rust::invoke("plugin:strings|c", &params)
            .await
            .unwrap()
    }
}
