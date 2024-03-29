#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod records {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct Empty {}
    /**A record containing two scalar fields
that both have the same type*/
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct Scalars {
        ///The first field, named a
        pub a: u32,
        ///The second field, named b
        pub b: u32,
    }
    /**A record that is really just flags
All of the fields are bool*/
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct ReallyFlags {
        pub a: bool,
        pub b: bool,
        pub c: bool,
        pub d: bool,
        pub e: bool,
        pub f: bool,
        pub g: bool,
        pub h: bool,
        pub i: bool,
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct Aggregates {
        pub a: Scalars,
        pub b: u32,
        pub c: Empty,
        pub d: String,
        pub e: ReallyFlags,
    }
    pub type IntTypedef = i32;
    pub type TupleTypedef2 = (IntTypedef,);
    pub trait Records: Sized {
        fn tuple_arg(&self, x: (char, u32));
        fn tuple_result(&self) -> (char, u32);
        fn empty_arg(&self, x: Empty);
        fn empty_result(&self) -> Empty;
        fn scalar_arg(&self, x: Scalars);
        fn scalar_result(&self) -> Scalars;
        fn flags_arg(&self, x: ReallyFlags);
        fn flags_result(&self) -> ReallyFlags;
        fn aggregate_arg(&self, x: Aggregates);
        fn aggregate_result(&self) -> Aggregates;
        fn typedef_inout(&self, e: TupleTypedef2) -> i32;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: Records + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "records",
                "tuple_arg",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (char, u32)|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.tuple_arg(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "records",
                "tuple_result",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.tuple_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "records",
                "empty_arg",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Empty| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.empty_arg(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "records",
                "empty_result",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.empty_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "records",
                "scalar_arg",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Scalars| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.scalar_arg(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "records",
                "scalar_result",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.scalar_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "records",
                "flags_arg",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: ReallyFlags|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.flags_arg(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "records",
                "flags_result",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.flags_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "records",
                "aggregate_arg",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: Aggregates|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.aggregate_arg(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "records",
                "aggregate_result",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.aggregate_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "records",
                "typedef_inout",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: TupleTypedef2|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.typedef_inout(p))
                },
            )?;
        Ok(())
    }
}
