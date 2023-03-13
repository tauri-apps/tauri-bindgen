#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod records {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct Empty {}
    /**A record containing two scalar fields
that both have the same type*/
    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct Scalars {
        ///The first field, named a
        a: u32,
        ///The second field, named b
        b: u32,
    }
    /**A record that is really just flags
All of the fields are bool*/
    #[derive(serde::Deserialize, serde::Serialize)]
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
    #[derive(serde::Deserialize, serde::Serialize)]
    pub struct Aggregates<'a> {
        a: Scalars,
        b: u32,
        c: Empty,
        d: &'a str,
        e: ReallyFlags,
    }
    pub type IntTypedef = i32;
    pub type TupleTypedef2 = (IntTypedef,);
    pub trait Records: Sized {
        fn tuple_arg(&mut self, x: (char, u32));
        fn tuple_result(&mut self) -> (char, u32);
        fn empty_arg(&mut self, x: Empty);
        fn empty_result(&mut self) -> Empty;
        fn scalar_arg(&mut self, x: Scalars);
        fn scalar_result(&mut self) -> Scalars;
        fn flags_arg(&mut self, x: ReallyFlags);
        fn flags_result(&mut self) -> ReallyFlags;
        fn aggregate_arg(&mut self, x: Aggregates);
        fn aggregate_result(&mut self) -> Aggregates<'_>;
        fn typedef_inout(&mut self, e: TupleTypedef2) -> i32;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Records,
    {
        Ok(())
    }
}
