#[allow(clippy::all)]
pub mod import_unions {
    /// A union of all of the integral types
    #[derive(Debug, Clone, Copy)]
    pub enum AllIntegers {
        /// Bool is equivalent to a 1 bit integer
        /// and is treated that way in some languages
        Bool(bool),
        U8(u8),
        U16(u16),
        U32(u32),
        U64(u64),
        I8(i8),
        I16(i16),
        I32(i32),
        I64(i64),
    }
    #[derive(Debug, Clone, Copy)]
    pub enum AllFloats {
        F32(f32),
        F64(f64),
    }
    #[derive(Debug, Clone)]
    pub enum AllText {
        Char(char),
        String(String),
    }
    #[derive(Debug, Clone, Copy)]
    pub enum DuplicatedS32 {
        /// The first s32
        I320(i32),
        /// The second s32
        I321(i32),
        /// The third s32
        I322(i32),
    }
    /// A type containing numeric types that are distinct in most languages
    #[derive(Debug, Clone, Copy)]
    pub enum DistinguishableNum {
        /// A Floating Point Number
        F64(f64),
        /// A Signed Integer
        I64(i64),
    }
    #[host::async_trait]
    pub trait ImportUnions: Sized {
        async fn add_one_integer(&mut self, num: AllIntegers) -> anyhow::Result<AllIntegers>;
        async fn add_one_float(&mut self, num: AllFloats) -> anyhow::Result<AllFloats>;
        async fn replace_first_char(
            &mut self,
            text: AllText,
            letter: char,
        ) -> anyhow::Result<AllText>;
        async fn identify_integer(&mut self, num: AllIntegers) -> anyhow::Result<u8>;
        async fn identify_float(&mut self, num: AllFloats) -> anyhow::Result<u8>;
        async fn identify_text(&mut self, text: AllText) -> anyhow::Result<u8>;
        async fn add_one_duplicated(&mut self, num: DuplicatedS32)
            -> anyhow::Result<DuplicatedS32>;
        async fn identify_duplicated(&mut self, num: DuplicatedS32) -> anyhow::Result<u8>;
        async fn add_one_distinguishable_num(
            &mut self,
            num: DistinguishableNum,
        ) -> anyhow::Result<DistinguishableNum>;
        async fn identify_distinguishable_num(
            &mut self,
            num: DistinguishableNum,
        ) -> anyhow::Result<u8>;
    }

    pub fn add_to_linker<T, U>(
        _linker: &mut (),
        _get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()>
    where
        T: Send,
        U: ImportUnions + Send,
    {
        todo!()
    }
}
