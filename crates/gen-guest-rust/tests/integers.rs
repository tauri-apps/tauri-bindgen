#[allow(clippy::all, unused)]
pub mod imports {
    const IDL_HASH: &str = "279b557e344c2e05";
    pub async fn a1(x: u8) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: u8,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a1", &params).await
    }
    pub async fn a2(x: i8) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: i8,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a2", &params).await
    }
    pub async fn a3(x: u16) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: u16,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a3", &params).await
    }
    pub async fn a4(x: i16) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: i16,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a4", &params).await
    }
    pub async fn a5(x: u32) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: u32,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a5", &params).await
    }
    pub async fn a6(x: i32) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: i32,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a6", &params).await
    }
    pub async fn a7(x: u64) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: u64,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a7", &params).await
    }
    pub async fn a8(x: i64) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: i64,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a8", &params).await
    }
    pub async fn a9(p1: u8, p2: i8, p3: u16, p4: i16, p5: u32, p6: i32, p7: u64, p8: i64) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
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
            idl_hash: IDL_HASH,
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
            p8,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|a9", &params).await
    }
    pub async fn r1() -> u8 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r1", ()).await
    }
    pub async fn r2() -> i8 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r2", ()).await
    }
    pub async fn r3() -> u16 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r3", ()).await
    }
    pub async fn r4() -> i16 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r4", ()).await
    }
    pub async fn r5() -> u32 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r5", ()).await
    }
    pub async fn r6() -> i32 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r6", ()).await
    }
    pub async fn r7() -> u64 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r7", ()).await
    }
    pub async fn r8() -> i64 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|r8", ()).await
    }
    pub async fn pair_ret() -> (i64, u8) {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:integers|pair-ret", ()).await
    }
}
