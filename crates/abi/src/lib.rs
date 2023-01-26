use std::{
    io::{self, Read},
    mem::MaybeUninit,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    /// An error returned when an operation could not be completed because an "end of file" was reached prematurely.
    /// This typically means that an operation could only succeed if it read a particular number of bytes but only a smaller number of bytes could be read.
    #[error("Unexpected End Of File")]
    UnexpectedEof,
    #[error("Invalid Character")]
    InvalidChar,
    #[error("Invalid Flags")]
    InvalidFlags,
}

pub trait Writable {
    /// # Errors
    ///
    /// Implementations should return errors when writing to the underlying [`std::io::Write`] implementation fails.
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error>;
}

pub trait Readable: Sized {
    /// # Errors
    ///
    /// Implementations should return errors when reading from the underlying [`std::io::Read`] implementation fails or when validating data fails.
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error>;
}

macro_rules! ensure {
    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            return ::std::result::Result::Err($err);
        }
    };
}

impl Readable for u8 {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<u8> = MaybeUninit::uninit();
        unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(value.as_mut_ptr(), 1))?;

            ensure!(bytes_read == 1, Error::UnexpectedEof);

            Ok(value.assume_init())
        }
    }
}

impl Readable for u16 {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<u16> = MaybeUninit::uninit();
        unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(
                value.as_mut_ptr().cast::<u8>(),
                2,
            ))?;

            ensure!(bytes_read == 2, Error::UnexpectedEof);

            Ok(value.assume_init())
        }
    }
}

impl Readable for u32 {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(
                value.as_mut_ptr().cast::<u8>(),
                4,
            ))?;

            ensure!(bytes_read == 4, Error::UnexpectedEof);

            Ok(value.assume_init())
        }
    }
}

impl Readable for u64 {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<u64> = MaybeUninit::uninit();
        unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(
                value.as_mut_ptr().cast::<u8>(),
                8,
            ))?;

            ensure!(bytes_read == 8, Error::UnexpectedEof);

            Ok(value.assume_init())
        }
    }
}

impl Readable for u128 {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<u128> = MaybeUninit::uninit();
        unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(
                value.as_mut_ptr().cast::<u8>(),
                16,
            ))?;

            ensure!(bytes_read == 16, Error::UnexpectedEof);

            Ok(value.assume_init())
        }
    }
}

impl Readable for usize {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(
                value.as_mut_ptr().cast::<u8>(),
                4,
            ))?;

            ensure!(bytes_read == 4, Error::UnexpectedEof);

            Ok(value.assume_init() as usize)
        }
    }
}

impl Readable for i8 {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<i8> = MaybeUninit::uninit();
        unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(
                value.as_mut_ptr().cast::<u8>(),
                1,
            ))?;

            ensure!(bytes_read == 1, Error::UnexpectedEof);

            Ok(value.assume_init())
        }
    }
}

impl Readable for i16 {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<i16> = MaybeUninit::uninit();
        unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(
                value.as_mut_ptr().cast::<u8>(),
                2,
            ))?;

            ensure!(bytes_read == 2, Error::UnexpectedEof);

            Ok(value.assume_init())
        }
    }
}

impl Readable for i32 {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<i32> = MaybeUninit::uninit();
        unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(
                value.as_mut_ptr().cast::<u8>(),
                4,
            ))?;

            ensure!(bytes_read == 4, Error::UnexpectedEof);

            Ok(value.assume_init())
        }
    }
}

impl Readable for i64 {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<i64> = MaybeUninit::uninit();
        unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(
                value.as_mut_ptr().cast::<u8>(),
                8,
            ))?;

            ensure!(bytes_read == 8, Error::UnexpectedEof);

            Ok(value.assume_init())
        }
    }
}

impl Readable for i128 {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<i128> = MaybeUninit::uninit();
        unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(
                value.as_mut_ptr().cast::<u8>(),
                16,
            ))?;

            ensure!(bytes_read == 16, Error::UnexpectedEof);

            Ok(value.assume_init())
        }
    }
}

impl Readable for isize {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<isize> = MaybeUninit::uninit();
        unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(
                value.as_mut_ptr().cast::<u8>(),
                4,
            ))?;

            ensure!(bytes_read == 4, Error::UnexpectedEof);

            Ok(value.assume_init())
        }
    }
}

impl Readable for f32 {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<f32> = MaybeUninit::uninit();
        unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(
                value.as_mut_ptr().cast::<u8>(),
                4,
            ))?;

            ensure!(bytes_read == 4, Error::UnexpectedEof);

            Ok(value.assume_init())
        }
    }
}

impl Readable for f64 {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<f64> = MaybeUninit::uninit();
        unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(
                value.as_mut_ptr().cast::<u8>(),
                8,
            ))?;

            ensure!(bytes_read == 8, Error::UnexpectedEof);

            Ok(value.assume_init())
        }
    }
}

impl Readable for char {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<u32> = MaybeUninit::uninit();
        let value = unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(
                value.as_mut_ptr().cast::<u8>(),
                4,
            ))?;

            ensure!(bytes_read == 4, Error::UnexpectedEof);

            value.assume_init()
        };

        char::from_u32(value).ok_or(Error::InvalidChar)
    }
}

impl Readable for bool {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let mut value: MaybeUninit<bool> = MaybeUninit::uninit();
        unsafe {
            let bytes_read = read.read(std::slice::from_raw_parts_mut(
                value.as_mut_ptr().cast::<u8>(),
                1,
            ))?;

            ensure!(bytes_read == 1, Error::UnexpectedEof);

            Ok(value.assume_init())
        }
    }
}

impl<T: Readable> Readable for Vec<T> {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let length = usize::read_from(read)?;

        Ok((0..length)
            .into_iter()
            .map(|i| {
                T::read_from(read)
                    .unwrap_or_else(|err| panic!("failed to read el {i} with error {err}"))
            })
            .collect())
    }
}

impl Readable for String {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let length = usize::read_from(read)?;

        let mut value = String::with_capacity(length);
        let bytes_read = read.take(length as u64).read_to_string(&mut value)?;

        ensure!(bytes_read == length, Error::UnexpectedEof);

        Ok(value)
    }
}

impl<T: Readable> Readable for Option<T> {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let tag = u8::read_from(read)?;

        match tag {
            0 => Ok(None),
            1 => Ok(Some(T::read_from(read)?)),
            _ => panic!(),
        }
    }
}

impl<T: Readable, E: Readable> Readable for Result<T, E> {
    fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
        let tag = u8::read_from(read)?;

        match tag {
            0 => Ok(Ok(T::read_from(read)?)),
            1 => Ok(Err(E::read_from(read)?)),
            _ => panic!(),
        }
    }
}

impl Writable for u8 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for u16 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for u32 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for u64 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for u128 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for usize {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes()[0..4])?;
        Ok(())
    }
}

impl Writable for i8 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for i16 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for i32 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for i64 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for i128 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for isize {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes()[0..4])?;
        Ok(())
    }
}

impl Writable for f32 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for f64 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }
}

impl Writable for char {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        Writable::write_to(&(*self as u32), write)
    }
}

impl Writable for bool {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&[u8::from(*self)])?;
        Ok(())
    }
}

impl<T: Writable> Writable for Vec<T> {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        self.len().write_to(write)?;

        for el in self {
            el.write_to(write)?;
        }

        Ok(())
    }
}

impl Writable for String {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        self.len().write_to(write)?;

        write.write_all(self.as_bytes())?;

        Ok(())
    }
}

impl<T: Writable> Writable for Option<T> {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        match self {
            None => 0u8.write_to(write),
            Some(val) => {
                1u8.write_to(write)?;
                val.write_to(write)
            }
        }
    }
}

impl<T: Writable, E: Writable> Writable for Result<T, E> {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        match self {
            Ok(val) => {
                0u8.write_to(write)?;
                val.write_to(write)
            }
            Err(err) => {
                1u8.write_to(write)?;
                err.write_to(write)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Error, Readable, Writable};
    use std::io;

    #[test]
    fn struct_() {
        #[derive(Debug, PartialEq)]
        struct Foo {
            a: u8,
            b: u64,
            c: String,
        }

        impl Readable for Foo {
            fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
                Ok(Self {
                    a: Readable::read_from(read)?,
                    b: Readable::read_from(read)?,
                    c: Readable::read_from(read)?,
                })
            }
        }

        impl Writable for Foo {
            fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
                self.a.write_to(write)?;
                self.b.write_to(write)?;
                self.c.write_to(write)?;

                Ok(())
            }
        }

        let input = Foo {
            a: 3,
            b: 16,
            c: "foo".to_string(),
        };

        let mut bytes: Vec<u8> = vec![];
        Writable::write_to(&input, &mut bytes).unwrap();

        assert_eq!(
            bytes,
            vec![3, 16, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 102, 111, 111]
        );

        let output: Foo = Readable::read_from(&mut bytes.as_slice()).unwrap();

        assert_eq!(output, input);
    }

    #[test]
    fn variant() {
        #[derive(Debug, PartialEq)]
        pub enum U1 {
            U32(u32),
            F32(f32),
        }

        impl Readable for U1 {
            fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
                let tag = u8::read_from(read)?;

                match tag {
                    0 => Ok(Self::U32(Readable::read_from(read)?)),
                    1 => Ok(Self::F32(Readable::read_from(read)?)),
                    _ => panic!(),
                }
            }
        }

        impl Writable for U1 {
            fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
                match self {
                    U1::U32(val) => {
                        0u8.write_to(write)?;
                        val.write_to(write)
                    }
                    U1::F32(val) => {
                        1u8.write_to(write)?;
                        val.write_to(write)
                    }
                }
            }
        }

        let input = U1::U32(50);

        let mut bytes: Vec<u8> = vec![];
        Writable::write_to(&input, &mut bytes).unwrap();

        assert_eq!(bytes, vec![0, 50, 0, 0, 0]);

        let output: U1 = Readable::read_from(&mut bytes.as_slice()).unwrap();

        assert_eq!(output, input);
    }

    #[test]
    fn flags() {
        bitflags::bitflags! {
          pub struct Flag4: u8 {
            const B0 = 1 << 0;
            const B1 = 1 << 1;
            const B2 = 1 << 2;
            const B3 = 1 << 3;
          }
        }

        impl Readable for Flag4 {
            fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
                let bits = Readable::read_from(read)?;
                Self::from_bits(bits).ok_or(Error::InvalidFlags)
            }
        }

        impl Writable for Flag4 {
            fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
                self.bits().write_to(write)
            }
        }

        let input = Flag4::B0 | Flag4::B2;

        let mut bytes: Vec<u8> = vec![];
        Writable::write_to(&input, &mut bytes).unwrap();

        assert_eq!(bytes, vec![0b0000_0101]);

        let output: Flag4 = Readable::read_from(&mut bytes.as_slice()).unwrap();

        assert_eq!(output, input);
    }
}
