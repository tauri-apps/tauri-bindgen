#[allow(clippy::all, unused)]
pub mod imports {
    #[repr(u8)]
    #[derive(Debug, Clone, Copy)]
    pub enum Error {
        Success,
        Failure,
    }
    pub fn option_test() -> Result<Option<String>, Error> {
        todo!()
    }
}
