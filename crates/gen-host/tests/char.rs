#[allow(clippy::all)]
pub mod imports {
    #[host::async_trait]
    pub trait Imports: Sized {
        /// A function that accepts a character
        async fn take_char(&mut self, x: char) -> anyhow::Result<()>;
        /// A function that returns a character
        async fn return_char(&mut self) -> anyhow::Result<char>;
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
