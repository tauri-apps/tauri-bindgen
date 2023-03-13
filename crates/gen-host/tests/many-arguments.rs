#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod many_arguments {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[derive(serde::Deserialize)]
    pub struct BigStruct<'a> {
        a1: &'a str,
        a2: &'a str,
        a3: &'a str,
        a4: &'a str,
        a5: &'a str,
        a6: &'a str,
        a7: &'a str,
        a8: &'a str,
        a9: &'a str,
        a10: &'a str,
        a11: &'a str,
        a12: &'a str,
        a13: &'a str,
        a14: &'a str,
        a15: &'a str,
        a16: &'a str,
        a17: &'a str,
        a18: &'a str,
        a19: &'a str,
        a20: &'a str,
    }
    pub trait ManyArguments: Sized {
        fn many_args(
            &mut self,
            a1: u64,
            a2: u64,
            a3: u64,
            a4: u64,
            a5: u64,
            a6: u64,
            a7: u64,
            a8: u64,
            a9: u64,
            a10: u64,
            a11: u64,
            a12: u64,
            a13: u64,
            a14: u64,
            a15: u64,
            a16: u64,
        );
        fn big_argument(&mut self, x: BigStruct);
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: ManyArguments,
    {
        Ok(())
    }
}
