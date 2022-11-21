#[allow(clippy::all)]
pub mod imports {
    #[host::async_trait]
    pub trait Imports: Sized {
        async fn f1(&mut self) -> anyhow::Result<()>;
        async fn f2(&mut self, a: u32) -> anyhow::Result<()>;
        async fn f3(&mut self, a: u32, b: u32) -> anyhow::Result<()>;
        async fn f4(&mut self) -> anyhow::Result<u32>;
        async fn f5(&mut self) -> anyhow::Result<(u32, u32)>;
        async fn f6(&mut self, a: u32, b: u32, c: u32) -> anyhow::Result<(u32, u32, u32)>;
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
