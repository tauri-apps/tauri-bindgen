pub mod unions {
    use ::tauri_bindgen_guest_rust::tauri_bindgen_abi;
    use ::tauri_bindgen_guest_rust::bitflags;
    ///A union of all of the integral types
    #[derive(tauri_bindgen_abi::Writable)]
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
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum AllFloats {
        F32(f32),
        F64(f64),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum AllText<'a> {
        Char(char),
        String(&'a str),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum DuplicatedS32 {
        ///The first s32
        S320(i32),
        ///The second s32
        S321(i32),
        ///The third s32
        S322(i32),
    }
    ///A type containing numeric types that are distinct in most languages
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum DistinguishableNum {
        ///A Floating Point Number
        F64(f64),
        ///A Signed Integer
        S64(i64),
    }
    pub async fn add_one_integer(num: AllIntegers) -> AllIntegers {
        todo!()
    }
    pub async fn add_one_float(num: AllFloats) -> AllFloats {
        todo!()
    }
    pub async fn replace_first_char(text: AllText<'_>, letter: char) -> AllText {
        todo!()
    }
    pub async fn identify_integer(num: AllIntegers) -> u8 {
        todo!()
    }
    pub async fn identify_float(num: AllFloats) -> u8 {
        todo!()
    }
    pub async fn identify_text(text: AllText<'_>) -> u8 {
        todo!()
    }
    pub async fn add_one_duplicated(num: DuplicatedS32) -> DuplicatedS32 {
        todo!()
    }
    pub async fn identify_duplicated(num: DuplicatedS32) -> u8 {
        todo!()
    }
    pub async fn add_one_distinguishable_num(
        num: DistinguishableNum,
    ) -> DistinguishableNum {
        todo!()
    }
    pub async fn identify_distinguishable_num(num: DistinguishableNum) -> u8 {
        todo!()
    }
}
