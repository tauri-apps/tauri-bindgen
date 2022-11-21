#[allow(clippy::all)]
pub mod imports {
    #[repr(C)]
    #[derive(Debug, Copy, Clone)]
    pub struct LudicrousSpeed {
        pub how_fast_are_you_going: u32,
        pub i_am_going_extremely_slow: u64,
    }
    #[host::async_trait]
    pub trait Imports: Sized {
        async fn kebab_case(&mut self) -> anyhow::Result<()>;
        async fn foo(&mut self, x: LudicrousSpeed) -> anyhow::Result<()>;
        async fn function_with_dashes(&mut self) -> anyhow::Result<()>;
        async fn function_with_no_weird_characters(&mut self) -> anyhow::Result<()>;
        async fn apple(&mut self) -> anyhow::Result<()>;
        async fn apple_pear(&mut self) -> anyhow::Result<()>;
        async fn apple_pear_grape(&mut self) -> anyhow::Result<()>;
        async fn a0(&mut self) -> anyhow::Result<()>;
        async fn is_xml(&mut self) -> anyhow::Result<()>;
        async fn explicit(&mut self) -> anyhow::Result<()>;
        async fn explicit_kebab(&mut self) -> anyhow::Result<()>;
        async fn bool(&mut self) -> anyhow::Result<()>;
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
