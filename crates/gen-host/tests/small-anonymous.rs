#[allow(unused_imports, unused_variables)]
pub mod small_anonymous {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[derive(serde::Serialize)]
    pub enum Error {
        Success,
        Failure,
    }
    pub trait SmallAnonymous: Sized {
        fn option_test(&mut self) -> Result<Option<&'_ str>, Error>;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: SmallAnonymous,
    {
        Ok(())
    }
}
