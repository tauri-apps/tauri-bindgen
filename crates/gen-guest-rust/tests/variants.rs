pub mod variants {
    use ::tauri_bindgen_guest_rust::tauri_bindgen_abi;
    use ::tauri_bindgen_guest_rust::bitflags;
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub struct Empty<'a> {}
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum V1<'a> {
        A,
        B(U1),
        C(E1),
        D(&'a str),
        E(Empty<'a>),
        F,
        G(u32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct Empty {}
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum V1 {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum Casts1 {
        A(i32),
        B(f32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum Casts2 {
        A(f64),
        B(f32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum Casts3 {
        A(f64),
        B(u64),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum Casts4 {
        A(u32),
        B(i64),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum Casts5 {
        A(f32),
        B(i64),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum Casts6 {
        A((f32, u32)),
        B((u32, u32)),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum Casts1 {
        A(i32),
        B(f32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum Casts2 {
        A(f64),
        B(f32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum Casts3 {
        A(f64),
        B(u64),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum Casts4 {
        A(u32),
        B(i64),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum Casts5 {
        A(f32),
        B(i64),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum Casts6 {
        A((f32, u32)),
        B((u32, u32)),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub struct Empty<'a> {}
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum V1<'a> {
        A,
        B(U1),
        C(E1),
        D(&'a str),
        E(Empty<'a>),
        F,
        G(u32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct Empty {}
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum V1 {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub struct Empty<'a> {}
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum V1<'a> {
        A,
        B(U1),
        C(E1),
        D(&'a str),
        E(Empty<'a>),
        F,
        G(u32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub struct IsClone<'a> {
        v1: V1<'a>,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct Empty {}
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum V1 {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct IsClone {
        v1: V1,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    pub async fn e1_arg(x: E1) -> () {
        todo!()
    }
    pub async fn e1_result() -> E1 {
        todo!()
    }
    pub async fn u1_arg(x: U1) -> () {
        todo!()
    }
    pub async fn u1_result() -> U1 {
        todo!()
    }
    pub async fn v1_arg(x: V1<'_>) -> () {
        todo!()
    }
    pub async fn v1_result() -> V1 {
        todo!()
    }
    pub async fn bool_arg(x: bool) -> () {
        todo!()
    }
    pub async fn bool_result() -> bool {
        todo!()
    }
    pub async fn option_arg(
        a: Option<bool>,
        b: Option<()>,
        c: Option<u32>,
        d: Option<E1>,
        e: Option<f32>,
        f: Option<U1>,
        g: Option<Option<bool>>,
    ) -> () {
        todo!()
    }
    pub async fn option_result() -> (
        Option<bool>,
        Option<()>,
        Option<u32>,
        Option<E1>,
        Option<f32>,
        Option<U1>,
        Option<Option<bool>>,
    ) {
        todo!()
    }
    pub async fn casts(
        a: Casts1,
        b: Casts2,
        c: Casts3,
        d: Casts4,
        e: Casts5,
        f: Casts6,
    ) -> (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6) {
        todo!()
    }
    pub async fn result_arg(
        a: Result<(), ()>,
        b: Result<(), E1>,
        c: Result<E1, ()>,
        d: Result<(), ()>,
        e: Result<u32, V1<'_>>,
        f: Result<&'_ str, &'_ [u8]>,
    ) -> () {
        todo!()
    }
    pub async fn result_result() -> (
        Result<(), ()>,
        Result<(), E1>,
        Result<E1, ()>,
        Result<(), ()>,
        Result<u32, V1>,
        Result<String, Vec<u8>>,
    ) {
        todo!()
    }
    pub async fn return_result_sugar() -> Result<i32, MyErrno> {
        todo!()
    }
    pub async fn return_result_sugar2() -> Result<(), MyErrno> {
        todo!()
    }
    pub async fn return_result_sugar3() -> Result<MyErrno, MyErrno> {
        todo!()
    }
    pub async fn return_result_sugar4() -> Result<(i32, u32), MyErrno> {
        todo!()
    }
    pub async fn return_option_sugar() -> Option<i32> {
        todo!()
    }
    pub async fn return_option_sugar2() -> Option<MyErrno> {
        todo!()
    }
    pub async fn result_simple() -> Result<u32, i32> {
        todo!()
    }
    pub async fn is_clone_arg(a: IsClone<'_>) -> () {
        todo!()
    }
    pub async fn is_clone_return() -> IsClone {
        todo!()
    }
    pub async fn return_named_option() -> Option<u8> {
        todo!()
    }
    pub async fn return_named_result() -> Result<u8, MyErrno> {
        todo!()
    }
}
