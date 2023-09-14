#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod flegs {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    bitflags::bitflags! {
        #[derive(serde::Deserialize, serde::Serialize)] pub struct Flag1 : u8 { const B0
        = 1 << 0; }
    }
    bitflags::bitflags! {
        #[derive(serde::Deserialize, serde::Serialize)] pub struct Flag2 : u8 { const B0
        = 1 << 0; const B1 = 1 << 1; }
    }
    bitflags::bitflags! {
        #[derive(serde::Deserialize, serde::Serialize)] pub struct Flag4 : u8 { const B0
        = 1 << 0; const B1 = 1 << 1; const B2 = 1 << 2; const B3 = 1 << 3; }
    }
    bitflags::bitflags! {
        #[derive(serde::Deserialize, serde::Serialize)] pub struct Flag8 : u8 { const B0
        = 1 << 0; const B1 = 1 << 1; const B2 = 1 << 2; const B3 = 1 << 3; const B4 = 1
        << 4; const B5 = 1 << 5; const B6 = 1 << 6; const B7 = 1 << 7; }
    }
    bitflags::bitflags! {
        #[derive(serde::Deserialize, serde::Serialize)] pub struct Flag16 : u16 { const
        B0 = 1 << 0; const B1 = 1 << 1; const B2 = 1 << 2; const B3 = 1 << 3; const B4 =
        1 << 4; const B5 = 1 << 5; const B6 = 1 << 6; const B7 = 1 << 7; const B8 = 1 <<
        8; const B9 = 1 << 9; const B10 = 1 << 10; const B11 = 1 << 11; const B12 = 1 <<
        12; const B13 = 1 << 13; const B14 = 1 << 14; const B15 = 1 << 15; }
    }
    bitflags::bitflags! {
        #[derive(serde::Deserialize, serde::Serialize)] pub struct Flag32 : u32 { const
        B0 = 1 << 0; const B1 = 1 << 1; const B2 = 1 << 2; const B3 = 1 << 3; const B4 =
        1 << 4; const B5 = 1 << 5; const B6 = 1 << 6; const B7 = 1 << 7; const B8 = 1 <<
        8; const B9 = 1 << 9; const B10 = 1 << 10; const B11 = 1 << 11; const B12 = 1 <<
        12; const B13 = 1 << 13; const B14 = 1 << 14; const B15 = 1 << 15; const B16 = 1
        << 16; const B17 = 1 << 17; const B18 = 1 << 18; const B19 = 1 << 19; const B20 =
        1 << 20; const B21 = 1 << 21; const B22 = 1 << 22; const B23 = 1 << 23; const B24
        = 1 << 24; const B25 = 1 << 25; const B26 = 1 << 26; const B27 = 1 << 27; const
        B28 = 1 << 28; const B29 = 1 << 29; const B30 = 1 << 30; const B31 = 1 << 31; }
    }
    bitflags::bitflags! {
        #[derive(serde::Deserialize, serde::Serialize)] pub struct Flag64 : u64 { const
        B0 = 1 << 0; const B1 = 1 << 1; const B2 = 1 << 2; const B3 = 1 << 3; const B4 =
        1 << 4; const B5 = 1 << 5; const B6 = 1 << 6; const B7 = 1 << 7; const B8 = 1 <<
        8; const B9 = 1 << 9; const B10 = 1 << 10; const B11 = 1 << 11; const B12 = 1 <<
        12; const B13 = 1 << 13; const B14 = 1 << 14; const B15 = 1 << 15; const B16 = 1
        << 16; const B17 = 1 << 17; const B18 = 1 << 18; const B19 = 1 << 19; const B20 =
        1 << 20; const B21 = 1 << 21; const B22 = 1 << 22; const B23 = 1 << 23; const B24
        = 1 << 24; const B25 = 1 << 25; const B26 = 1 << 26; const B27 = 1 << 27; const
        B28 = 1 << 28; const B29 = 1 << 29; const B30 = 1 << 30; const B31 = 1 << 31;
        const B32 = 1 << 32; const B33 = 1 << 33; const B34 = 1 << 34; const B35 = 1 <<
        35; const B36 = 1 << 36; const B37 = 1 << 37; const B38 = 1 << 38; const B39 = 1
        << 39; const B40 = 1 << 40; const B41 = 1 << 41; const B42 = 1 << 42; const B43 =
        1 << 43; const B44 = 1 << 44; const B45 = 1 << 45; const B46 = 1 << 46; const B47
        = 1 << 47; const B48 = 1 << 48; const B49 = 1 << 49; const B50 = 1 << 50; const
        B51 = 1 << 51; const B52 = 1 << 52; const B53 = 1 << 53; const B54 = 1 << 54;
        const B55 = 1 << 55; const B56 = 1 << 56; const B57 = 1 << 57; const B58 = 1 <<
        58; const B59 = 1 << 59; const B60 = 1 << 60; const B61 = 1 << 61; const B62 = 1
        << 62; const B63 = 1 << 63; }
    }
    #[::tauri_bindgen_host::async_trait]
    pub trait Flegs: Sized {
        async fn roundtrip_flag1(&self, x: Flag1) -> Flag1;
        async fn roundtrip_flag2(&self, x: Flag2) -> Flag2;
        async fn roundtrip_flag4(&self, x: Flag4) -> Flag4;
        async fn roundtrip_flag8(&self, x: Flag8) -> Flag8;
        async fn roundtrip_flag16(&self, x: Flag16) -> Flag16;
        async fn roundtrip_flag32(&self, x: Flag32) -> Flag32;
        async fn roundtrip_flag64(&self, x: Flag64) -> Flag64;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        T: Send + Sync + 'static,
        U: Flegs + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "flegs",
                "roundtrip_flag1",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Flag1| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.roundtrip_flag1(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "flegs",
                "roundtrip_flag2",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Flag2| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.roundtrip_flag2(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "flegs",
                "roundtrip_flag4",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Flag4| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.roundtrip_flag4(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "flegs",
                "roundtrip_flag8",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Flag8| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.roundtrip_flag8(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "flegs",
                "roundtrip_flag16",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Flag16| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.roundtrip_flag16(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "flegs",
                "roundtrip_flag32",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Flag32| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.roundtrip_flag32(p).await)
                    })
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .define_async(
                "flegs",
                "roundtrip_flag64",
                move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: Flag64| {
                    let get_cx = get_cx.clone();
                    Box::pin(async move {
                        let ctx = get_cx(ctx.data());
                        Ok(ctx.roundtrip_flag64(p).await)
                    })
                },
            )?;
        Ok(())
    }
}
