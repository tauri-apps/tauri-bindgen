#[allow(clippy::all, unused)]
pub mod imports {

    #[cfg(debug_assertions)]
    static START: ::std::sync::Once = ::std::sync::Once::new();
    #[cfg(debug_assertions)]
    fn check_idl_version() {
        ::tauri_bindgen_guest_rust::wasm_bindgen_futures::spawn_local(async {
            if ::tauri_bindgen_guest_rust::invoke::<_, ()>(
                "plugin:integers|279b557e344c2e05853f5c89d6d511dc",
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

    pub async fn a1(x: u8) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u8,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a1", &params)
            .await
            .unwrap()
    }
    pub async fn a2(x: i8) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i8,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a2", &params)
            .await
            .unwrap()
    }
    pub async fn a3(x: u16) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u16,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a3", &params)
            .await
            .unwrap()
    }
    pub async fn a4(x: i16) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i16,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a4", &params)
            .await
            .unwrap()
    }
    pub async fn a5(x: u32) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u32,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a5", &params)
            .await
            .unwrap()
    }
    pub async fn a6(x: i32) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i32,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a6", &params)
            .await
            .unwrap()
    }
    pub async fn a7(x: u64) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: u64,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a7", &params)
            .await
            .unwrap()
    }
    pub async fn a8(x: i64) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: i64,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a8", &params)
            .await
            .unwrap()
    }
    pub async fn a9(p1: u8, p2: i8, p3: u16, p4: i16, p5: u32, p6: i32, p7: u64, p8: i64) -> () {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            p1: u8,
            p2: i8,
            p3: u16,
            p4: i16,
            p5: u32,
            p6: i32,
            p7: u64,
            p8: i64,
        }
        let params = Params {
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
            p8,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a9", &params)
            .await
            .unwrap()
    }
    pub async fn r1() -> u8 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r1", ())
            .await
            .unwrap()
    }
    pub async fn r2() -> i8 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r2", ())
            .await
            .unwrap()
    }
    pub async fn r3() -> u16 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r3", ())
            .await
            .unwrap()
    }
    pub async fn r4() -> i16 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r4", ())
            .await
            .unwrap()
    }
    pub async fn r5() -> u32 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r5", ())
            .await
            .unwrap()
    }
    pub async fn r6() -> i32 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r6", ())
            .await
            .unwrap()
    }
    pub async fn r7() -> u64 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r7", ())
            .await
            .unwrap()
    }
    pub async fn r8() -> i64 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r8", ())
            .await
            .unwrap()
    }
    pub async fn pair_ret() -> (i64, u8) {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|pair-ret", ())
            .await
            .unwrap()
    }
}
