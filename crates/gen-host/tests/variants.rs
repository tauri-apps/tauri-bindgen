#[allow(clippy::all)]
pub mod imports {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy)]
    pub enum E1 {
        A,
    }
    #[derive(Debug, Clone, Copy)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Empty {}
    #[derive(Debug, Clone)]
    pub enum V1 {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(Debug, Clone, Copy)]
    pub enum Casts1 {
        A(i32),
        B(f32),
    }
    #[derive(Debug, Clone, Copy)]
    pub enum Casts2 {
        A(f64),
        B(f32),
    }
    #[derive(Debug, Clone, Copy)]
    pub enum Casts3 {
        A(f64),
        B(u64),
    }
    #[derive(Debug, Clone, Copy)]
    pub enum Casts4 {
        A(u32),
        B(i64),
    }
    #[derive(Debug, Clone, Copy)]
    pub enum Casts5 {
        A(f32),
        B(i64),
    }
    #[derive(Debug, Clone, Copy)]
    pub enum Casts6 {
        A((f32, u32)),
        B((u32, u32)),
    }
    #[repr(u8)]
    #[derive(Debug, Clone, Copy)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(Debug, Clone)]
    pub struct IsClone {
        pub v1: V1,
    }
    #[host::async_trait]
    pub trait Imports: Sized {
        async fn e1_arg(&mut self, x: E1) -> anyhow::Result<()>;
        async fn e1_result(&mut self) -> anyhow::Result<E1>;
        async fn u1_arg(&mut self, x: U1) -> anyhow::Result<()>;
        async fn u1_result(&mut self) -> anyhow::Result<U1>;
        async fn v1_arg(&mut self, x: V1) -> anyhow::Result<()>;
        async fn v1_result(&mut self) -> anyhow::Result<V1>;
        async fn bool_arg(&mut self, x: bool) -> anyhow::Result<()>;
        async fn bool_result(&mut self) -> anyhow::Result<bool>;
        async fn option_arg(
            &mut self,
            a: Option<bool>,
            b: Option<()>,
            c: Option<u32>,
            d: Option<E1>,
            e: Option<f32>,
            f: Option<U1>,
            g: Option<Option<bool>>,
        ) -> anyhow::Result<()>;
        async fn option_result(
            &mut self,
        ) -> anyhow::Result<(
            Option<bool>,
            Option<()>,
            Option<u32>,
            Option<E1>,
            Option<f32>,
            Option<U1>,
            Option<Option<bool>>,
        )>;
        async fn casts(
            &mut self,
            a: Casts1,
            b: Casts2,
            c: Casts3,
            d: Casts4,
            e: Casts5,
            f: Casts6,
        ) -> anyhow::Result<(Casts1, Casts2, Casts3, Casts4, Casts5, Casts6)>;
        async fn result_arg(
            &mut self,
            a: Result<(), ()>,
            b: Result<(), E1>,
            c: Result<E1, ()>,
            d: Result<(), ()>,
            e: Result<u32, V1>,
            f: Result<String, Vec<u8>>,
        ) -> anyhow::Result<()>;
        async fn result_result(
            &mut self,
        ) -> anyhow::Result<(
            Result<(), ()>,
            Result<(), E1>,
            Result<E1, ()>,
            Result<(), ()>,
            Result<u32, V1>,
            Result<String, Vec<u8>>,
        )>;
        async fn return_result_sugar(&mut self) -> anyhow::Result<Result<i32, MyErrno>>;
        async fn return_result_sugar2(&mut self) -> anyhow::Result<Result<(), MyErrno>>;
        async fn return_result_sugar3(&mut self) -> anyhow::Result<Result<MyErrno, MyErrno>>;
        async fn return_result_sugar4(&mut self) -> anyhow::Result<Result<(i32, u32), MyErrno>>;
        async fn return_option_sugar(&mut self) -> anyhow::Result<Option<i32>>;
        async fn return_option_sugar2(&mut self) -> anyhow::Result<Option<MyErrno>>;
        async fn result_simple(&mut self) -> anyhow::Result<Result<u32, i32>>;
        async fn is_clone_arg(&mut self, a: IsClone) -> anyhow::Result<()>;
        async fn is_clone_return(&mut self) -> anyhow::Result<IsClone>;
        async fn return_named_option(&mut self) -> anyhow::Result<Option<u8>>;
        async fn return_named_result(&mut self) -> anyhow::Result<Result<u8, MyErrno>>;
    }

    pub fn add_to_linker<T, U>(
        _linker: &mut (),
        _get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()>
    where
        T: Send,
        U: Imports + Send,
    {
        todo!()
    }
}
