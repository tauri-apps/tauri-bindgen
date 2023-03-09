#[allow(unused_imports, unused_variables)]
pub mod conventions {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[derive(serde::Deserialize)]
    pub struct LudicrousSpeed {
        how_fast_are_you_going: u32,
        i_am_going_extremely_slow: u64,
    }
    pub trait Conventions: Sized {
        fn kebab_case(&mut self) -> ();
        fn foo(&mut self, x: LudicrousSpeed) -> ();
        fn function_with_underscores(&mut self) -> ();
        fn function_with_no_weird_characters(&mut self) -> ();
        fn apple(&mut self) -> ();
        fn apple_pear(&mut self) -> ();
        fn apple_pear_grape(&mut self) -> ();
        fn a0(&mut self) -> ();
        fn is_xml(&mut self) -> ();
        fn explicit(&mut self) -> ();
        fn explicit_snake(&mut self) -> ();
        fn bool(&mut self) -> ();
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Conventions,
    {
        Ok(())
    }
}
