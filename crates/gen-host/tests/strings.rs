#[allow(clippy::all)]
pub mod imports {
    #[host::async_trait]
    pub trait Imports: Sized {
        async fn a(&mut self, x: String) -> anyhow::Result<()>;
        async fn b(&mut self) -> anyhow::Result<String>;
        async fn c(&mut self, a: String, b: String) -> anyhow::Result<String>;
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
