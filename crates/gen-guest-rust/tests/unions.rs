#[allow(clippy::all, unused)]
pub mod import_unions {

    #[::tauri_bindgen_guest_rust::wasm_bindgen::prelude::wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
        pub async fn invoke(
            cmd: ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue,
            args: ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue,
        ) -> ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue;
    }

    /// A union of all of the integral types
    #[derive(
        Debug,
        Clone,
        Copy,
        ::tauri_bindgen_guest_rust::serde::Serialize,
        ::tauri_bindgen_guest_rust::serde::Deserialize,
    )]
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
    #[derive(
        Debug,
        Clone,
        Copy,
        ::tauri_bindgen_guest_rust::serde::Serialize,
        ::tauri_bindgen_guest_rust::serde::Deserialize,
    )]
    pub enum AllFloats {
        F32(f32),
        F64(f64),
    }
    #[derive(Debug, Clone, ::tauri_bindgen_guest_rust::serde::Serialize)]
    pub enum AllTextParam<'a> {
        Char(char),
        String(&'a str),
    }
    #[derive(Debug, Clone, ::tauri_bindgen_guest_rust::serde::Deserialize)]
    pub enum AllTextResult {
        Char(char),
        String(String),
    }
    #[derive(
        Debug,
        Clone,
        Copy,
        ::tauri_bindgen_guest_rust::serde::Serialize,
        ::tauri_bindgen_guest_rust::serde::Deserialize,
    )]
    pub enum DuplicatedS32 {
        /// The first s32
        I320(i32),
        /// The second s32
        I321(i32),
        /// The third s32
        I322(i32),
    }
    /// A type containing numeric types that are distinct in most languages
    #[derive(
        Debug,
        Clone,
        Copy,
        ::tauri_bindgen_guest_rust::serde::Serialize,
        ::tauri_bindgen_guest_rust::serde::Deserialize,
    )]
    pub enum DistinguishableNum {
        /// A Floating Point Number
        F64(f64),
        /// A Signed Integer
        I64(i64),
    }
    pub async fn add_one_integer(num: AllIntegers) -> AllIntegers {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: AllIntegers,
        }
        let params = Params { num };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str(
                "plugin:import_unions|add_one_integer",
            ),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn add_one_float(num: AllFloats) -> AllFloats {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: AllFloats,
        }
        let params = Params { num };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str(
                "plugin:import_unions|add_one_float",
            ),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn replace_first_char(text: AllTextParam<'_>, letter: char) -> AllTextResult {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            text: AllTextParam<'a>,
            letter: char,
        }
        let params = Params { text, letter };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str(
                "plugin:import_unions|replace_first_char",
            ),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn identify_integer(num: AllIntegers) -> u8 {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: AllIntegers,
        }
        let params = Params { num };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str(
                "plugin:import_unions|identify_integer",
            ),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn identify_float(num: AllFloats) -> u8 {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: AllFloats,
        }
        let params = Params { num };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str(
                "plugin:import_unions|identify_float",
            ),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn identify_text(text: AllTextParam<'_>) -> u8 {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            text: AllTextParam<'a>,
        }
        let params = Params { text };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str(
                "plugin:import_unions|identify_text",
            ),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn add_one_duplicated(num: DuplicatedS32) -> DuplicatedS32 {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: DuplicatedS32,
        }
        let params = Params { num };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str(
                "plugin:import_unions|add_one_duplicated",
            ),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn identify_duplicated(num: DuplicatedS32) -> u8 {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: DuplicatedS32,
        }
        let params = Params { num };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str(
                "plugin:import_unions|identify_duplicated",
            ),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn add_one_distinguishable_num(num: DistinguishableNum) -> DistinguishableNum {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: DistinguishableNum,
        }
        let params = Params { num };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str(
                "plugin:import_unions|add_one_distinguishable_num",
            ),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
    pub async fn identify_distinguishable_num(num: DistinguishableNum) -> u8 {
        #[derive(::tauri_bindgen_guest_rust::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            num: DistinguishableNum,
        }
        let params = Params { num };
        let raw = invoke(
            ::tauri_bindgen_guest_rust::wasm_bindgen::JsValue::from_str(
                "plugin:import_unions|identify_distinguishable_num",
            ),
            ::tauri_bindgen_guest_rust::serde_wasm_bindgen::to_value(&params).unwrap(),
        )
        .await;
        ::tauri_bindgen_guest_rust::serde_wasm_bindgen::from_value(raw).unwrap()
    }
}
