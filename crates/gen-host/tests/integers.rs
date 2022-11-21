#[allow(clippy::all)]
pub mod imports {
    #[host::async_trait]
    pub trait Imports: Sized {
        async fn a1(&mut self, x: u8) -> anyhow::Result<()>;
        async fn a2(&mut self, x: i8) -> anyhow::Result<()>;
        async fn a3(&mut self, x: u16) -> anyhow::Result<()>;
        async fn a4(&mut self, x: i16) -> anyhow::Result<()>;
        async fn a5(&mut self, x: u32) -> anyhow::Result<()>;
        async fn a6(&mut self, x: i32) -> anyhow::Result<()>;
        async fn a7(&mut self, x: u64) -> anyhow::Result<()>;
        async fn a8(&mut self, x: i64) -> anyhow::Result<()>;
        async fn a9(
            &mut self,
            p1: u8,
            p2: i8,
            p3: u16,
            p4: i16,
            p5: u32,
            p6: i32,
            p7: u64,
            p8: i64,
        ) -> anyhow::Result<()>;
        async fn r1(&mut self) -> anyhow::Result<u8>;
        async fn r2(&mut self) -> anyhow::Result<i8>;
        async fn r3(&mut self) -> anyhow::Result<u16>;
        async fn r4(&mut self) -> anyhow::Result<i16>;
        async fn r5(&mut self) -> anyhow::Result<u32>;
        async fn r6(&mut self) -> anyhow::Result<i32>;
        async fn r7(&mut self) -> anyhow::Result<u64>;
        async fn r8(&mut self) -> anyhow::Result<i64>;
        async fn pair_ret(&mut self) -> anyhow::Result<(i64, u8)>;
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
