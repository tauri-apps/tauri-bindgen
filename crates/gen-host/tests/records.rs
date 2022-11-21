#[allow(clippy::all)]
pub mod imports {
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Empty {}
    /// A record containing two scalar fields
    /// that both have the same type
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct Scalars {
        /// The first field, named a
        pub a: u32,
        /// The second field, named b
        pub b: u32,
    }
    /// A record that is really just flags
    /// All of the fields are bool
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
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
    #[derive(Debug, Clone)]
    pub struct Aggregates {
        pub a: Scalars,
        pub b: u32,
        pub c: Empty,
        pub d: String,
        pub e: ReallyFlags,
    }
    pub type IntTypedef = i32;
    pub type TupleTypedef2 = (IntTypedef,);
    #[host::async_trait]
    pub trait Imports: Sized {
        async fn tuple_arg(&mut self, x: (char, u32)) -> anyhow::Result<()>;
        async fn tuple_result(&mut self) -> anyhow::Result<(char, u32)>;
        async fn empty_arg(&mut self, x: Empty) -> anyhow::Result<()>;
        async fn empty_result(&mut self) -> anyhow::Result<Empty>;
        async fn scalar_arg(&mut self, x: Scalars) -> anyhow::Result<()>;
        async fn scalar_result(&mut self) -> anyhow::Result<Scalars>;
        async fn flags_arg(&mut self, x: ReallyFlags) -> anyhow::Result<()>;
        async fn flags_result(&mut self) -> anyhow::Result<ReallyFlags>;
        async fn aggregate_arg(&mut self, x: Aggregates) -> anyhow::Result<()>;
        async fn aggregate_result(&mut self) -> anyhow::Result<Aggregates>;
        async fn typedef_inout(&mut self, e: TupleTypedef2) -> anyhow::Result<i32>;
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
