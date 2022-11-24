#[allow(clippy::all, unused)]
pub mod imports {

    #[cfg(debug_assertions)]
    static START: ::std::sync::Once = ::std::sync::Once::new();
    #[cfg(debug_assertions)]
    fn check_idl_version() {
        ::tauri_bindgen_guest_rust::wasm_bindgen_futures::spawn_local(async {
            if ::tauri_bindgen_guest_rust::invoke::<_, ()>(
                "plugin:floats|b2ded0ef970e65969239249842d626ce",
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

    pub async fn float32_param(x: f32) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: f32,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:floats|float32-param", &params)
            .await
            .unwrap()
    }
    pub async fn float64_param(x: f64) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: f64,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:floats|float64-param", &params)
            .await
            .unwrap()
    }
    pub async fn float32_result() -> f32 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:floats|float32-result", ())
            .await
            .unwrap()
    }
    pub async fn float64_result() -> f64 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:floats|float64-result", ())
            .await
            .unwrap()
    }
}
