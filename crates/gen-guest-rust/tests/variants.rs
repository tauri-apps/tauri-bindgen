#[allow(clippy::all, unused)]
pub mod imports {
    const IDL_HASH: &str = "d5901a6520084a85";
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
        struct Params<'a> {
            idl_hash: &'a str,
            x: E1,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|e1-arg", &params).await
    }
    pub async fn e1_result() -> E1 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|e1-result", ()).await
    }
    pub async fn u1_arg(x: U1) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: U1,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|u1-arg", &params).await
    }
    pub async fn u1_result() -> U1 {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|u1-result", ()).await
    }
    pub async fn v1_arg(x: V1Param<'_>) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: V1Param<'a>,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|v1-arg", &params).await
    }
    pub async fn v1_result() -> V1Result {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|v1-result", ()).await
    }
    pub async fn bool_arg(x: bool) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            x: bool,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            x,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|bool-arg", &params).await
    }
    pub async fn bool_result() -> bool {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|bool-result", ()).await
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
        struct Params<'a> {
            idl_hash: &'a str,
            a: Option<bool>,
            b: Option<()>,
            c: Option<u32>,
            d: Option<E1>,
            e: Option<f32>,
            f: Option<U1>,
            g: Option<Option<bool>>,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            a,
            b,
            c,
            d,
            e,
            f,
            g,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|option-arg", &params).await
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
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|option-result", ()).await
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
        struct Params<'a> {
            idl_hash: &'a str,
            a: Casts1,
            b: Casts2,
            c: Casts3,
            d: Casts4,
            e: Casts5,
            f: Casts6,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            a,
            b,
            c,
            d,
            e,
            f,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|casts", &params).await
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
            idl_hash: &'a str,
            a: Result<(), ()>,
            b: Result<(), E1>,
            c: Result<E1, ()>,
            d: Result<(), ()>,
            e: Result<u32, V1Param<'a>>,
            f: Result<&'a str, &'a [u8]>,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            a,
            b,
            c,
            d,
            e,
            f,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|result-arg", &params).await
    }
    pub async fn result_result() -> (
        Result<(), ()>,
        Result<(), E1>,
        Result<E1, ()>,
        Result<(), ()>,
        Result<u32, V1Result>,
        Result<String, Vec<u8>>,
    ) {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|result-result", ()).await
    }
    pub async fn return_result_sugar() -> Result<i32, MyErrno> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|return-result-sugar", ()).await
    }
    pub async fn return_result_sugar2() -> Result<(), MyErrno> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|return-result-sugar2", ()).await
    }
    pub async fn return_result_sugar3() -> Result<MyErrno, MyErrno> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|return-result-sugar3", ()).await
    }
    pub async fn return_result_sugar4() -> Result<(i32, u32), MyErrno> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|return-result-sugar4", ()).await
    }
    pub async fn return_option_sugar() -> Option<i32> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|return-option-sugar", ()).await
    }
    pub async fn return_option_sugar2() -> Option<MyErrno> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|return-option-sugar2", ()).await
    }
    pub async fn result_simple() -> Result<u32, i32> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|result-simple", ()).await
    }
    pub async fn is_clone_arg(a: IsCloneParam<'_>) -> () {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
            a: IsCloneParam<'a>,
        }
        let params = Params {
            idl_hash: IDL_HASH,
            a,
        };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|is-clone-arg", &params).await
    }
    pub async fn is_clone_return() -> IsCloneResult {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|is-clone-return", ()).await
    }
    pub async fn return_named_option() -> Option<u8> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|return-named-option", ()).await
    }
    pub async fn return_named_result() -> Result<u8, MyErrno> {
        #[derive(::serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Params<'a> {
            idl_hash: &'a str,
        }
        let params = Params { idl_hash: IDL_HASH };
        ::tauri_bindgen_guest_rust::invoke("plugin:variants|return-named-result", ()).await
    }
}
