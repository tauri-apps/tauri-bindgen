#[allow(clippy::all, unused)]
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
    pub struct AggregatesParam<'a> {
        pub a: Scalars,
        pub b: u32,
        pub c: Empty,
        pub d: &'a str,
        pub e: ReallyFlags,
    }
    #[derive(Debug, Clone)]
    pub struct AggregatesResult {
        pub a: Scalars,
        pub b: u32,
        pub c: Empty,
        pub d: String,
        pub e: ReallyFlags,
    }
    pub type IntTypedef = i32;
    pub type TupleTypedef2 = (IntTypedef,);
    pub fn tuple_arg(x: (char, u32)) -> () {
        todo!()
    }
    pub fn tuple_result() -> (char, u32) {
        todo!()
    }
    pub fn empty_arg(x: Empty) -> () {
        todo!()
    }
    pub fn empty_result() -> Empty {
        todo!()
    }
    pub fn scalar_arg(x: Scalars) -> () {
        todo!()
    }
    pub fn scalar_result() -> Scalars {
        todo!()
    }
    pub fn flags_arg(x: ReallyFlags) -> () {
        todo!()
    }
    pub fn flags_result() -> ReallyFlags {
        todo!()
    }
    pub fn aggregate_arg(x: AggregatesParam<'_>) -> () {
        todo!()
    }
    pub fn aggregate_result() -> AggregatesResult {
        todo!()
    }
    pub fn typedef_inout(e: TupleTypedef2) -> i32 {
        todo!()
    }
}
