pub mod small_anonymous {
    use ::tauri_bindgen_host::bitflags;
    use ::tauri_bindgen_host::tauri_bindgen_abi;
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum Error {
        Success,
        Failure,
    }
    pub trait SmallAnonymous: Sized {
        fn option_test(&mut self) -> Result<Option<&'_ str>, Error<'_>>;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: SmallAnonymous,
    {
        router
            .func_wrap(
                "small_anonymous",
                "option_test",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> Result<Option<&'_ str>, Error<'_>> {
                    let cx = get_cx(cx.data_mut());
                    cx.option_test()
                },
            )?;
        Ok(())
    }
}
