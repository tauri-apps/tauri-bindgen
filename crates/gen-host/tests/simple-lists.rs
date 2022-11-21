#[allow(clippy::all)]
pub mod imports {
    #[host::async_trait]
    pub trait Imports: Sized {
        async fn simple_list1(&mut self, l: Vec<u32>) -> anyhow::Result<()>;
        async fn simple_list2(&mut self) -> anyhow::Result<Vec<u32>>;
        async fn simple_list4(&mut self, l: Vec<Vec<u32>>) -> anyhow::Result<Vec<Vec<u32>>>;
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
