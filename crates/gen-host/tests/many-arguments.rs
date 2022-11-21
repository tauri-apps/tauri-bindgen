#[allow(clippy::all)]
pub mod imports {
    #[derive(Debug, Clone)]
    pub struct BigStruct {
        pub a1: String,
        pub a2: String,
        pub a3: String,
        pub a4: String,
        pub a5: String,
        pub a6: String,
        pub a7: String,
        pub a8: String,
        pub a9: String,
        pub a10: String,
        pub a11: String,
        pub a12: String,
        pub a13: String,
        pub a14: String,
        pub a15: String,
        pub a16: String,
        pub a17: String,
        pub a18: String,
        pub a19: String,
        pub a20: String,
    }
    #[host::async_trait]
    pub trait Imports: Sized {
        async fn many_args(
            &mut self,
            a1: u64,
            a2: u64,
            a3: u64,
            a4: u64,
            a5: u64,
            a6: u64,
            a7: u64,
            a8: u64,
            a9: u64,
            a10: u64,
            a11: u64,
            a12: u64,
            a13: u64,
            a14: u64,
            a15: u64,
            a16: u64,
        ) -> anyhow::Result<()>;
        async fn big_argument(&mut self, x: BigStruct) -> anyhow::Result<()>;
    }

    pub fn add_to_linker<T, U>(
        _linker: &mut (),
        _get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
    ) -> anyhow::Result<()>
    where
        T: Send,
        U: Imports + Send,
    {
        todo!()
    }
}
