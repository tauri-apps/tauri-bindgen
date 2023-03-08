pub mod variants {
    use ::tauri_bindgen_host::tauri_bindgen_abi;
    use ::tauri_bindgen_host::bitflags;
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct Empty {}
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum V1 {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub struct Empty {}
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum V1 {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum Casts1 {
        A(i32),
        B(f32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum Casts2 {
        A(f64),
        B(f32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum Casts3 {
        A(f64),
        B(u64),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum Casts4 {
        A(u32),
        B(i64),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum Casts5 {
        A(f32),
        B(i64),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum Casts6 {
        A((f32, u32)),
        B((u32, u32)),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum Casts1 {
        A(i32),
        B(f32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum Casts2 {
        A(f64),
        B(f32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum Casts3 {
        A(f64),
        B(u64),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum Casts4 {
        A(u32),
        B(i64),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum Casts5 {
        A(f32),
        B(i64),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum Casts6 {
        A((f32, u32)),
        B((u32, u32)),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct Empty {}
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum V1 {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub struct Empty {}
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum V1 {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct Empty {}
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum V1 {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub struct IsClone {
        v1: V1,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum U1 {
        U32(u32),
        F32(f32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum E1 {
        A,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub struct Empty {}
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum V1 {
        A,
        B(U1),
        C(E1),
        D(String),
        E(Empty),
        F,
        G(u32),
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub struct IsClone {
        v1: V1,
    }
    #[derive(tauri_bindgen_abi::Writable)]
    pub enum MyErrno {
        Bad1,
        Bad2,
    }
    pub trait Variants: Sized {
        fn e1_arg(&mut self, x: E1) -> ();
        fn e1_result(&mut self) -> E1;
        fn u1_arg(&mut self, x: U1) -> ();
        fn u1_result(&mut self) -> U1;
        fn v1_arg(&mut self, x: V1) -> ();
        fn v1_result(&mut self) -> V1<'_>;
        fn bool_arg(&mut self, x: bool) -> ();
        fn bool_result(&mut self) -> bool;
        fn option_arg(
            &mut self,
            a: Option<bool>,
            b: Option<()>,
            c: Option<u32>,
            d: Option<E1>,
            e: Option<f32>,
            f: Option<U1>,
            g: Option<Option<bool>>,
        ) -> ();
        fn option_result(
            &mut self,
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
            &mut self,
            a: Casts1,
            b: Casts2,
            c: Casts3,
            d: Casts4,
            e: Casts5,
            f: Casts6,
        ) -> (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6);
        fn result_arg(
            &mut self,
            a: Result<(), ()>,
            b: Result<(), E1>,
            c: Result<E1, ()>,
            d: Result<(), ()>,
            e: Result<u32, V1>,
            f: Result<String, Vec<u8>>,
        ) -> ();
        fn result_result(
            &mut self,
        ) -> (
            Result<(), ()>,
            Result<(), E1>,
            Result<E1, ()>,
            Result<(), ()>,
            Result<u32, V1<'_>>,
            Result<&'_ str, &'_ [u8]>,
        );
        fn return_result_sugar(&mut self) -> Result<i32, MyErrno>;
        fn return_result_sugar2(&mut self) -> Result<(), MyErrno>;
        fn return_result_sugar3(&mut self) -> Result<MyErrno, MyErrno>;
        fn return_result_sugar4(&mut self) -> Result<(i32, u32), MyErrno>;
        fn return_option_sugar(&mut self) -> Option<i32>;
        fn return_option_sugar2(&mut self) -> Option<MyErrno>;
        fn result_simple(&mut self) -> Result<u32, i32>;
        fn is_clone_arg(&mut self, a: IsClone) -> ();
        fn is_clone_return(&mut self) -> IsClone<'_>;
        fn return_named_option(&mut self) -> Option<u8>;
        fn return_named_result(&mut self) -> Result<u8, MyErrno>;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Variants,
    {
        router
            .func_wrap(
                "variants",
                "e1_arg",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: E1| -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.e1_arg(x)
                },
            )?;
        router
            .func_wrap(
                "variants",
                "e1_result",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> E1 {
                    let cx = get_cx(cx.data_mut());
                    cx.e1_result()
                },
            )?;
        router
            .func_wrap(
                "variants",
                "u1_arg",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: U1| -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.u1_arg(x)
                },
            )?;
        router
            .func_wrap(
                "variants",
                "u1_result",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> U1 {
                    let cx = get_cx(cx.data_mut());
                    cx.u1_result()
                },
            )?;
        router
            .func_wrap(
                "variants",
                "v1_arg",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, x: V1| -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.v1_arg(x)
                },
            )?;
        router
            .func_wrap(
                "variants",
                "v1_result",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> V1<'_> {
                    let cx = get_cx(cx.data_mut());
                    cx.v1_result()
                },
            )?;
        router
            .func_wrap(
                "variants",
                "bool_arg",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: bool,
                | -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.bool_arg(x)
                },
            )?;
        router
            .func_wrap(
                "variants",
                "bool_result",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> bool {
                    let cx = get_cx(cx.data_mut());
                    cx.bool_result()
                },
            )?;
        router
            .func_wrap(
                "variants",
                "option_arg",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: Option<bool>,
                    b: Option<()>,
                    c: Option<u32>,
                    d: Option<E1>,
                    e: Option<f32>,
                    f: Option<U1>,
                    g: Option<Option<bool>>,
                | -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.option_arg(a, b, c, d, e, f, g)
                },
            )?;
        router
            .func_wrap(
                "variants",
                "option_result",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> (
                    Option<bool>,
                    Option<()>,
                    Option<u32>,
                    Option<E1>,
                    Option<f32>,
                    Option<U1>,
                    Option<Option<bool>>,
                ) {
                    let cx = get_cx(cx.data_mut());
                    cx.option_result()
                },
            )?;
        router
            .func_wrap(
                "variants",
                "casts",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: Casts1,
                    b: Casts2,
                    c: Casts3,
                    d: Casts4,
                    e: Casts5,
                    f: Casts6,
                | -> (Casts1, Casts2, Casts3, Casts4, Casts5, Casts6) {
                    let cx = get_cx(cx.data_mut());
                    cx.casts(a, b, c, d, e, f)
                },
            )?;
        router
            .func_wrap(
                "variants",
                "result_arg",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: Result<(), ()>,
                    b: Result<(), E1>,
                    c: Result<E1, ()>,
                    d: Result<(), ()>,
                    e: Result<u32, V1>,
                    f: Result<String, Vec<u8>>,
                | -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.result_arg(a, b, c, d, e, f)
                },
            )?;
        router
            .func_wrap(
                "variants",
                "result_result",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> (
                    Result<(), ()>,
                    Result<(), E1>,
                    Result<E1, ()>,
                    Result<(), ()>,
                    Result<u32, V1<'_>>,
                    Result<&'_ str, &'_ [u8]>,
                ) {
                    let cx = get_cx(cx.data_mut());
                    cx.result_result()
                },
            )?;
        router
            .func_wrap(
                "variants",
                "return_result_sugar",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> Result<i32, MyErrno> {
                    let cx = get_cx(cx.data_mut());
                    cx.return_result_sugar()
                },
            )?;
        router
            .func_wrap(
                "variants",
                "return_result_sugar2",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> Result<(), MyErrno> {
                    let cx = get_cx(cx.data_mut());
                    cx.return_result_sugar2()
                },
            )?;
        router
            .func_wrap(
                "variants",
                "return_result_sugar3",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> Result<MyErrno, MyErrno> {
                    let cx = get_cx(cx.data_mut());
                    cx.return_result_sugar3()
                },
            )?;
        router
            .func_wrap(
                "variants",
                "return_result_sugar4",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> Result<(i32, u32), MyErrno> {
                    let cx = get_cx(cx.data_mut());
                    cx.return_result_sugar4()
                },
            )?;
        router
            .func_wrap(
                "variants",
                "return_option_sugar",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> Option<i32> {
                    let cx = get_cx(cx.data_mut());
                    cx.return_option_sugar()
                },
            )?;
        router
            .func_wrap(
                "variants",
                "return_option_sugar2",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> Option<MyErrno> {
                    let cx = get_cx(cx.data_mut());
                    cx.return_option_sugar2()
                },
            )?;
        router
            .func_wrap(
                "variants",
                "result_simple",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> Result<u32, i32> {
                    let cx = get_cx(cx.data_mut());
                    cx.result_simple()
                },
            )?;
        router
            .func_wrap(
                "variants",
                "is_clone_arg",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    a: IsClone,
                | -> () {
                    let cx = get_cx(cx.data_mut());
                    cx.is_clone_arg(a)
                },
            )?;
        router
            .func_wrap(
                "variants",
                "is_clone_return",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> IsClone<'_> {
                    let cx = get_cx(cx.data_mut());
                    cx.is_clone_return()
                },
            )?;
        router
            .func_wrap(
                "variants",
                "return_named_option",
                move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>| -> Option<u8> {
                    let cx = get_cx(cx.data_mut());
                    cx.return_named_option()
                },
            )?;
        router
            .func_wrap(
                "variants",
                "return_named_result",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> Result<u8, MyErrno> {
                    let cx = get_cx(cx.data_mut());
                    cx.return_named_result()
                },
            )?;
        Ok(())
    }
}
