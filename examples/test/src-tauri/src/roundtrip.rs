use tauri::{
    plugin::{self, TauriPlugin},
    Runtime,
};

tauri_bindgen_host::generate!({
  path: "../world.wit",
  async: false,
  tracing: true
});

struct Ctx;

impl roundtrip::Roundtrip for Ctx {
    fn record_scalars(
        &self,
        val: roundtrip::Scalars,
    ) -> tauri_bindgen_host::anyhow::Result<roundtrip::Scalars> {
        Ok(val)
    }

    fn record_really_flags(
        &self,
        val: roundtrip::ReallyFlags,
    ) -> tauri_bindgen_host::anyhow::Result<roundtrip::ReallyFlags> {
        Ok(val)
    }

    fn record_aggregates(
        &self,
        val: roundtrip::Aggregates,
    ) -> tauri_bindgen_host::anyhow::Result<roundtrip::Aggregates> {
        Ok(val)
    }

    fn empty(&self, x: roundtrip::Empty) -> tauri_bindgen_host::anyhow::Result<roundtrip::Empty> {
        Ok(x)
    }

    fn flag1(&self, x: roundtrip::Flag1) -> tauri_bindgen_host::anyhow::Result<roundtrip::Flag1> {
        Ok(x)
    }

    fn flag2(&self, x: roundtrip::Flag2) -> tauri_bindgen_host::anyhow::Result<roundtrip::Flag2> {
        Ok(x)
    }

    fn flag4(&self, x: roundtrip::Flag4) -> tauri_bindgen_host::anyhow::Result<roundtrip::Flag4> {
        Ok(x)
    }

    fn flag8(&self, x: roundtrip::Flag8) -> tauri_bindgen_host::anyhow::Result<roundtrip::Flag8> {
        Ok(x)
    }

    fn flag16(
        &self,
        x: roundtrip::Flag16,
    ) -> tauri_bindgen_host::anyhow::Result<roundtrip::Flag16> {
        Ok(x)
    }

    fn flag32(
        &self,
        x: roundtrip::Flag32,
    ) -> tauri_bindgen_host::anyhow::Result<roundtrip::Flag32> {
        Ok(x)
    }

    fn flag64(
        &self,
        x: roundtrip::Flag64,
    ) -> tauri_bindgen_host::anyhow::Result<roundtrip::Flag64> {
        Ok(x)
    }

    fn float32(&self, x: f32) -> tauri_bindgen_host::anyhow::Result<f32> {
        Ok(x)
    }

    fn float64(&self, x: f64) -> tauri_bindgen_host::anyhow::Result<f64> {
        Ok(x)
    }

    fn u8(&self, x: u8) -> tauri_bindgen_host::anyhow::Result<u8> {
        Ok(x)
    }

    fn s8(&self, x: i8) -> tauri_bindgen_host::anyhow::Result<i8> {
        Ok(x)
    }

    fn u16(&self, x: u16) -> tauri_bindgen_host::anyhow::Result<u16> {
        Ok(x)
    }

    fn s16(&self, x: i16) -> tauri_bindgen_host::anyhow::Result<i16> {
        Ok(x)
    }

    fn u32(&self, x: u32) -> tauri_bindgen_host::anyhow::Result<u32> {
        Ok(x)
    }

    fn s32(&self, x: i32) -> tauri_bindgen_host::anyhow::Result<i32> {
        Ok(x)
    }

    fn u64(&self, x: u64) -> tauri_bindgen_host::anyhow::Result<u64> {
        Ok(x)
    }

    fn s64(&self, x: i64) -> tauri_bindgen_host::anyhow::Result<i64> {
        Ok(x)
    }

    fn list_u8(&self, x: Vec<u8>) -> tauri_bindgen_host::anyhow::Result<Vec<u8>> {
        Ok(x)
    }

    fn list_u16(&self, x: Vec<u16>) -> tauri_bindgen_host::anyhow::Result<Vec<u16>> {
        Ok(x)
    }

    fn list_u32(&self, x: Vec<u32>) -> tauri_bindgen_host::anyhow::Result<Vec<u32>> {
        Ok(x)
    }

    fn list_u64(&self, x: Vec<u64>) -> tauri_bindgen_host::anyhow::Result<Vec<u64>> {
        Ok(x)
    }

    fn list_s8(&self, x: Vec<i8>) -> tauri_bindgen_host::anyhow::Result<Vec<i8>> {
        Ok(x)
    }

    fn list_s16(&self, x: Vec<i16>) -> tauri_bindgen_host::anyhow::Result<Vec<i16>> {
        Ok(x)
    }

    fn list_s32(&self, x: Vec<i32>) -> tauri_bindgen_host::anyhow::Result<Vec<i32>> {
        Ok(x)
    }

    fn list_s64(&self, x: Vec<i64>) -> tauri_bindgen_host::anyhow::Result<Vec<i64>> {
        Ok(x)
    }

    fn list_float32(&self, x: Vec<f32>) -> tauri_bindgen_host::anyhow::Result<Vec<f32>> {
        Ok(x)
    }

    fn list_float64(&self, x: Vec<f64>) -> tauri_bindgen_host::anyhow::Result<Vec<f64>> {
        Ok(x)
    }

    fn tuple_list(&self, x: Vec<(u8, i8)>) -> tauri_bindgen_host::anyhow::Result<Vec<(u8, i8)>> {
        Ok(x)
    }

    fn string_list(&self, a: Vec<String>) -> tauri_bindgen_host::anyhow::Result<Vec<String>> {
        Ok(a)
    }

    fn tuple_string_list(
        &self,
        x: Vec<(u8, String)>,
    ) -> tauri_bindgen_host::anyhow::Result<Vec<(u8, String)>> {
        Ok(x)
    }

    fn record_list(
        &self,
        x: Vec<roundtrip::SomeRecord>,
    ) -> tauri_bindgen_host::anyhow::Result<Vec<roundtrip::OtherRecord>> {
        Ok(x.into_iter().map(|x| x.y).collect())
    }

    fn all_integers(
        &self,
        x: roundtrip::AllIntegers,
    ) -> tauri_bindgen_host::anyhow::Result<roundtrip::AllIntegers> {
        Ok(x)
    }

    fn all_floats(
        &self,
        x: roundtrip::AllFloats,
    ) -> tauri_bindgen_host::anyhow::Result<roundtrip::AllFloats> {
        Ok(x)
    }

    fn all_text(
        &self,
        x: roundtrip::AllText,
    ) -> tauri_bindgen_host::anyhow::Result<roundtrip::AllText> {
        Ok(x)
    }

    fn e1(&self, x: roundtrip::E1) -> tauri_bindgen_host::anyhow::Result<roundtrip::E1> {
        Ok(x)
    }

    fn v1(&self, x: roundtrip::V1) -> tauri_bindgen_host::anyhow::Result<roundtrip::V1> {
        Ok(x)
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
    ) -> tauri_bindgen_host::anyhow::Result<(
        Option<bool>,
        Option<()>,
        Option<u32>,
        Option<roundtrip::E1>,
        Option<f32>,
        Option<roundtrip::U1>,
        Option<Option<bool>>,
    )> {
        Ok((a, b, c, d, e, f, g))
    }

    fn results(
        &self,
        a: Result<(), ()>,
        b: Result<(), roundtrip::E1>,
        c: Result<roundtrip::E1, ()>,
        d: Result<(), ()>,
        e: Result<u32, roundtrip::V1>,
        f: Result<String, Vec<u8>>,
    ) -> tauri_bindgen_host::anyhow::Result<(
        Result<(), ()>,
        Result<(), roundtrip::E1>,
        Result<roundtrip::E1, ()>,
        Result<(), ()>,
        Result<u32, roundtrip::V1>,
        Result<String, Vec<u8>>,
    )> {
        Ok((a, b, c, d, e, f))
    }
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    plugin::Builder::new(roundtrip::WORLD_HASH)
        .invoke_handler(roundtrip::invoke_handler(Ctx))
        .build()
}
