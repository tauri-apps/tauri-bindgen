#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod variants {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum E1 {
        A,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct Empty {}
    #[derive(serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum V1Param<'a> {
        A,
        B(U1),
        C(E1),
        D(&'a str),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum V1Result {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts1 {
        A(i32),
        B(f32),
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts2 {
        A(f64),
        B(f32),
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts3 {
        A(f64),
        B(u64),
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts4 {
        A(u32),
        B(i64),
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts5 {
        A(f32),
        B(i64),
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts6 {
        A((f32, u32)),
        B((u32, u32)),
    }
    #[derive(serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct IsCloneParam<'a> {
        #[serde(borrow)]
        pub v1: V1Param<'a>,
    }
    #[derive(serde::Deserialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct IsCloneResult {
        pub v1: V1Result,
    }
    pub async fn e1_arg(x: E1) {
        ::tauri_bindgen_guest_rust::invoke("variants", "e1_arg", &(x)).await.unwrap()
    }
    pub async fn e1_result() -> E1 {
        ::tauri_bindgen_guest_rust::invoke("variants", "e1_result", &()).await.unwrap()
    }
    pub async fn u1_arg(x: U1) {
        ::tauri_bindgen_guest_rust::invoke("variants", "u1_arg", &(x)).await.unwrap()
    }
    pub async fn u1_result() -> U1 {
        ::tauri_bindgen_guest_rust::invoke("variants", "u1_result", &()).await.unwrap()
    }
    pub async fn v1_arg(x: V1Param<'_>) {
        ::tauri_bindgen_guest_rust::invoke("variants", "v1_arg", &(x)).await.unwrap()
    }
    pub async fn v1_result() -> V1Result {
        ::tauri_bindgen_guest_rust::invoke("variants", "v1_result", &()).await.unwrap()
    }
    pub async fn bool_arg(x: bool) {
        ::tauri_bindgen_guest_rust::invoke("variants", "bool_arg", &(x)).await.unwrap()
    }
    pub async fn bool_result() -> bool {
        ::tauri_bindgen_guest_rust::invoke("variants", "bool_result", &()).await.unwrap()
    }
    pub async fn option_arg(
        a: Option<bool>,
        b: Option<()>,
        c: Option<u32>,
        d: Option<E1>,
        e: Option<f32>,
        f: Option<U1>,
        g: Option<Option<bool>>,
    ) {
        ::tauri_bindgen_guest_rust::invoke(
                "variants",
                "option_arg",
                &(a, b, c, d, e, f, g),
            )
            .await
            .unwrap()
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
        ::tauri_bindgen_guest_rust::invoke("variants", "option_result", &())
            .await
            .unwrap()
    }
    pub async fn casts(
        a: Casts1,
        b: Casts2,
        c: Casts3,
        d: Casts4,
        e: Casts5,
        f: Casts6,
    ) -> (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6) {
        ::tauri_bindgen_guest_rust::invoke("variants", "casts", &(a, b, c, d, e, f))
            .await
            .unwrap()
    }
    pub async fn result_arg(
        a: Result<(), ()>,
        b: Result<(), E1>,
        c: Result<E1, ()>,
        d: Result<(), ()>,
        e: Result<u32, V1Param<'_>>,
        f: Result<&'_ str, &'_ [u8]>,
    ) {
        ::tauri_bindgen_guest_rust::invoke("variants", "result_arg", &(a, b, c, d, e, f))
            .await
            .unwrap()
    }
    pub async fn result_result() -> (
        Result<(), ()>,
        Result<(), E1>,
        Result<E1, ()>,
        Result<(), ()>,
        Result<u32, V1Result>,
        Result<String, Vec<u8>>,
    ) {
        ::tauri_bindgen_guest_rust::invoke("variants", "result_result", &())
            .await
            .unwrap()
    }
    pub async fn return_result_sugar() -> Result<i32, MyErrno> {
        ::tauri_bindgen_guest_rust::invoke("variants", "return_result_sugar", &())
            .await
            .unwrap()
    }
    pub async fn return_result_sugar2() -> Result<(), MyErrno> {
        ::tauri_bindgen_guest_rust::invoke("variants", "return_result_sugar2", &())
            .await
            .unwrap()
    }
    pub async fn return_result_sugar3() -> Result<MyErrno, MyErrno> {
        ::tauri_bindgen_guest_rust::invoke("variants", "return_result_sugar3", &())
            .await
            .unwrap()
    }
    pub async fn return_result_sugar4() -> Result<(i32, u32), MyErrno> {
        ::tauri_bindgen_guest_rust::invoke("variants", "return_result_sugar4", &())
            .await
            .unwrap()
    }
    pub async fn return_option_sugar() -> Option<i32> {
        ::tauri_bindgen_guest_rust::invoke("variants", "return_option_sugar", &())
            .await
            .unwrap()
    }
    pub async fn return_option_sugar2() -> Option<MyErrno> {
        ::tauri_bindgen_guest_rust::invoke("variants", "return_option_sugar2", &())
            .await
            .unwrap()
    }
    pub async fn result_simple() -> Result<u32, i32> {
        ::tauri_bindgen_guest_rust::invoke("variants", "result_simple", &())
            .await
            .unwrap()
    }
    pub async fn is_clone_arg(a: IsCloneParam<'_>) {
        ::tauri_bindgen_guest_rust::invoke("variants", "is_clone_arg", &(a))
            .await
            .unwrap()
    }
    pub async fn is_clone_return() -> IsCloneResult {
        ::tauri_bindgen_guest_rust::invoke("variants", "is_clone_return", &())
            .await
            .unwrap()
    }
    pub async fn return_named_option() -> Option<u8> {
        ::tauri_bindgen_guest_rust::invoke("variants", "return_named_option", &())
            .await
            .unwrap()
    }
    pub async fn return_named_result() -> Result<u8, MyErrno> {
        ::tauri_bindgen_guest_rust::invoke("variants", "return_named_result", &())
            .await
            .unwrap()
    }
}
