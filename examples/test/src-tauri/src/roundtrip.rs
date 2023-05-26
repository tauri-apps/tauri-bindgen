pub use roundtrip::add_to_router;

tauri_bindgen_host::generate!({
  path: "../world.wit",
  async: false,
  tracing: true
});

pub struct Ctx;

impl roundtrip::Roundtrip for Ctx {
    fn empty(&self, x: roundtrip::Empty) -> roundtrip::Empty {
        x
    }

    fn record_scalars(&self, val: roundtrip::Scalars) -> roundtrip::Scalars {
        val
    }

    fn record_really_flags(&self, val: roundtrip::ReallyFlags) -> roundtrip::ReallyFlags {
        val
    }

    fn record_aggregates(&self, val: roundtrip::Aggregates) -> roundtrip::Aggregates {
        val
    }

    fn flag1(&self, x: roundtrip::Flag1) -> roundtrip::Flag1 {
        x
    }

    fn flag2(&self, x: roundtrip::Flag2) -> roundtrip::Flag2 {
        x
    }

    fn flag4(&self, x: roundtrip::Flag4) -> roundtrip::Flag4 {
        x
    }

    fn flag8(&self, x: roundtrip::Flag8) -> roundtrip::Flag8 {
        x
    }

    fn flag16(&self, x: roundtrip::Flag16) -> roundtrip::Flag16 {
        x
    }

    fn flag32(&self, x: roundtrip::Flag32) -> roundtrip::Flag32 {
        x
    }

    fn flag64(&self, x: roundtrip::Flag64) -> roundtrip::Flag64 {
        x
    }

    fn float32(&self, x: f32) -> f32 {
        x
    }

    fn float64(&self, x: f64) -> f64 {
        x
    }

    fn u8(&self, x: u8) -> u8 {
        x
    }

    fn s8(&self, x: i8) -> i8 {
        x
    }

    fn u16(&self, x: u16) -> u16 {
        x
    }

    fn s16(&self, x: i16) -> i16 {
        x
    }

    fn u32(&self, x: u32) -> u32 {
        x
    }

    fn s32(&self, x: i32) -> i32 {
        x
    }

    fn u64(&self, x: u64) -> u64 {
        x
    }

    fn s64(&self, x: i64) -> i64 {
        x
    }

    fn list_u8(&self, x: Vec<u8>) -> Vec<u8> {
        x
    }

    fn list_u16(&self, x: Vec<u16>) -> Vec<u16> {
        x
    }

    fn list_u32(&self, x: Vec<u32>) -> Vec<u32> {
        x
    }

    fn list_u64(&self, x: Vec<u64>) -> Vec<u64> {
        x
    }

    fn list_s8(&self, x: Vec<i8>) -> Vec<i8> {
        x
    }

    fn list_s16(&self, x: Vec<i16>) -> Vec<i16> {
        x
    }

    fn list_s32(&self, x: Vec<i32>) -> Vec<i32> {
        x
    }

    fn list_s64(&self, x: Vec<i64>) -> Vec<i64> {
        x
    }

    fn list_float32(&self, x: Vec<f32>) -> Vec<f32> {
        x
    }

    fn list_float64(&self, x: Vec<f64>) -> Vec<f64> {
        x
    }

    fn tuple_list(&self, x: Vec<(u8, i8)>) -> Vec<(u8, i8)> {
        x
    }

    fn string_list(&self, a: Vec<String>) -> Vec<String> {
        a
    }

    fn tuple_string_list(&self, x: Vec<(u8, String)>) -> Vec<(u8, String)> {
        x
    }

    fn record_list(&self, x: Vec<roundtrip::SomeRecord>) -> Vec<roundtrip::OtherRecord> {
        x.into_iter()
            .map(|r| roundtrip::OtherRecord {
                a1: r.c1,
                a2: r.c2,
                a3: r.c3,
                a4: r.c4,
                b: r.x,
                c: vec![],
            })
            .collect()
    }

    fn all_integers(&self, x: roundtrip::AllIntegers) -> roundtrip::AllIntegers {
        x
    }

    fn all_floats(&self, x: roundtrip::AllFloats) -> roundtrip::AllFloats {
        x
    }

    fn all_text(&self, x: roundtrip::AllText) -> roundtrip::AllText {
        x
    }

    fn e1(&self, x: roundtrip::E1) -> roundtrip::E1 {
        x
    }

    fn v1(&self, x: roundtrip::V1) -> roundtrip::V1 {
        x
    }

    fn options(
        &self,
        a: Option<bool>,
        b: Option<()>,
        c: Option<u32>,
        d: Option<roundtrip::E1>,
        e: Option<f32>,
        f: Option<roundtrip::U1>,
        g: Option<Option<bool>>,
    ) -> (
        Option<bool>,
        Option<()>,
        Option<u32>,
        Option<roundtrip::E1>,
        Option<f32>,
        Option<roundtrip::U1>,
        Option<Option<bool>>,
    ) {
        (a, b, c, d, e, f, g)
    }

    fn results(
        &self,
        a: Result<(), ()>,
        b: Result<(), roundtrip::E1>,
        c: Result<roundtrip::E1, ()>,
        d: Result<(), ()>,
        e: Result<u32, roundtrip::V1>,
        f: Result<String, Vec<u8>>,
    ) -> (
        Result<(), ()>,
        Result<(), roundtrip::E1>,
        Result<roundtrip::E1, ()>,
        Result<(), ()>,
        Result<u32, roundtrip::V1>,
        Result<String, Vec<u8>>,
    ) {
        (a, b, c, d, e, f)
    }
}
