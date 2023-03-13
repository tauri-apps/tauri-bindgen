#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod records {
    use ::tauri_bindgen_guest_rust::serde;
    use ::tauri_bindgen_guest_rust::bitflags;
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Empty {}
    /**A record containing two scalar fields
that both have the same type*/
    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Scalars {
        ///The first field, named a
        a: u32,
        ///The second field, named b
        b: u32,
    }
    /**A record that is really just flags
All of the fields are bool*/
    #[derive(serde::Serialize, serde::Deserialize)]
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
    #[derive(serde::Serialize)]
    pub struct AggregatesParam<'a> {
        a: Scalars,
        b: u32,
        c: Empty,
        d: &'a str,
        e: ReallyFlags,
    }
    #[derive(serde::Deserialize)]
    pub struct AggregatesResult {
        a: Scalars,
        b: u32,
        c: Empty,
        d: String,
        e: ReallyFlags,
    }
    pub type IntTypedef = i32;
    pub type TupleTypedef2 = (IntTypedef,);
    pub async fn tuple_arg(x: (char, u32)) {
        todo!()
    }
    pub async fn tuple_result() -> (char, u32) {
        todo!()
    }
    pub async fn empty_arg(x: Empty) {
        todo!()
    }
    pub async fn empty_result() -> Empty {
        todo!()
    }
    pub async fn scalar_arg(x: Scalars) {
        todo!()
    }
    pub async fn scalar_result() -> Scalars {
        todo!()
    }
    pub async fn flags_arg(x: ReallyFlags) {
        todo!()
    }
    pub async fn flags_result() -> ReallyFlags {
        todo!()
    }
    pub async fn aggregate_arg(x: AggregatesParam<'_>) {
        todo!()
    }
    pub async fn aggregate_result() -> AggregatesResult {
        todo!()
    }
    pub async fn typedef_inout(e: TupleTypedef2) -> i32 {
        todo!()
    }
}
