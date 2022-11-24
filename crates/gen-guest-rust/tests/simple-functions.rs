#[allow(clippy::all, unused)]
pub mod imports {

    #[cfg(debug_assertions)]
    static START: ::std::sync::Once = ::std::sync::Once::new();
    #[cfg(debug_assertions)]
    fn check_idl_version() {
        ::tauri_bindgen_guest_rust::wasm_bindgen_futures::spawn_local(async {
            if ::tauri_bindgen_guest_rust::invoke::<_, ()>(
                "plugin:simple|ebb2d6f0441e00a02915e2faf10bbe90",
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

    pub async fn f1() -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:simple|f1", ())
            .await
            .unwrap()
    }
    pub async fn f2(a: u32) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            a: u32,
        }
        let params = Params { a };
        ::tauri_bindgen_guest_rust::invoke("plugin:simple|f2", &params)
            .await
            .unwrap()
    }
    pub async fn f3(a: u32, b: u32) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            a: u32,
            b: u32,
        }
        let params = Params { a, b };
        ::tauri_bindgen_guest_rust::invoke("plugin:simple|f3", &params)
            .await
            .unwrap()
    }
    pub async fn f4() -> u32 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:simple|f4", ())
            .await
            .unwrap()
    }
    pub async fn f5() -> (u32, u32) {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:simple|f5", ())
            .await
            .unwrap()
    }
    pub async fn f6(a: u32, b: u32, c: u32) -> (u32, u32, u32) {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            a: u32,
            b: u32,
            c: u32,
        }
        let params = Params { a, b, c };
        ::tauri_bindgen_guest_rust::invoke("plugin:simple|f6", &params)
            .await
            .unwrap()
    }
}
