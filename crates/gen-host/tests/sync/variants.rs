#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod variants {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum E1 {
        A,
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct Empty {}
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum V1 {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts1 {
        A(i32),
        B(f32),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts2 {
        A(f64),
        B(f32),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts3 {
        A(f64),
        B(u64),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts4 {
        A(u32),
        B(i64),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts5 {
        A(f32),
        B(i64),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum Casts6 {
        A((f32, u32)),
        B((u32, u32)),
    }
    #[derive(serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub struct IsClone {
        pub v1: V1,
    }
    pub trait Variants: Sized {
        fn e1_arg(&self, x: E1);
        fn e1_result(&self) -> E1;
        fn u1_arg(&self, x: U1);
        fn u1_result(&self) -> U1;
        fn v1_arg(&self, x: V1);
        fn v1_result(&self) -> V1;
        fn bool_arg(&self, x: bool);
        fn bool_result(&self) -> bool;
        fn option_arg(
            &self,
            a: Option<bool>,
            b: Option<()>,
            c: Option<u32>,
            d: Option<E1>,
            e: Option<f32>,
            f: Option<U1>,
            g: Option<Option<bool>>,
        );
        fn option_result(
            &self,
        ) -> (
            Option<bool>,
            Option<()>,
            Option<u32>,
            Option<E1>,
            Option<f32>,
            Option<U1>,
            Option<Option<bool>>,
        );
        fn casts(
            &self,
            a: Casts1,
            b: Casts2,
            c: Casts3,
            d: Casts4,
            e: Casts5,
            f: Casts6,
        ) -> (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6);
        fn result_arg(
            &self,
            a: Result<(), ()>,
            b: Result<(), E1>,
            c: Result<E1, ()>,
            d: Result<(), ()>,
            e: Result<u32, V1>,
            f: Result<String, Vec<u8>>,
        );
        fn result_result(
            &self,
        ) -> (
            Result<(), ()>,
            Result<(), E1>,
            Result<E1, ()>,
            Result<(), ()>,
            Result<u32, V1>,
            Result<String, Vec<u8>>,
        );
        fn return_result_sugar(&self) -> Result<i32, MyErrno>;
        fn return_result_sugar2(&self) -> Result<(), MyErrno>;
        fn return_result_sugar3(&self) -> Result<MyErrno, MyErrno>;
        fn return_result_sugar4(&self) -> Result<(i32, u32), MyErrno>;
        fn return_option_sugar(&self) -> Option<i32>;
        fn return_option_sugar2(&self) -> Option<MyErrno>;
        fn result_simple(&self) -> Result<u32, i32>;
        fn is_clone_arg(&self, a: IsClone);
        fn is_clone_return(&self) -> IsClone;
        fn return_named_option(&self) -> Option<u8>;
        fn return_named_result(&self) -> Result<u8, MyErrno>;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: Variants + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "e1_arg",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: E1| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.e1_arg(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "e1_result",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.e1_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "u1_arg",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: U1| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.u1_arg(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "u1_result",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.u1_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "v1_arg",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: V1| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.v1_arg(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "v1_result",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.v1_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "bool_arg",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: bool| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.bool_arg(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "bool_result",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.bool_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "option_arg",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (
                        Option<bool>,
                        Option<()>,
                        Option<u32>,
                        Option<E1>,
                        Option<f32>,
                        Option<U1>,
                        Option<Option<bool>>,
                    )|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.option_arg(p.0, p.1, p.2, p.3, p.4, p.5, p.6))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "option_result",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.option_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "casts",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6)|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.casts(p.0, p.1, p.2, p.3, p.4, p.5))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "result_arg",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (
                        Result<(), ()>,
                        Result<(), E1>,
                        Result<E1, ()>,
                        Result<(), ()>,
                        Result<u32, V1>,
                        Result<String, Vec<u8>>,
                    )|
                {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.result_arg(p.0, p.1, p.2, p.3, p.4, p.5))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "result_result",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.result_result())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "return_result_sugar",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.return_result_sugar())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "return_result_sugar2",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.return_result_sugar2())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "return_result_sugar3",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.return_result_sugar3())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "return_result_sugar4",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.return_result_sugar4())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "return_option_sugar",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.return_option_sugar())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "return_option_sugar2",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.return_option_sugar2())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "result_simple",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.result_simple())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "is_clone_arg",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: IsClone| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.is_clone_arg(p))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "is_clone_return",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.is_clone_return())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "return_named_option",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.return_named_option())
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define(
                "variants",
                "return_named_result",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: ()| {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.return_named_result())
                },
            )?;
        Ok(())
    }
}
