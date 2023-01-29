#[allow(clippy::all, unused)]
#[rustfmt::skip]
pub mod records{
  use ::tauri_bindgen_guest_rust::tauri_bindgen_abi;
  #[repr(C)]
  #[derive(Debug, Copy, Clone, PartialEq)]
  #[derive(tauri_bindgen_abi::Writable, tauri_bindgen_abi::Readable)]
  pub struct Empty {
  }
  /// A record containing two scalar fields that both have the same type
  #[repr(C)]
  #[derive(Debug, Copy, Clone, PartialEq)]
  #[derive(tauri_bindgen_abi::Writable, tauri_bindgen_abi::Readable)]
  pub struct Scalars {
    /// The first field, named a
    pub a: u32,
    /// The second field, named b
    pub b: u32,
  }
  /// A record that is really just flags All of the fields are bool
  #[repr(C)]
  #[derive(Debug, Copy, Clone, PartialEq)]
  #[derive(tauri_bindgen_abi::Writable, tauri_bindgen_abi::Readable)]
  pub struct ReallyFlags {
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub d: bool,
    pub e: bool,
    pub f: bool,
    pub g: bool,
    pub h: bool,
    pub i: bool,
  }
  #[derive(Debug, Clone, PartialEq)]
  #[derive(tauri_bindgen_abi::Writable)]
  pub struct AggregatesParam<'a,> {
    pub a: Scalars,
    pub b: u32,
    pub c: Empty,
    pub d: &'a str,
    pub e: ReallyFlags,
  }
  #[derive(Debug, Clone, PartialEq)]
  #[derive(tauri_bindgen_abi::Readable)]
  pub struct AggregatesResult {
    pub a: Scalars,
    pub b: u32,
    pub c: Empty,
    pub d: String,
    pub e: ReallyFlags,
  }
  pub type IntTypedef = i32;
  pub type TupleTypedef2 = (IntTypedef,);
  pub async fn tuple_arg(x: (char,u32,),) -> () {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params {
      x : (char,u32,),
    }
    let params = Params {x,};
    ::tauri_bindgen_guest_rust::invoke("plugin:ac98167d7d43eb21|tuple-arg", &params).await.unwrap()
  }
  pub async fn tuple_result() -> (char,u32,) {
    ::tauri_bindgen_guest_rust::invoke("plugin:ac98167d7d43eb21|tuple-result", ()).await.unwrap()
  }
  pub async fn empty_arg(x: Empty,) -> () {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params {
      x : Empty,
    }
    let params = Params {x,};
    ::tauri_bindgen_guest_rust::invoke("plugin:ac98167d7d43eb21|empty-arg", &params).await.unwrap()
  }
  pub async fn empty_result() -> Empty {
    ::tauri_bindgen_guest_rust::invoke("plugin:ac98167d7d43eb21|empty-result", ()).await.unwrap()
  }
  pub async fn scalar_arg(x: Scalars,) -> () {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params {
      x : Scalars,
    }
    let params = Params {x,};
    ::tauri_bindgen_guest_rust::invoke("plugin:ac98167d7d43eb21|scalar-arg", &params).await.unwrap()
  }
  pub async fn scalar_result() -> Scalars {
    ::tauri_bindgen_guest_rust::invoke("plugin:ac98167d7d43eb21|scalar-result", ()).await.unwrap()
  }
  pub async fn flags_arg(x: ReallyFlags,) -> () {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params {
      x : ReallyFlags,
    }
    let params = Params {x,};
    ::tauri_bindgen_guest_rust::invoke("plugin:ac98167d7d43eb21|flags-arg", &params).await.unwrap()
  }
  pub async fn flags_result() -> ReallyFlags {
    ::tauri_bindgen_guest_rust::invoke("plugin:ac98167d7d43eb21|flags-result", ()).await.unwrap()
  }
  pub async fn aggregate_arg(x: AggregatesParam<'_,>,) -> () {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params<'a,> {
      x : AggregatesParam<'a,>,
    }
    let params = Params {x,};
    ::tauri_bindgen_guest_rust::invoke("plugin:ac98167d7d43eb21|aggregate-arg", &params).await.unwrap()
  }
  pub async fn aggregate_result() -> AggregatesResult {
    ::tauri_bindgen_guest_rust::invoke("plugin:ac98167d7d43eb21|aggregate-result", ()).await.unwrap()
  }
  pub async fn typedef_inout(e: TupleTypedef2,) -> i32 {
    #[derive(Debug, tauri_bindgen_abi::Writable)]
    struct Params {
      e : TupleTypedef2,
    }
    let params = Params {e,};
    ::tauri_bindgen_guest_rust::invoke("plugin:ac98167d7d43eb21|typedef-inout", &params).await.unwrap()
  }
  
}
