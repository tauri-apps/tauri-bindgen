#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod unions {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    ///A union of all of the integral types
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum AllIntegers {
        /**Bool is equivalent to a 1 bit integer
and is treated that way in some languages*/
        Bool(bool),
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        I8(i8),
        I16(i16),
        S32(i32),
        S64(i64),
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum AllFloats {
        F32(f32),
        F64(f64),
    }
    #[derive(serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum AllTextParam<'a> {
        Char(char),
        String(&'a str),
    }
    #[derive(serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum AllTextResult {
        Char(char),
        String(String),
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum DuplicatedS32 {
        ///The first s32
        S320(i32),
        ///The second s32
        S321(i32),
        ///The third s32
        S322(i32),
    }
    ///A type containing numeric types that are distinct in most languages
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum DistinguishableNum {
        ///A Floating Point Number
        F64(f64),
        ///A Signed Integer
        S64(i64),
    }
    pub async fn add_one_integer(num: AllIntegers) -> AllIntegers {
        ::tauri_bindgen_guest_rust::invoke("unions", "add_one_integer", &(num))
            .await
            .unwrap()
    }
    pub async fn add_one_float(num: AllFloats) -> AllFloats {
        ::tauri_bindgen_guest_rust::invoke("unions", "add_one_float", &(num))
            .await
            .unwrap()
    }
    pub async fn replace_first_char(
        text: AllTextParam<'_>,
        letter: char,
    ) -> AllTextResult {
        ::tauri_bindgen_guest_rust::invoke(
                "unions",
                "replace_first_char",
                &(text, letter),
            )
            .await
            .unwrap()
    }
    pub async fn identify_integer(num: AllIntegers) -> u8 {
        ::tauri_bindgen_guest_rust::invoke("unions", "identify_integer", &(num))
            .await
            .unwrap()
    }
    pub async fn identify_float(num: AllFloats) -> u8 {
        ::tauri_bindgen_guest_rust::invoke("unions", "identify_float", &(num))
            .await
            .unwrap()
    }
    pub async fn identify_text(text: AllTextParam<'_>) -> u8 {
        ::tauri_bindgen_guest_rust::invoke("unions", "identify_text", &(text))
            .await
            .unwrap()
    }
    pub async fn add_one_duplicated(num: DuplicatedS32) -> DuplicatedS32 {
        ::tauri_bindgen_guest_rust::invoke("unions", "add_one_duplicated", &(num))
            .await
            .unwrap()
    }
    pub async fn identify_duplicated(num: DuplicatedS32) -> u8 {
        ::tauri_bindgen_guest_rust::invoke("unions", "identify_duplicated", &(num))
            .await
            .unwrap()
    }
    pub async fn add_one_distinguishable_num(
        num: DistinguishableNum,
    ) -> DistinguishableNum {
        ::tauri_bindgen_guest_rust::invoke(
                "unions",
                "add_one_distinguishable_num",
                &(num),
            )
            .await
            .unwrap()
    }
    pub async fn identify_distinguishable_num(num: DistinguishableNum) -> u8 {
        ::tauri_bindgen_guest_rust::invoke(
                "unions",
                "identify_distinguishable_num",
                &(num),
            )
            .await
            .unwrap()
    }
}
