#[allow(clippy::all, unused)]
#[rustfmt::skip]
pub mod unions{
  use ::tauri_bindgen_guest_rust::tauri_bindgen_abi;
  /// A union of all of the integral types
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(tauri_bindgen_abi::Writable, tauri_bindgen_abi::Readable)]
  pub enum AllIntegers{
    /// Bool is equivalent to a 1 bit integer and is treated that way in some languages
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
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(tauri_bindgen_abi::Writable, tauri_bindgen_abi::Readable)]
  pub enum AllFloats{
    F32(f32),
    F64(f64),
  }
  #[derive(Debug, Clone, PartialEq)]
  #[derive(tauri_bindgen_abi::Writable)]
  pub enum AllTextParam<'a,>{
    Char(char),
    String(&'a str),
  }
  #[derive(Debug, Clone, PartialEq)]
  #[derive(tauri_bindgen_abi::Readable)]
  pub enum AllTextResult{
    Char(char),
    String(String),
  }
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(tauri_bindgen_abi::Writable, tauri_bindgen_abi::Readable)]
  pub enum DuplicatedS32{
    /// The first s32
    I320(i32),
    /// The second s32
    I321(i32),
    /// The third s32
    I322(i32),
  }
  /// A type containing numeric types that are distinct in most languages
  #[derive(Debug, Clone, Copy, PartialEq)]
  #[derive(tauri_bindgen_abi::Writable, tauri_bindgen_abi::Readable)]
  pub enum DistinguishableNum{
    /// A Floating Point Number
    F64(f64),
    /// A Signed Integer
    I64(i64),
  }
  pub async fn add_one_integer(num: AllIntegers,) -> AllIntegers {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params {
      num : AllIntegers,
    }
    let params = Params {num,};
    ::tauri_bindgen_guest_rust::invoke("plugin:37da362e4966fe5b|add-one-integer", &params).await.unwrap()
  }
  pub async fn add_one_float(num: AllFloats,) -> AllFloats {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params {
      num : AllFloats,
    }
    let params = Params {num,};
    ::tauri_bindgen_guest_rust::invoke("plugin:37da362e4966fe5b|add-one-float", &params).await.unwrap()
  }
  pub async fn replace_first_char(text: AllTextParam<'_,>,letter: char,) -> AllTextResult {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params<'a,> {
      text : AllTextParam<'a,>,
      letter : char,
    }
    let params = Params {text,letter,};
    ::tauri_bindgen_guest_rust::invoke("plugin:37da362e4966fe5b|replace-first-char", &params).await.unwrap()
  }
  pub async fn identify_integer(num: AllIntegers,) -> u8 {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params {
      num : AllIntegers,
    }
    let params = Params {num,};
    ::tauri_bindgen_guest_rust::invoke("plugin:37da362e4966fe5b|identify-integer", &params).await.unwrap()
  }
  pub async fn identify_float(num: AllFloats,) -> u8 {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params {
      num : AllFloats,
    }
    let params = Params {num,};
    ::tauri_bindgen_guest_rust::invoke("plugin:37da362e4966fe5b|identify-float", &params).await.unwrap()
  }
  pub async fn identify_text(text: AllTextParam<'_,>,) -> u8 {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params<'a,> {
      text : AllTextParam<'a,>,
    }
    let params = Params {text,};
    ::tauri_bindgen_guest_rust::invoke("plugin:37da362e4966fe5b|identify-text", &params).await.unwrap()
  }
  pub async fn add_one_duplicated(num: DuplicatedS32,) -> DuplicatedS32 {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params {
      num : DuplicatedS32,
    }
    let params = Params {num,};
    ::tauri_bindgen_guest_rust::invoke("plugin:37da362e4966fe5b|add-one-duplicated", &params).await.unwrap()
  }
  pub async fn identify_duplicated(num: DuplicatedS32,) -> u8 {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params {
      num : DuplicatedS32,
    }
    let params = Params {num,};
    ::tauri_bindgen_guest_rust::invoke("plugin:37da362e4966fe5b|identify-duplicated", &params).await.unwrap()
  }
  pub async fn add_one_distinguishable_num(num: DistinguishableNum,) -> DistinguishableNum {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params {
      num : DistinguishableNum,
    }
    let params = Params {num,};
    ::tauri_bindgen_guest_rust::invoke("plugin:37da362e4966fe5b|add-one-distinguishable-num", &params).await.unwrap()
  }
  pub async fn identify_distinguishable_num(num: DistinguishableNum,) -> u8 {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params {
      num : DistinguishableNum,
    }
    let params = Params {num,};
    ::tauri_bindgen_guest_rust::invoke("plugin:37da362e4966fe5b|identify-distinguishable-num", &params).await.unwrap()
  }
  
}
