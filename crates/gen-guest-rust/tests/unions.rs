#[allow(clippy::all, unused)]
pub mod import_unions {
    const IDL_HASH: &str = "cccf67b47414af61";
    /// A union of all of the integral types
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum AllIntegers {
        /// Bool is equivalent to a 1 bit integer
        /// and is treated that way in some languages
        Bool(bool),
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        I8(i8),
        I16(i16),
        I32(i32),
        I64(i64),
    }
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum AllFloats {
        F32(f32),
        F64(f64),
    }
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize)]
    pub enum AllTextParam<'a> {
        Char(char),
        String(&'a str),
    }
    #[derive(Debug, Clone, PartialEq, ::serde::Deserialize)]
    pub enum AllTextResult {
        Char(char),
        String(String),
    }
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum DuplicatedS32 {
        /// The first s32
        I320(i32),
        /// The second s32
        I321(i32),
        /// The third s32
        I322(i32),
    }
    /// A type containing numeric types that are distinct in most languages
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum DistinguishableNum {
        /// A Floating Point Number
        F64(f64),
        /// A Signed Integer
        I64(i64),
    }
    pub async fn add_one_integer(num: AllIntegers) -> AllIntegers {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            num: AllIntegers,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            num,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|add-one-integer", &params).await
    }
    pub async fn add_one_float(num: AllFloats) -> AllFloats {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            num: AllFloats,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            num,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|add-one-float", &params).await
    }
    pub async fn replace_first_char(text: AllTextParam<'_>, letter: char) -> AllTextResult {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            text: AllTextParam<'a>,
            letter: char,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            text,
            letter,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|replace-first-char", &params).await
    }
    pub async fn identify_integer(num: AllIntegers) -> u8 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            num: AllIntegers,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            num,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|identify-integer", &params).await
    }
    pub async fn identify_float(num: AllFloats) -> u8 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            num: AllFloats,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            num,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|identify-float", &params).await
    }
    pub async fn identify_text(text: AllTextParam<'_>) -> u8 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            text: AllTextParam<'a>,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            text,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|identify-text", &params).await
    }
    pub async fn add_one_duplicated(num: DuplicatedS32) -> DuplicatedS32 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            num: DuplicatedS32,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            num,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|add-one-duplicated", &params).await
    }
    pub async fn identify_duplicated(num: DuplicatedS32) -> u8 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            num: DuplicatedS32,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            num,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|identify-duplicated", &params).await
    }
    pub async fn add_one_distinguishable_num(num: DistinguishableNum) -> DistinguishableNum {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            num: DistinguishableNum,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            num,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|add-one-distinguishable-num", &params)
            .await
    }
    pub async fn identify_distinguishable_num(num: DistinguishableNum) -> u8 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            num: DistinguishableNum,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            num,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|identify-distinguishable-num", &params)
            .await
    }
}
