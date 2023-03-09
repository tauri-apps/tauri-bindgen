#[allow(unused_imports, unused_variables)]
#[rustfmt::skip]
pub mod integers {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait Integers: Sized {
        fn a1(&mut self, x: u8) -> ();
        fn a2(&mut self, x: i8) -> ();
        fn a3(&mut self, x: u16) -> ();
        fn a4(&mut self, x: i16) -> ();
        fn a5(&mut self, x: u32) -> ();
        fn a6(&mut self, x: i32) -> ();
        fn a7(&mut self, x: u64) -> ();
        fn a8(&mut self, x: i64) -> ();
        fn a9(
            &mut self,
            p1: u8,
            p2: i8,
            p3: u16,
            p4: i16,
            p5: u32,
            p6: i32,
            p7: u64,
            p8: i64,
        ) -> ();
        fn r1(&mut self) -> u8;
        fn r2(&mut self) -> i8;
        fn r3(&mut self) -> u16;
        fn r4(&mut self) -> i16;
        fn r5(&mut self) -> u32;
        fn r6(&mut self) -> i32;
        fn r7(&mut self) -> u64;
        fn r8(&mut self) -> i64;
        fn pair_ret(&mut self) -> (i64, u8);
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Integers,
    {
        Ok(())
    }
}
