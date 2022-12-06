#[allow(clippy::all, unused)]
pub mod variants {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum E1 {
        A,
    }
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Empty {}
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize)]
    pub enum V1Param<'a> {
        A,
        B(U1),
        C(E1),
        D(&'a str),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(Debug, Clone, PartialEq, ::serde::Deserialize)]
    pub enum V1Result {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum Casts1 {
        A(i32),
        B(f32),
    }
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum Casts2 {
        A(f64),
        B(f32),
    }
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum Casts3 {
        A(f64),
        B(u64),
    }
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum Casts4 {
        A(u32),
        B(i64),
    }
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum Casts5 {
        A(f32),
        B(i64),
    }
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Serialize, ::serde::Deserialize)]
    pub enum Casts6 {
        A((f32, u32)),
        B((u32, u32)),
    }
    #[repr(u8)]
    #[derive(Debug, Clone, Copy, PartialEq, ::serde::Deserialize)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(Debug, Clone, PartialEq, ::serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct IsCloneParam<'a> {
        #[serde(borrow)]
        pub v1: V1Param<'a>,
    }
    #[derive(Debug, Clone, PartialEq, ::serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct IsCloneResult {
        pub v1: V1Result,
    }
    pub async fn e1_arg(x: E1) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: E1,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|e1-arg", &params)
            .await
            .unwrap()
    }
    pub async fn e1_result() -> E1 {
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|e1-result", ())
            .await
            .unwrap()
    }
    pub async fn u1_arg(x: U1) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: U1,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|u1-arg", &params)
            .await
            .unwrap()
    }
    pub async fn u1_result() -> U1 {
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|u1-result", ())
            .await
            .unwrap()
    }
    pub async fn v1_arg(x: V1Param<'_>) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            x: V1Param<'a>,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|v1-arg", &params)
            .await
            .unwrap()
    }
    pub async fn v1_result() -> V1Result {
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|v1-result", ())
            .await
            .unwrap()
    }
    pub async fn bool_arg(x: bool) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            x: bool,
        }
        let params = Params { x };
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|bool-arg", &params)
            .await
            .unwrap()
    }
    pub async fn bool_result() -> bool {
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|bool-result", ())
            .await
            .unwrap()
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
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            a: Option<bool>,
            b: Option<()>,
            c: Option<u32>,
            d: Option<E1>,
            e: Option<f32>,
            f: Option<U1>,
            g: Option<Option<bool>>,
        }
        let params = Params {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|option-arg", &params)
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
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|option-result", ())
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
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params {
            a: Casts1,
            b: Casts2,
            c: Casts3,
            d: Casts4,
            e: Casts5,
            f: Casts6,
        }
        let params = Params { a, b, c, d, e, f };
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|casts", &params)
            .await
            .unwrap()
    }
    pub async fn result_arg(
        a: Result<(), ()>,
        b: Result<(), E1>,
        c: Result<E1, ()>,
        d: Result<(), ()>,
        e: Result<u32, V1Param<'_>>,
        f: Result<&str, &[u8]>,
    ) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            a: Result<(), ()>,
            b: Result<(), E1>,
            c: Result<E1, ()>,
            d: Result<(), ()>,
            e: Result<u32, V1Param<'a>>,
            f: Result<&'a str, &'a [u8]>,
        }
        let params = Params { a, b, c, d, e, f };
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|result-arg", &params)
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
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|result-result", ())
            .await
            .unwrap()
    }
    pub async fn return_result_sugar() -> Result<i32, MyErrno> {
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|return-result-sugar", ())
            .await
            .unwrap()
    }
    pub async fn return_result_sugar2() -> Result<(), MyErrno> {
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|return-result-sugar2", ())
            .await
            .unwrap()
    }
    pub async fn return_result_sugar3() -> Result<MyErrno, MyErrno> {
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|return-result-sugar3", ())
            .await
            .unwrap()
    }
    pub async fn return_result_sugar4() -> Result<(i32, u32), MyErrno> {
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|return-result-sugar4", ())
            .await
            .unwrap()
    }
    pub async fn return_option_sugar() -> Option<i32> {
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|return-option-sugar", ())
            .await
            .unwrap()
    }
    pub async fn return_option_sugar2() -> Option<MyErrno> {
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|return-option-sugar2", ())
            .await
            .unwrap()
    }
    pub async fn result_simple() -> Result<u32, i32> {
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|result-simple", ())
            .await
            .unwrap()
    }
    pub async fn is_clone_arg(a: IsCloneParam<'_>) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            a: IsCloneParam<'a>,
        }
        let params = Params { a };
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|is-clone-arg", &params)
            .await
            .unwrap()
    }
    pub async fn is_clone_return() -> IsCloneResult {
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|is-clone-return", ())
            .await
            .unwrap()
    }
    pub async fn return_named_option() -> Option<u8> {
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|return-named-option", ())
            .await
            .unwrap()
    }
    pub async fn return_named_result() -> Result<u8, MyErrno> {
        ::tauri_bindgen_guest_rust::invoke("plugin:8178d1f91285bbc1|return-named-result", ())
            .await
            .unwrap()
    }
}
