#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod resources {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait A {
        fn f1(&self);
        fn f2(&self, a: u32);
        fn f3(&self, a: u32, b: u32);
    }
    pub trait B {
        type A: A;
        fn f1(&self) -> ::tauri_bindgen_host::ResourceId;
        fn f2(&self, x: ::tauri_bindgen_host::ResourceId) -> Result<u32, ()>;
        fn f3(
            &self,
            x: Option<Vec<::tauri_bindgen_host::ResourceId>>,
        ) -> Result<::tauri_bindgen_host::ResourceId, ()>;
    }
    pub trait Resources: Sized {
        type A: A;
        fn get_a_mut(&mut self, id: ::tauri_bindgen_host::ResourceId) -> &mut Self::A;
        type B: B;
        fn get_b_mut(&mut self, id: ::tauri_bindgen_host::ResourceId) -> &mut Self::B;
        fn constructor_a(&self) -> ::tauri_bindgen_host::ResourceId;
        fn constructor_b(&self) -> ::tauri_bindgen_host::ResourceId;
    }
    pub fn add_to_router<T, U, R>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T, R>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: Resources + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "resources",
                "constructor_a",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.constructor_a())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "resources",
                "constructor_b",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.constructor_b())
                },
            )?;
        Ok(())
    }
}
