#[allow(clippy::all)]
pub mod imports {
    #[host::async_trait]
    pub trait Imports: Sized {
        async fn float32_param(&mut self, x: f32) -> anyhow::Result<()>;
        async fn float64_param(&mut self, x: f64) -> anyhow::Result<()>;
        async fn float32_result(&mut self) -> anyhow::Result<f32>;
        async fn float64_result(&mut self) -> anyhow::Result<f64>;
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
