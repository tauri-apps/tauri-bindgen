#[allow(clippy::all, unused)]
pub mod import_unions {

    #[cfg(debug_assertions)]
    static START: ::std::sync::Once = ::std::sync::Once::new();
    #[cfg(debug_assertions)]
    fn check_idl_version() {
        ::tauri_bindgen_guest_rust::wasm_bindgen_futures::spawn_local(async {
            if ::tauri_bindgen_guest_rust::invoke::<_, ()>(
                "plugin:unions|cccf67b47414af61861a06498c06cf03",
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
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: AllIntegers,
        }
        let params = Params { num };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|add-one-integer", &params)
            .await
            .unwrap()
    }
    pub async fn add_one_float(num: AllFloats) -> AllFloats {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: AllFloats,
        }
        let params = Params { num };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|add-one-float", &params)
            .await
            .unwrap()
    }
    pub async fn replace_first_char(text: AllTextParam<'_>, letter: char) -> AllTextResult {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            text: AllTextParam<'a>,
            letter: char,
        }
        let params = Params { text, letter };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|replace-first-char", &params)
            .await
            .unwrap()
    }
    pub async fn identify_integer(num: AllIntegers) -> u8 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: AllIntegers,
        }
        let params = Params { num };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|identify-integer", &params)
            .await
            .unwrap()
    }
    pub async fn identify_float(num: AllFloats) -> u8 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: AllFloats,
        }
        let params = Params { num };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|identify-float", &params)
            .await
            .unwrap()
    }
    pub async fn identify_text(text: AllTextParam<'_>) -> u8 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            text: AllTextParam<'a>,
        }
        let params = Params { text };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|identify-text", &params)
            .await
            .unwrap()
    }
    pub async fn add_one_duplicated(num: DuplicatedS32) -> DuplicatedS32 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: DuplicatedS32,
        }
        let params = Params { num };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|add-one-duplicated", &params)
            .await
            .unwrap()
    }
    pub async fn identify_duplicated(num: DuplicatedS32) -> u8 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: DuplicatedS32,
        }
        let params = Params { num };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|identify-duplicated", &params)
            .await
            .unwrap()
    }
    pub async fn add_one_distinguishable_num(num: DistinguishableNum) -> DistinguishableNum {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: DistinguishableNum,
        }
        let params = Params { num };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|add-one-distinguishable-num", &params)
            .await
            .unwrap()
    }
    pub async fn identify_distinguishable_num(num: DistinguishableNum) -> u8 {
        #[cfg(debug_assertions)]
        START.call_once(check_idl_version);
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: DistinguishableNum,
        }
        let params = Params { num };
        ::tauri_bindgen_guest_rust::invoke("plugin:unions|identify-distinguishable-num", &params)
            .await
            .unwrap()
    }
}
