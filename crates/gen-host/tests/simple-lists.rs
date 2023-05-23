#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod simple_lists {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub trait SimpleLists: Sized {
        fn simple_list1(&mut self, l: Vec<u32>);
        fn simple_list2(&mut self) -> Vec<u32>;
        fn simple_list3(&mut self, a: Vec<u32>, b: Vec<u32>) -> (Vec<u32>, Vec<u32>);
        fn simple_list4(&mut self, l: Vec<Vec<u32>>) -> Vec<Vec<u32>>;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: SimpleLists + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "simple_lists",
                "simple_list1",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    l: Vec<u32>,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data_mut());
                    ctx.simple_list1(l);
                    Ok(())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "simple_lists",
                "simple_list2",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<u32>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.simple_list2())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "simple_lists",
                "simple_list3",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: Vec<u32>,
                    b: Vec<u32>,
                | -> ::tauri_bindgen_host::anyhow::Result<(Vec<u32>, Vec<u32>)> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.simple_list3(a, b))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "simple_lists",
                "simple_list4",
                move |
                    mut ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    l: Vec<Vec<u32>>,
                | -> ::tauri_bindgen_host::anyhow::Result<Vec<Vec<u32>>> {
                    let ctx = get_cx(ctx.data_mut());
                    Ok(ctx.simple_list4(l))
                },
            )?;
        Ok(())
    }
}
