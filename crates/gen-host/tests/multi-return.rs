#[allow(clippy::all)]
pub mod imports {
    #[host::async_trait]
    pub trait Imports: Sized {
        async fn mra(&mut self) -> anyhow::Result<()>;
        async fn mrb(&mut self) -> anyhow::Result<()>;
        async fn mrc(&mut self) -> anyhow::Result<u32>;
        async fn mrd(&mut self) -> anyhow::Result<u32>;
        async fn mre(&mut self) -> anyhow::Result<(u32, f32)>;
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
