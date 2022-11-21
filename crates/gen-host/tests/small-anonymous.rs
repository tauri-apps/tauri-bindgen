#[allow(clippy::all)]
pub mod imports {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy)]
    pub enum Error {
        Success,
        Failure,
    }
    #[host::async_trait]
    pub trait Imports: Sized {
        async fn option_test(&mut self) -> anyhow::Result<Result<Option<String>, Error>>;
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
