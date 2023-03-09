#[allow(unused_imports, unused_variables)]
#[rustfmt::skip]
pub mod simple_lists {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait SimpleLists: Sized {
        fn simple_list1(&mut self, l: Vec<u32>) -> ();
        fn simple_list2(&mut self) -> &'_ [u32];
        fn simple_list3(&mut self, a: Vec<u32>, b: Vec<u32>) -> (&'_ [u32], &'_ [u32]);
        fn simple_list4(&mut self, l: Vec<Vec<u32>>) -> &'_ [&'_ [u32]];
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: SimpleLists,
    {
        Ok(())
    }
}
