mod roundtrip {
    pub struct Aggregates {
        a: Scalars,
        b: u32,
        c: Empty,
        d: &str,
        e: ReallyFlags,
    }
    pub enum AllFloats {
        F32(f32),
        F64(f64),
    }
    pub enum AllIntegers {
        Bool(bool),
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        I8(i8),
        I16(i16),
        S32(i32),
        S64(i64),
    }
    pub enum AllText {
        Char(char),
        String(&str),
    }
    pub enum DuplicateTypes {
        U80(u8),
        U81(u8),
        U82(u8),
    }
    pub enum E1 {
        a,
    }
    pub struct Empty {}
    bitflags::bitflags! {
        pub struct Flag1 : u8 { pub const B0 = 1 << 0; }
    }
    bitflags::bitflags! {
        pub struct Flag16 : u16 { pub const B0 = 1 << 0; pub const B1 = 1 << 1; pub const
        B2 = 1 << 2; pub const B3 = 1 << 3; pub const B4 = 1 << 4; pub const B5 = 1 << 5;
        pub const B6 = 1 << 6; pub const B7 = 1 << 7; pub const B8 = 1 << 8; pub const B9
        = 1 << 9; pub const B10 = 1 << 10; pub const B11 = 1 << 11; pub const B12 = 1 <<
        12; pub const B13 = 1 << 13; pub const B14 = 1 << 14; pub const B15 = 1 << 15; }
    }
    bitflags::bitflags! {
        pub struct Flag2 : u8 { pub const B0 = 1 << 0; pub const B1 = 1 << 1; }
    }
    bitflags::bitflags! {
        pub struct Flag32 : u32 { pub const B0 = 1 << 0; pub const B1 = 1 << 1; pub const
        B2 = 1 << 2; pub const B3 = 1 << 3; pub const B4 = 1 << 4; pub const B5 = 1 << 5;
        pub const B6 = 1 << 6; pub const B7 = 1 << 7; pub const B8 = 1 << 8; pub const B9
        = 1 << 9; pub const B10 = 1 << 10; pub const B11 = 1 << 11; pub const B12 = 1 <<
        12; pub const B13 = 1 << 13; pub const B14 = 1 << 14; pub const B15 = 1 << 15;
        pub const B16 = 1 << 16; pub const B17 = 1 << 17; pub const B18 = 1 << 18; pub
        const B19 = 1 << 19; pub const B20 = 1 << 20; pub const B21 = 1 << 21; pub const
        B22 = 1 << 22; pub const B23 = 1 << 23; pub const B24 = 1 << 24; pub const B25 =
        1 << 25; pub const B26 = 1 << 26; pub const B27 = 1 << 27; pub const B28 = 1 <<
        28; pub const B29 = 1 << 29; pub const B30 = 1 << 30; pub const B31 = 1 << 31; }
    }
    bitflags::bitflags! {
        pub struct Flag4 : u8 { pub const B0 = 1 << 0; pub const B1 = 1 << 1; pub const
        B2 = 1 << 2; pub const B3 = 1 << 3; }
    }
    bitflags::bitflags! {
        pub struct Flag64 : u64 { pub const B0 = 1 << 0; pub const B1 = 1 << 1; pub const
        B2 = 1 << 2; pub const B3 = 1 << 3; pub const B4 = 1 << 4; pub const B5 = 1 << 5;
        pub const B6 = 1 << 6; pub const B7 = 1 << 7; pub const B8 = 1 << 8; pub const B9
        = 1 << 9; pub const B10 = 1 << 10; pub const B11 = 1 << 11; pub const B12 = 1 <<
        12; pub const B13 = 1 << 13; pub const B14 = 1 << 14; pub const B15 = 1 << 15;
        pub const B16 = 1 << 16; pub const B17 = 1 << 17; pub const B18 = 1 << 18; pub
        const B19 = 1 << 19; pub const B20 = 1 << 20; pub const B21 = 1 << 21; pub const
        B22 = 1 << 22; pub const B23 = 1 << 23; pub const B24 = 1 << 24; pub const B25 =
        1 << 25; pub const B26 = 1 << 26; pub const B27 = 1 << 27; pub const B28 = 1 <<
        28; pub const B29 = 1 << 29; pub const B30 = 1 << 30; pub const B31 = 1 << 31;
        pub const B32 = 1 << 32; pub const B33 = 1 << 33; pub const B34 = 1 << 34; pub
        const B35 = 1 << 35; pub const B36 = 1 << 36; pub const B37 = 1 << 37; pub const
        B38 = 1 << 38; pub const B39 = 1 << 39; pub const B40 = 1 << 40; pub const B41 =
        1 << 41; pub const B42 = 1 << 42; pub const B43 = 1 << 43; pub const B44 = 1 <<
        44; pub const B45 = 1 << 45; pub const B46 = 1 << 46; pub const B47 = 1 << 47;
        pub const B48 = 1 << 48; pub const B49 = 1 << 49; pub const B50 = 1 << 50; pub
        const B51 = 1 << 51; pub const B52 = 1 << 52; pub const B53 = 1 << 53; pub const
        B54 = 1 << 54; pub const B55 = 1 << 55; pub const B56 = 1 << 56; pub const B57 =
        1 << 57; pub const B58 = 1 << 58; pub const B59 = 1 << 59; pub const B60 = 1 <<
        60; pub const B61 = 1 << 61; pub const B62 = 1 << 62; pub const B63 = 1 << 63; }
    }
    bitflags::bitflags! {
        pub struct Flag8 : u8 { pub const B0 = 1 << 0; pub const B1 = 1 << 1; pub const
        B2 = 1 << 2; pub const B3 = 1 << 3; pub const B4 = 1 << 4; pub const B5 = 1 << 5;
        pub const B6 = 1 << 6; pub const B7 = 1 << 7; }
    }
    ///A record that is really just flagsAll of the fields are bool
    pub struct ReallyFlags {
        a: bool,
        b: bool,
        c: bool,
        d: bool,
        e: bool,
        f: bool,
        g: bool,
        h: bool,
        i: bool,
    }
    ///A record containing two scalar fieldsthat both have the same type
    pub struct Scalars {
        ///The first field, named a
        a: u32,
        ///The second field, named b
        b: u32,
    }
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    pub enum V1 {
        a,
        b(U1),
        c(E1),
        d(&str),
        e(Empty),
        f,
        g(u32),
    }
    pub fn empty(x: Empty) -> Empty {}
    pub fn record_scalars(val: Scalars) -> Scalars {}
    pub fn record_really_flags(val: ReallyFlags) -> ReallyFlags {}
    pub fn record_aggregates(val: Aggregates) -> Aggregates {}
    pub fn flag1(x: Flag1) -> Flag1 {}
    pub fn flag2(x: Flag2) -> Flag2 {}
    pub fn flag4(x: Flag4) -> Flag4 {}
    pub fn flag8(x: Flag8) -> Flag8 {}
    pub fn flag16(x: Flag16) -> Flag16 {}
    pub fn flag32(x: Flag32) -> Flag32 {}
    pub fn flag64(x: Flag64) -> Flag64 {}
    pub fn float32(x: f32) -> f32 {}
    pub fn float64(x: f64) -> f64 {}
    pub fn u8(x: u8) -> u8 {}
    pub fn s8(x: i8) -> i8 {}
    pub fn u16(x: u16) -> u16 {}
    pub fn s16(x: i16) -> i16 {}
    pub fn u32(x: u32) -> u32 {}
    pub fn s32(x: i32) -> i32 {}
    pub fn u64(x: u64) -> u64 {}
    pub fn s64(x: i64) -> i64 {}
    pub fn list_u8(x: &[u8]) -> &[u8] {}
    pub fn list_u16(x: &[u16]) -> &[u16] {}
    pub fn list_u32(x: &[u32]) -> &[u32] {}
    pub fn list_u64(x: &[u64]) -> &[u64] {}
    pub fn list_s8(x: &[i8]) -> &[i8] {}
    pub fn list_s16(x: &[i16]) -> &[i16] {}
    pub fn list_s32(x: &[i32]) -> &[i32] {}
    pub fn list_s64(x: &[i64]) -> &[i64] {}
    pub fn list_float32(x: &[f32]) -> &[f32] {}
    pub fn list_float64(x: &[f64]) -> &[f64] {}
    pub fn tuple_list(x: &[(u8, i8)]) -> &[(u8, i8)] {}
    pub fn string_list(a: &[&str]) -> &[&str] {}
    pub fn tuple_string_list(x: &[(u8, &str)]) -> &[(u8, &str)] {}
    pub fn record_list(x: &[SomeRecord]) -> &[OtherRecord] {}
    pub fn all_integers(x: AllIntegers) -> AllIntegers {}
    pub fn all_floats(x: AllFloats) -> AllFloats {}
    pub fn all_text(x: AllText) -> AllText {}
    pub fn e1(x: E1) -> E1 {}
    pub fn v1(x: V1) -> V1 {}
    pub fn options(
        a: Option<bool>,
        b: Option<()>,
        c: Option<u32>,
        d: Option<E1>,
        e: Option<f32>,
        f: Option<U1>,
        g: Option<Option<bool>>,
    ) -> (
        Option<bool>,
        Option<()>,
        Option<u32>,
        Option<E1>,
        Option<f32>,
        Option<U1>,
        Option<Option<bool>>,
    ) {}
    pub fn results(
        a: Result<(), ()>,
        b: Result<(), E1>,
        c: Result<E1, ()>,
        d: Result<(), ()>,
        e: Result<u32, V1>,
        f: Result<&str, &[u8]>,
    ) -> (
        Result<(), ()>,
        Result<(), E1>,
        Result<E1, ()>,
        Result<(), ()>,
        Result<u32, V1>,
        Result<&str, &[u8]>,
    ) {}
    pub fn duplicate_types(a: DuplicateTypes) -> DuplicateTypes {}
}