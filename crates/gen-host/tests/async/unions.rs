#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod unions {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    ///A union of all of the integral types
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum AllIntegers {
        /**Bool is equivalent to a 1 bit integer
and is treated that way in some languages*/
        Bool(bool),
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        I8(i8),
        I16(i16),
        S32(i32),
        S64(i64),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum AllFloats {
        F32(f32),
        F64(f64),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum AllText {
        Char(char),
        String(String),
    }
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum DuplicatedS32 {
        ///The first s32
        S320(i32),
        ///The second s32
        S321(i32),
        ///The third s32
        S322(i32),
    }
    ///A type containing numeric types that are distinct in most languages
    #[derive(serde::Deserialize, serde::Serialize)]
    #[derive(Debug, Clone, PartialEq)]
    pub enum DistinguishableNum {
        ///A Floating Point Number
        F64(f64),
        ///A Signed Integer
        S64(i64),
    }
    #[::tauri_bindgen_host::async_trait]
    pub trait Unions: Sized {
        async fn add_one_integer(&self, num: AllIntegers) -> AllIntegers;
        async fn add_one_float(&self, num: AllFloats) -> AllFloats;
        async fn replace_first_char(&self, text: AllText, letter: char) -> AllText;
        async fn identify_integer(&self, num: AllIntegers) -> u8;
        async fn identify_float(&self, num: AllFloats) -> u8;
        async fn identify_text(&self, text: AllText) -> u8;
        async fn add_one_duplicated(&self, num: DuplicatedS32) -> DuplicatedS32;
        async fn identify_duplicated(&self, num: DuplicatedS32) -> u8;
        async fn add_one_distinguishable_num(
            &self,
            num: DistinguishableNum,
        ) -> DistinguishableNum;
        async fn identify_distinguishable_num(&self, num: DistinguishableNum) -> u8;
    }
    pub fn add_to_router<T, U, R>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T, R>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: Unions + Send + Sync + 'static,
        R: ::tauri_bindgen_host::tauri::Runtime,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "unions",
                "add_one_integer",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: AllIntegers|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.add_one_integer(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "unions",
                "add_one_float",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: AllFloats|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.add_one_float(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "unions",
                "replace_first_char",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: (AllText, char)|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.replace_first_char(p.0, p.1).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "unions",
                "identify_integer",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: AllIntegers|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.identify_integer(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "unions",
                "identify_float",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: AllFloats|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.identify_float(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "unions",
                "identify_text",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: AllText| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.identify_text(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "unions",
                "add_one_duplicated",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: DuplicatedS32|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.add_one_duplicated(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "unions",
                "identify_duplicated",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: DuplicatedS32|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.identify_duplicated(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "unions",
                "add_one_distinguishable_num",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: DistinguishableNum|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.add_one_distinguishable_num(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "unions",
                "identify_distinguishable_num",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    p: DistinguishableNum|
                {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.identify_distinguishable_num(p).await)
                    })
                },
            )?;
        Ok(())
    }
}
