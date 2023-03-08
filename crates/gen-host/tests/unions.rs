pub mod unions {
    use ::tauri_bindgen_host::tauri_bindgen_abi;
    use ::tauri_bindgen_host::bitflags;
    ///A union of all of the integral types
    #[derive(tauri_bindgen_abi::Readable)]
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
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum AllFloats {
        F32(f32),
        F64(f64),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum AllText {
        Char(char),
        String(String),
    }
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum DuplicatedS32 {
        ///The first s32
        S320(i32),
        ///The second s32
        S321(i32),
        ///The third s32
        S322(i32),
    }
    ///A type containing numeric types that are distinct in most languages
    #[derive(tauri_bindgen_abi::Readable)]
    pub enum DistinguishableNum {
        ///A Floating Point Number
        F64(f64),
        ///A Signed Integer
        S64(i64),
    }
    pub trait Unions: Sized {
        fn add_one_integer(&mut self, num: AllIntegers) -> AllIntegers;
        fn add_one_float(&mut self, num: AllFloats) -> AllFloats;
        fn replace_first_char(&mut self, text: AllText, letter: char) -> AllText<'_>;
        fn identify_integer(&mut self, num: AllIntegers) -> u8;
        fn identify_float(&mut self, num: AllFloats) -> u8;
        fn identify_text(&mut self, text: AllText) -> u8;
        fn add_one_duplicated(&mut self, num: DuplicatedS32) -> DuplicatedS32;
        fn identify_duplicated(&mut self, num: DuplicatedS32) -> u8;
        fn add_one_distinguishable_num(
            &mut self,
            num: DistinguishableNum,
        ) -> DistinguishableNum;
        fn identify_distinguishable_num(&mut self, num: DistinguishableNum) -> u8;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Unions,
    {
        router
            .func_wrap(
                "unions",
                "add_one_integer",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: AllIntegers,
                | -> AllIntegers {
                    let cx = get_cx(cx.data_mut());
                    cx.add_one_integer(num)
                },
            )?;
        router
            .func_wrap(
                "unions",
                "add_one_float",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: AllFloats,
                | -> AllFloats {
                    let cx = get_cx(cx.data_mut());
                    cx.add_one_float(num)
                },
            )?;
        router
            .func_wrap(
                "unions",
                "replace_first_char",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    text: AllText,
                    letter: char,
                | -> AllText<'_> {
                    let cx = get_cx(cx.data_mut());
                    cx.replace_first_char(text, letter)
                },
            )?;
        router
            .func_wrap(
                "unions",
                "identify_integer",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: AllIntegers,
                | -> u8 {
                    let cx = get_cx(cx.data_mut());
                    cx.identify_integer(num)
                },
            )?;
        router
            .func_wrap(
                "unions",
                "identify_float",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: AllFloats,
                | -> u8 {
                    let cx = get_cx(cx.data_mut());
                    cx.identify_float(num)
                },
            )?;
        router
            .func_wrap(
                "unions",
                "identify_text",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    text: AllText,
                | -> u8 {
                    let cx = get_cx(cx.data_mut());
                    cx.identify_text(text)
                },
            )?;
        router
            .func_wrap(
                "unions",
                "add_one_duplicated",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: DuplicatedS32,
                | -> DuplicatedS32 {
                    let cx = get_cx(cx.data_mut());
                    cx.add_one_duplicated(num)
                },
            )?;
        router
            .func_wrap(
                "unions",
                "identify_duplicated",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: DuplicatedS32,
                | -> u8 {
                    let cx = get_cx(cx.data_mut());
                    cx.identify_duplicated(num)
                },
            )?;
        router
            .func_wrap(
                "unions",
                "add_one_distinguishable_num",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: DistinguishableNum,
                | -> DistinguishableNum {
                    let cx = get_cx(cx.data_mut());
                    cx.add_one_distinguishable_num(num)
                },
            )?;
        router
            .func_wrap(
                "unions",
                "identify_distinguishable_num",
                move |
                    cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    num: DistinguishableNum,
                | -> u8 {
                    let cx = get_cx(cx.data_mut());
                    cx.identify_distinguishable_num(num)
                },
            )?;
        Ok(())
    }
}
