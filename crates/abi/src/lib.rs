use std::{
    io::{self, Read},
    mem::{self, MaybeUninit},
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

/// # Errors
///
/// This conversion can fail if the structure of the input does not match the structure expected by `T`,
/// for example if `T` is an enum type but the input doesn't lead with a valid tag.
/// It can also fail if the structure is correct but T’s implementation of `Readable` decides decides that something is wrong with the data,
/// for example when the bytes given for a `char` are outside the allowed ranges.
/// It will also fail if the [`std::io::Read`] ends prematurely without delivering enough data for deserializing the type.
pub fn from_reader<R, T>(rdr: &mut R) -> Result<T, Error>
where
    R: Read,
    T: Readable,
{
    T::read_from(rdr)
}

/// # Errors
///
/// This conversion can fail if the structure of the input does not match the structure expected by `T`,
/// for example if `T` is an enum type but the input doesn't lead with a valid tag.
/// It can also fail if the structure is correct but T’s implementation of `Readable` decides decides that something is wrong with the data,
/// for example when the bytes given for a `char` are outside the allowed ranges.
/// It will also fail if byte slice ends prematurely.
pub fn from_slice<T>(mut v: &[u8]) -> Result<T, Error>
where
    T: Readable,
{
    T::read_from(&mut v)
}

/// # Errors
///
/// Serialization can fail if T’s implementation of `Writable` decides to fail, or if the `io::Write` fails.
pub fn to_writer<W, T>(writer: &mut W, value: &T) -> Result<(), Error>
where
    W: io::Write,
    T: ?Sized + Writable,
{
    value.write_to(writer).map_err(Into::into)
}

/// # Errors
///
/// Serialization can fail if T’s implementation of `Writable` decides to fail.
pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>, Error>
where
    T: ?Sized + Writable,
{
    let mut bytes = Vec::with_capacity(value.size_hint());
    value.write_to(&mut bytes)?;

    Ok(bytes)
}

pub trait Writable {
    /// # Errors
    ///
    /// Implementations should return errors when writing to the underlying [`std::io::Write`] implementation fails.
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error>;

    fn size_hint(&self) -> usize;
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

    fn size_hint(&self) -> usize {
        mem::size_of::<Self>()
    }
}

impl Writable for u16 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<Self>()
    }
}

impl Writable for u32 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<Self>()
    }
}

impl Writable for u64 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<Self>()
    }
}

impl Writable for u128 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<Self>()
    }
}

impl Writable for usize {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes()[0..4])?;
        Ok(())
    }

    fn size_hint(&self) -> usize {
        4
    }
}

impl Writable for i8 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<Self>()
    }
}

impl Writable for i16 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<Self>()
    }
}

impl Writable for i32 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<Self>()
    }
}

impl Writable for i64 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<Self>()
    }
}

impl Writable for i128 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<Self>()
    }
}

impl Writable for isize {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes()[0..4])?;
        Ok(())
    }

    fn size_hint(&self) -> usize {
        4
    }
}

impl Writable for f32 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<Self>()
    }
}

impl Writable for f64 {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&self.to_le_bytes())?;
        Ok(())
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<Self>()
    }
}

impl Writable for char {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        Writable::write_to(&(*self as u32), write)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<Self>()
    }
}

impl Writable for bool {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        write.write_all(&[u8::from(*self)])?;
        Ok(())
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<Self>()
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

    fn size_hint(&self) -> usize {
        let mut size = self.len().size_hint();

        for el in self {
            size += el.size_hint();
        }

        size
    }
}

impl Writable for String {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        self.len().write_to(write)?;

        write.write_all(self.as_bytes())?;

        Ok(())
    }

    fn size_hint(&self) -> usize {
        self.len().size_hint() + self.len()
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

    fn size_hint(&self) -> usize {
        match self {
            Some(val) => 1 + val.size_hint(),
            None => 1,
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

    fn size_hint(&self) -> usize {
        match self {
            Ok(val) => 1 + val.size_hint(),
            Err(err) => 1 + err.size_hint(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Error, Readable, Writable};
    use std::io;

    #[test]
    fn struct_() -> Result<(), Box<dyn std::error::Error>> {
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

            fn size_hint(&self) -> usize {
                self.a.size_hint() + self.b.size_hint() + self.c.size_hint()
            }
        }

        let input = Foo {
            a: 3,
            b: 16,
            c: "foo".to_string(),
        };

        let bytes = crate::to_bytes(&input)?;

        assert_eq!(
            bytes,
            vec![3, 16, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 102, 111, 111]
        );

        let output: Foo = crate::from_slice(&bytes)?;

        assert_eq!(output, input);

        Ok(())
    }

    #[test]
    fn variant() -> Result<(), Box<dyn std::error::Error>> {
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

            fn size_hint(&self) -> usize {
                1 + match self {
                    U1::U32(val) => val.size_hint(),
                    U1::F32(val) => val.size_hint(),
                }
            }
        }

        let input = U1::U32(50);

        let bytes = crate::to_bytes(&input)?;

        assert_eq!(bytes, vec![0, 50, 0, 0, 0]);

        let output: U1 = crate::from_slice(&bytes)?;

        assert_eq!(output, input);

        Ok(())
    }

    #[test]
    fn flags() -> Result<(), Box<dyn std::error::Error>> {
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

            fn size_hint(&self) -> usize {
                std::mem::size_of::<Self>()
            }
        }

        println!("{:?}", Flag4::B1.size_hint());

        let input = Flag4::B0 | Flag4::B2;

        let bytes = crate::to_bytes(&input)?;

        assert_eq!(bytes, vec![0b0000_0101]);

        let output: Flag4 = crate::from_slice(&bytes)?;

        assert_eq!(output, input);

        Ok(())
    }
}
