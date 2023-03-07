pub mod roundtrip {
    use ::tauri_bindgen_host::tauri_bindgen_abi;
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct AggregatesParam<'a> {
        a: Scalars,
        b: u32,
        c: Empty,
        d: &'a str,
        e: ReallyFlags,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub struct AggregatesResult {
        a: Scalars,
        b: u32,
        c: Empty,
        d: String,
        e: ReallyFlags,
    }
    #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
    pub enum AllFloats {
        F32(f32),
        F64(f64),
    }
    #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
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
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum AllTextParam<'a> {
        Char(char),
        String(&'a str),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum AllTextResult {
        Char(char),
        String(String),
    }
    #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
    pub enum DuplicateTypes {
        U80(u8),
        U81(u8),
        U82(u8),
    }
    #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
    pub struct Empty {}
    bitflags::bitflags! {
        pub struct Flag1 : u8 { const B0 = 1 << 0; }
    }
    bitflags::bitflags! {
        pub struct Flag16 : u16 { const B0 = 1 << 0; const B1 = 1 << 1; const B2 = 1 <<
        2; const B3 = 1 << 3; const B4 = 1 << 4; const B5 = 1 << 5; const B6 = 1 << 6;
        const B7 = 1 << 7; const B8 = 1 << 8; const B9 = 1 << 9; const B10 = 1 << 10;
        const B11 = 1 << 11; const B12 = 1 << 12; const B13 = 1 << 13; const B14 = 1 <<
        14; const B15 = 1 << 15; }
    }
    bitflags::bitflags! {
        pub struct Flag2 : u8 { const B0 = 1 << 0; const B1 = 1 << 1; }
    }
    bitflags::bitflags! {
        pub struct Flag32 : u32 { const B0 = 1 << 0; const B1 = 1 << 1; const B2 = 1 <<
        2; const B3 = 1 << 3; const B4 = 1 << 4; const B5 = 1 << 5; const B6 = 1 << 6;
        const B7 = 1 << 7; const B8 = 1 << 8; const B9 = 1 << 9; const B10 = 1 << 10;
        const B11 = 1 << 11; const B12 = 1 << 12; const B13 = 1 << 13; const B14 = 1 <<
        14; const B15 = 1 << 15; const B16 = 1 << 16; const B17 = 1 << 17; const B18 = 1
        << 18; const B19 = 1 << 19; const B20 = 1 << 20; const B21 = 1 << 21; const B22 =
        1 << 22; const B23 = 1 << 23; const B24 = 1 << 24; const B25 = 1 << 25; const B26
        = 1 << 26; const B27 = 1 << 27; const B28 = 1 << 28; const B29 = 1 << 29; const
        B30 = 1 << 30; const B31 = 1 << 31; }
    }
    bitflags::bitflags! {
        pub struct Flag4 : u8 { const B0 = 1 << 0; const B1 = 1 << 1; const B2 = 1 << 2;
        const B3 = 1 << 3; }
    }
    bitflags::bitflags! {
        pub struct Flag64 : u64 { const B0 = 1 << 0; const B1 = 1 << 1; const B2 = 1 <<
        2; const B3 = 1 << 3; const B4 = 1 << 4; const B5 = 1 << 5; const B6 = 1 << 6;
        const B7 = 1 << 7; const B8 = 1 << 8; const B9 = 1 << 9; const B10 = 1 << 10;
        const B11 = 1 << 11; const B12 = 1 << 12; const B13 = 1 << 13; const B14 = 1 <<
        14; const B15 = 1 << 15; const B16 = 1 << 16; const B17 = 1 << 17; const B18 = 1
        << 18; const B19 = 1 << 19; const B20 = 1 << 20; const B21 = 1 << 21; const B22 =
        1 << 22; const B23 = 1 << 23; const B24 = 1 << 24; const B25 = 1 << 25; const B26
        = 1 << 26; const B27 = 1 << 27; const B28 = 1 << 28; const B29 = 1 << 29; const
        B30 = 1 << 30; const B31 = 1 << 31; const B32 = 1 << 32; const B33 = 1 << 33;
        const B34 = 1 << 34; const B35 = 1 << 35; const B36 = 1 << 36; const B37 = 1 <<
        37; const B38 = 1 << 38; const B39 = 1 << 39; const B40 = 1 << 40; const B41 = 1
        << 41; const B42 = 1 << 42; const B43 = 1 << 43; const B44 = 1 << 44; const B45 =
        1 << 45; const B46 = 1 << 46; const B47 = 1 << 47; const B48 = 1 << 48; const B49
        = 1 << 49; const B50 = 1 << 50; const B51 = 1 << 51; const B52 = 1 << 52; const
        B53 = 1 << 53; const B54 = 1 << 54; const B55 = 1 << 55; const B56 = 1 << 56;
        const B57 = 1 << 57; const B58 = 1 << 58; const B59 = 1 << 59; const B60 = 1 <<
        60; const B61 = 1 << 61; const B62 = 1 << 62; const B63 = 1 << 63; }
    }
    bitflags::bitflags! {
        pub struct Flag8 : u8 { const B0 = 1 << 0; const B1 = 1 << 1; const B2 = 1 << 2;
        const B3 = 1 << 3; const B4 = 1 << 4; const B5 = 1 << 5; const B6 = 1 << 6; const
        B7 = 1 << 7; }
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct OtherRecordParam<'a> {
        a1: u32,
        a2: u64,
        a3: i32,
        a4: i64,
        b: &'a str,
        c: &'a [u8],
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub struct OtherRecordResult {
        a1: u32,
        a2: u64,
        a3: i32,
        a4: i64,
        b: String,
        c: Vec<u8>,
    }
    ///A record that is really just flagsAll of the fields are bool
    #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
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
    #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
    pub struct Scalars {
        ///The first field, named a
        a: u32,
        ///The second field, named b
        b: u32,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct SomeRecord<'a> {
        x: &'a str,
        y: OtherRecordParam<'a>,
        z: &'a [OtherRecordParam<'a>],
        c1: u32,
        c2: u64,
        c3: i32,
        c4: i64,
    }
    #[derive(tauri_bindgen_abi::Readable, tauri_bindgen_abi::Writable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    pub enum V1Param<'a> {
        A,
        B(U1),
        C(E1),
        D(&'a str),
        E(Empty),
        F,
        G(u32),
    }
    pub enum V1Result {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    pub trait Roundtrip: Sized {
        fn empty(&mut self, x: Empty) -> Empty;
        fn record_scalars(&mut self, val: Scalars) -> Scalars;
        fn record_really_flags(&mut self, val: ReallyFlags) -> ReallyFlags;
        fn record_aggregates(&mut self, val: AggregatesResult) -> AggregatesParam<'_>;
        fn flag1(&mut self, x: Flag1) -> Flag1;
        fn flag2(&mut self, x: Flag2) -> Flag2;
        fn flag4(&mut self, x: Flag4) -> Flag4;
        fn flag8(&mut self, x: Flag8) -> Flag8;
        fn flag16(&mut self, x: Flag16) -> Flag16;
        fn flag32(&mut self, x: Flag32) -> Flag32;
        fn flag64(&mut self, x: Flag64) -> Flag64;
        fn float32(&mut self, x: f32) -> f32;
        fn float64(&mut self, x: f64) -> f64;
        fn u8(&mut self, x: u8) -> u8;
        fn s8(&mut self, x: i8) -> i8;
        fn u16(&mut self, x: u16) -> u16;
        fn s16(&mut self, x: i16) -> i16;
        fn u32(&mut self, x: u32) -> u32;
        fn s32(&mut self, x: i32) -> i32;
        fn u64(&mut self, x: u64) -> u64;
        fn s64(&mut self, x: i64) -> i64;
        fn list_u8(&mut self, x: Vec<u8>) -> &'_ [u8];
        fn list_u16(&mut self, x: Vec<u16>) -> &'_ [u16];
        fn list_u32(&mut self, x: Vec<u32>) -> &'_ [u32];
        fn list_u64(&mut self, x: Vec<u64>) -> &'_ [u64];
        fn list_s8(&mut self, x: Vec<i8>) -> &'_ [i8];
        fn list_s16(&mut self, x: Vec<i16>) -> &'_ [i16];
        fn list_s32(&mut self, x: Vec<i32>) -> &'_ [i32];
        fn list_s64(&mut self, x: Vec<i64>) -> &'_ [i64];
        fn list_float32(&mut self, x: Vec<f32>) -> &'_ [f32];
        fn list_float64(&mut self, x: Vec<f64>) -> &'_ [f64];
        fn tuple_list(&mut self, x: Vec<(u8, i8)>) -> &'_ [(u8, i8)];
        fn string_list(&mut self, a: Vec<String>) -> &'_ [&'_ str];
        fn tuple_string_list(&mut self, x: Vec<(u8, String)>) -> &'_ [(u8, &'_ str)];
        fn record_list(&mut self, x: Vec<SomeRecord>) -> &'_ [OtherRecordParam<'_>];
        fn all_integers(&mut self, x: AllIntegers) -> AllIntegers;
        fn all_floats(&mut self, x: AllFloats) -> AllFloats;
        fn all_text(&mut self, x: AllTextResult) -> AllTextParam<'_>;
        fn e1(&mut self, x: E1) -> E1;
        fn v1(&mut self, x: V1Result) -> V1Param<'_>;
        fn options(
            &mut self,
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
        );
        fn results(
            &mut self,
            a: Result<(), ()>,
            b: Result<(), E1>,
            c: Result<E1, ()>,
            d: Result<(), ()>,
            e: Result<u32, V1Result>,
            f: Result<String, Vec<u8>>,
        ) -> (
            Result<(), ()>,
            Result<(), E1>,
            Result<E1, ()>,
            Result<(), ()>,
            Result<u32, V1Param<'_>>,
            Result<&'_ str, &'_ [u8]>,
        );
        fn duplicate_types(&mut self, a: DuplicateTypes) -> DuplicateTypes;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Roundtrip,
    {
        router
            .func_wrap(
                "roundtrip",
                "empty",
                |cx, x: Empty| -> Empty {
                    let cx: U = todo!();
                    cx.empty(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "record_scalars",
                |cx, val: Scalars| -> Scalars {
                    let cx: U = todo!();
                    cx.record_scalars(val)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "record_really_flags",
                |cx, val: ReallyFlags| -> ReallyFlags {
                    let cx: U = todo!();
                    cx.record_really_flags(val)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "record_aggregates",
                |cx, val: AggregatesResult| -> AggregatesParam<'_> {
                    let cx: U = todo!();
                    cx.record_aggregates(val)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "flag1",
                |cx, x: Flag1| -> Flag1 {
                    let cx: U = todo!();
                    cx.flag1(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "flag2",
                |cx, x: Flag2| -> Flag2 {
                    let cx: U = todo!();
                    cx.flag2(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "flag4",
                |cx, x: Flag4| -> Flag4 {
                    let cx: U = todo!();
                    cx.flag4(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "flag8",
                |cx, x: Flag8| -> Flag8 {
                    let cx: U = todo!();
                    cx.flag8(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "flag16",
                |cx, x: Flag16| -> Flag16 {
                    let cx: U = todo!();
                    cx.flag16(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "flag32",
                |cx, x: Flag32| -> Flag32 {
                    let cx: U = todo!();
                    cx.flag32(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "flag64",
                |cx, x: Flag64| -> Flag64 {
                    let cx: U = todo!();
                    cx.flag64(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "float32",
                |cx, x: f32| -> f32 {
                    let cx: U = todo!();
                    cx.float32(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "float64",
                |cx, x: f64| -> f64 {
                    let cx: U = todo!();
                    cx.float64(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "u8",
                |cx, x: u8| -> u8 {
                    let cx: U = todo!();
                    cx.u8(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "s8",
                |cx, x: i8| -> i8 {
                    let cx: U = todo!();
                    cx.s8(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "u16",
                |cx, x: u16| -> u16 {
                    let cx: U = todo!();
                    cx.u16(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "s16",
                |cx, x: i16| -> i16 {
                    let cx: U = todo!();
                    cx.s16(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "u32",
                |cx, x: u32| -> u32 {
                    let cx: U = todo!();
                    cx.u32(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "s32",
                |cx, x: i32| -> i32 {
                    let cx: U = todo!();
                    cx.s32(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "u64",
                |cx, x: u64| -> u64 {
                    let cx: U = todo!();
                    cx.u64(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "s64",
                |cx, x: i64| -> i64 {
                    let cx: U = todo!();
                    cx.s64(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "list_u8",
                |cx, x: Vec<u8>| -> &'_ [u8] {
                    let cx: U = todo!();
                    cx.list_u8(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "list_u16",
                |cx, x: Vec<u16>| -> &'_ [u16] {
                    let cx: U = todo!();
                    cx.list_u16(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "list_u32",
                |cx, x: Vec<u32>| -> &'_ [u32] {
                    let cx: U = todo!();
                    cx.list_u32(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "list_u64",
                |cx, x: Vec<u64>| -> &'_ [u64] {
                    let cx: U = todo!();
                    cx.list_u64(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "list_s8",
                |cx, x: Vec<i8>| -> &'_ [i8] {
                    let cx: U = todo!();
                    cx.list_s8(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "list_s16",
                |cx, x: Vec<i16>| -> &'_ [i16] {
                    let cx: U = todo!();
                    cx.list_s16(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "list_s32",
                |cx, x: Vec<i32>| -> &'_ [i32] {
                    let cx: U = todo!();
                    cx.list_s32(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "list_s64",
                |cx, x: Vec<i64>| -> &'_ [i64] {
                    let cx: U = todo!();
                    cx.list_s64(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "list_float32",
                |cx, x: Vec<f32>| -> &'_ [f32] {
                    let cx: U = todo!();
                    cx.list_float32(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "list_float64",
                |cx, x: Vec<f64>| -> &'_ [f64] {
                    let cx: U = todo!();
                    cx.list_float64(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "tuple_list",
                |cx, x: Vec<(u8, i8)>| -> &'_ [(u8, i8)] {
                    let cx: U = todo!();
                    cx.tuple_list(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "string_list",
                |cx, a: Vec<String>| -> &'_ [&'_ str] {
                    let cx: U = todo!();
                    cx.string_list(a)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "tuple_string_list",
                |cx, x: Vec<(u8, String)>| -> &'_ [(u8, &'_ str)] {
                    let cx: U = todo!();
                    cx.tuple_string_list(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "record_list",
                |cx, x: Vec<SomeRecord>| -> &'_ [OtherRecordParam<'_>] {
                    let cx: U = todo!();
                    cx.record_list(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "all_integers",
                |cx, x: AllIntegers| -> AllIntegers {
                    let cx: U = todo!();
                    cx.all_integers(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "all_floats",
                |cx, x: AllFloats| -> AllFloats {
                    let cx: U = todo!();
                    cx.all_floats(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "all_text",
                |cx, x: AllTextResult| -> AllTextParam<'_> {
                    let cx: U = todo!();
                    cx.all_text(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "e1",
                |cx, x: E1| -> E1 {
                    let cx: U = todo!();
                    cx.e1(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "v1",
                |cx, x: V1Result| -> V1Param<'_> {
                    let cx: U = todo!();
                    cx.v1(x)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "options",
                |
                    cx,
                    a: Option<bool>,
                    b: Option<()>,
                    c: Option<u32>,
                    d: Option<E1>,
                    e: Option<f32>,
                    f: Option<U1>,
                    g: Option<Option<bool>>,
                | -> (
                    Option<bool>,
                    Option<()>,
                    Option<u32>,
                    Option<E1>,
                    Option<f32>,
                    Option<U1>,
                    Option<Option<bool>>,
                ) {
                    let cx: U = todo!();
                    cx.options(a, b, c, d, e, f, g)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "results",
                |
                    cx,
                    a: Result<(), ()>,
                    b: Result<(), E1>,
                    c: Result<E1, ()>,
                    d: Result<(), ()>,
                    e: Result<u32, V1Result>,
                    f: Result<String, Vec<u8>>,
                | -> (
                    Result<(), ()>,
                    Result<(), E1>,
                    Result<E1, ()>,
                    Result<(), ()>,
                    Result<u32, V1Param<'_>>,
                    Result<&'_ str, &'_ [u8]>,
                ) {
                    let cx: U = todo!();
                    cx.results(a, b, c, d, e, f)
                },
            )?;
        router
            .func_wrap(
                "roundtrip",
                "duplicate_types",
                |cx, a: DuplicateTypes| -> DuplicateTypes {
                    let cx: U = todo!();
                    cx.duplicate_types(a)
                },
            )?;
        Ok(())
    }
}