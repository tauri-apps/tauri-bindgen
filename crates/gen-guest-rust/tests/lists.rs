#[allow(clippy::all, unused)]
pub mod import_lists {
    const IDL_HASH: &str = "a744d1c6fec40184";
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SomeRecordParam<'a> {
        pub x: &'a str,
        #[serde(borrow)]
        pub y: OtherRecordParam<'a>,
        #[serde(borrow)]
        pub z: &'a [OtherRecordParam<'a>],
        pub c1: u32,
        pub c2: u64,
        pub c3: i32,
        pub c4: i64,
    }
    #[derive(Debug, Clone, PartialEq, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SomeRecordResult {
        pub x: String,
        pub y: OtherRecordResult,
        pub z: Vec<OtherRecordResult>,
        pub c1: u32,
        pub c2: u64,
        pub c3: i32,
        pub c4: i64,
    }
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OtherRecordParam<'a> {
        pub a1: u32,
        pub a2: u64,
        pub a3: i32,
        pub a4: i64,
        pub b: &'a str,
        pub c: &'a [u8],
    }
    #[derive(Debug, Clone, PartialEq, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OtherRecordResult {
        pub a1: u32,
        pub a2: u64,
        pub a3: i32,
        pub a4: i64,
        pub b: String,
        pub c: Vec<u8>,
    }
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize)]
    pub enum SomeVariant<'a> {
        A(&'a str),
        B,
        C(u32),
        D(&'a [OtherVariantParam<'a>]),
    }
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize)]
    pub enum OtherVariantParam<'a> {
        A,
        B(u32),
        C(&'a str),
    }
    #[derive(Debug, Clone, PartialEq, ::serde::Deserialize)]
    pub enum OtherVariantResult {
        A,
        B(u32),
        C(String),
    }
    pub type LoadStoreAllSizesParam<'a> = &'a [(
        &'a str,
        u8,
        i8,
        u16,
        i16,
        u32,
        i32,
        u64,
        i64,
        f32,
        f64,
        char,
    )];
    pub type LoadStoreAllSizesResult =
        Vec<(String, u8, i8, u16, i16, u32, i32, u64, i64, f32, f64, char)>;
    pub async fn list_u8_param(x: &[u8]) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [u8],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-u8-param", &params).await
    }
    pub async fn list_u16_param(x: &[u16]) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [u16],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-u16-param", &params).await
    }
    pub async fn list_u32_param(x: &[u32]) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [u32],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-u32-param", &params).await
    }
    pub async fn list_u64_param(x: &[u64]) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [u64],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-u64-param", &params).await
    }
    pub async fn list_s8_param(x: &[i8]) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [i8],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-s8-param", &params).await
    }
    pub async fn list_s16_param(x: &[i16]) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [i16],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-s16-param", &params).await
    }
    pub async fn list_s32_param(x: &[i32]) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [i32],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-s32-param", &params).await
    }
    pub async fn list_s64_param(x: &[i64]) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [i64],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-s64-param", &params).await
    }
    pub async fn list_float32_param(x: &[f32]) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [f32],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-float32-param", &params).await
    }
    pub async fn list_float64_param(x: &[f64]) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [f64],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-float64-param", &params).await
    }
    pub async fn list_u8_ret() -> Vec<u8> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-u8-ret", ()).await
    }
    pub async fn list_u16_ret() -> Vec<u16> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-u16-ret", ()).await
    }
    pub async fn list_u32_ret() -> Vec<u32> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-u32-ret", ()).await
    }
    pub async fn list_u64_ret() -> Vec<u64> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-u64-ret", ()).await
    }
    pub async fn list_s8_ret() -> Vec<i8> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-s8-ret", ()).await
    }
    pub async fn list_s16_ret() -> Vec<i16> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-s16-ret", ()).await
    }
    pub async fn list_s32_ret() -> Vec<i32> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-s32-ret", ()).await
    }
    pub async fn list_s64_ret() -> Vec<i64> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-s64-ret", ()).await
    }
    pub async fn list_float32_ret() -> Vec<f32> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-float32-ret", ()).await
    }
    pub async fn list_float64_ret() -> Vec<f64> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|list-float64-ret", ()).await
    }
    pub async fn tuple_list(x: &[(u8, i8)]) -> Vec<(i64, u32)> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [(u8, i8)],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|tuple-list", &params).await
    }
    pub async fn string_list_arg(a: &[&str]) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            a: &'a [&'a str],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            a,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|string-list-arg", &params).await
    }
    pub async fn string_list_ret() -> Vec<String> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|string-list-ret", ()).await
    }
    pub async fn tuple_string_list(x: &[(u8, &str)]) -> Vec<(String, u8)> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [(u8, &'a str)],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|tuple-string-list", &params).await
    }
    pub async fn string_list(x: &[&str]) -> Vec<String> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [&'a str],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|string-list", &params).await
    }
    pub async fn record_list(x: &[SomeRecordParam<'_>]) -> Vec<OtherRecordResult> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [SomeRecordParam<'a>],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|record-list", &params).await
    }
    pub async fn record_list_reverse(x: &[OtherRecordParam<'_>]) -> Vec<SomeRecordResult> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [OtherRecordParam<'a>],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|record-list-reverse", &params).await
    }
    pub async fn variant_list(x: &[SomeVariant<'_>]) -> Vec<OtherVariantResult> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: &'a [SomeVariant<'a>],
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|variant-list", &params).await
    }
    pub async fn load_store_everything(a: LoadStoreAllSizesParam<'_>) -> LoadStoreAllSizesResult {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            a: LoadStoreAllSizesParam<'a>,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            a,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:lists|load-store-everything", &params).await
    }
}
