#[allow(unused_imports, unused_variables)]
pub mod variants {
    use ::tauri_bindgen_host::bitflags;
    use ::tauri_bindgen_host::serde;
    #[derive(serde::Deserialize, serde::Serialize)]
    pub enum E1 {
        A,
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct Empty {}
    #[derive(serde::Deserialize, serde::Serialize)]
    pub enum V1<'a> {
        A,
        B(U1),
        C(E1),
        D(&'a str),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    pub enum Casts1 {
        A(i32),
        B(f32),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    pub enum Casts2 {
        A(f64),
        B(f32),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    pub enum Casts3 {
        A(f64),
        B(u64),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    pub enum Casts4 {
        A(u32),
        B(i64),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    pub enum Casts5 {
        A(f32),
        B(i64),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    pub enum Casts6 {
        A((f32, u32)),
        B((u32, u32)),
    }
    #[derive(serde::Serialize)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct IsClone<'a> {
        #[serde(borrow)]
        v1: V1<'a>,
    }
    pub trait Variants: Sized {
        fn e1_arg(&mut self, x: E1) -> ();
        fn e1_result(&mut self) -> E1;
        fn u1_arg(&mut self, x: U1) -> ();
        fn u1_result(&mut self) -> U1;
        fn v1_arg(&mut self, x: V1) -> ();
        fn v1_result(&mut self) -> V1<'_>;
        fn bool_arg(&mut self, x: bool) -> ();
        fn bool_result(&mut self) -> bool;
        fn option_arg(
            &mut self,
            a: Option<bool>,
            b: Option<()>,
            c: Option<u32>,
            d: Option<E1>,
            e: Option<f32>,
            f: Option<U1>,
            g: Option<Option<bool>>,
        ) -> ();
        fn option_result(
            &mut self,
        ) -> (
            Option<bool>,
            Option<()>,
            Option<u32>,
            Option<E1>,
            Option<f32>,
            Option<U1>,
            Option<Option<bool>>,
        );
        fn casts(
            &mut self,
            a: Casts1,
            b: Casts2,
            c: Casts3,
            d: Casts4,
            e: Casts5,
            f: Casts6,
        ) -> (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6);
        fn result_arg(
            &mut self,
            a: Result<(), ()>,
            b: Result<(), E1>,
            c: Result<E1, ()>,
            d: Result<(), ()>,
            e: Result<u32, V1>,
            f: Result<String, Vec<u8>>,
        ) -> ();
        fn result_result(
            &mut self,
        ) -> (
            Result<(), ()>,
            Result<(), E1>,
            Result<E1, ()>,
            Result<(), ()>,
            Result<u32, V1<'_>>,
            Result<&'_ str, &'_ [u8]>,
        );
        fn return_result_sugar(&mut self) -> Result<i32, MyErrno>;
        fn return_result_sugar2(&mut self) -> Result<(), MyErrno>;
        fn return_result_sugar3(&mut self) -> Result<MyErrno, MyErrno>;
        fn return_result_sugar4(&mut self) -> Result<(i32, u32), MyErrno>;
        fn return_option_sugar(&mut self) -> Option<i32>;
        fn return_option_sugar2(&mut self) -> Option<MyErrno>;
        fn result_simple(&mut self) -> Result<u32, i32>;
        fn is_clone_arg(&mut self, a: IsClone) -> ();
        fn is_clone_return(&mut self) -> IsClone<'_>;
        fn return_named_option(&mut self) -> Option<u8>;
        fn return_named_result(&mut self) -> Result<u8, MyErrno>;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Variants,
    {
        Ok(())
    }
}
