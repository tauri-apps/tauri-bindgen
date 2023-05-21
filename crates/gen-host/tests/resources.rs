#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod resources {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait A {
        fn f1(&mut self);
        fn f2(&mut self, a: u32);
        fn f3(&mut self, a: u32, b: u32);
    }
    pub trait B {
        type A: A;
        fn f1(&mut self) -> ::tauri_bindgen_host::ResourceId;
        fn f2(&mut self, x: ::tauri_bindgen_host::ResourceId) -> Result<u32, ()>;
        fn f3(
            &mut self,
            x: Option<Vec<::tauri_bindgen_host::ResourceId>>,
        ) -> Result<::tauri_bindgen_host::ResourceId, ()>;
    }
    pub trait Resources: Sized {
        type A: A;
        fn get_a_mut(&mut self, id: ::tauri_bindgen_host::ResourceId) -> &mut Self::A;
        type B: B;
        fn get_b_mut(&mut self, id: ::tauri_bindgen_host::ResourceId) -> &mut Self::B;
        fn constructor_a(&mut self) -> ::tauri_bindgen_host::ResourceId;
        fn constructor_b(&mut self) -> ::tauri_bindgen_host::ResourceId;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Resources + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "resources",
                "constructor_a",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<
                    ::tauri_bindgen_host::ResourceId,
                > {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.constructor_a())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "resources",
                "constructor_b",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<
                    ::tauri_bindgen_host::ResourceId,
                > {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.constructor_b())
                },
            )?;
        Ok(())
    }
}
