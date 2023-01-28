use std::io::{self, Read};

mod readable_impl;
mod writable_impl;

pub use readable_impl::*;
pub use writable_impl::*;

extern crate self as tauri_bindgen_abi;

// Re-export #[derive(Serialize, Deserialize)].
//
// The reason re-exporting is not enabled by default is that disabling it would
// be annoying for crates that provide handwritten impls or data formats. They
// would need to disable default features and then explicitly re-enable std.
#[cfg(feature = "derive")]
#[allow(unused_imports)]
#[macro_use]
extern crate tauri_bindgen_abi_derive;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use tauri_bindgen_abi_derive::*;

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

#[macro_export(crate)]
macro_rules! ensure {
    ($cond:expr, $err:expr $(,)?) => {
        if !$cond {
            return ::std::result::Result::Err($err);
        }
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn struct_() -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Debug, PartialEq, crate::Writable, crate::Readable)]
        struct Foo {
            a: u8,
            b: u64,
            c: String,
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
        #[derive(Debug, PartialEq, crate::Writable, crate::Readable)]
        pub enum U1 {
            U32(u32),
            F32(f32),
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
          #[derive(crate::Writable, crate::Readable)]
          pub struct Flag4: u8 {
            const B0 = 1 << 0;
            const B1 = 1 << 1;
            const B2 = 1 << 2;
            const B3 = 1 << 3;
          }
        }

        let input = Flag4::B0 | Flag4::B2;

        let bytes = crate::to_bytes(&input)?;

        assert_eq!(bytes, vec![0b0000_0101]);

        let output: Flag4 = crate::from_slice(&bytes)?;

        assert_eq!(output, input);

        Ok(())
    }

    #[test]
    fn tuple() -> Result<(), Box<dyn std::error::Error>> {
        let input = (4u8, "foo".to_string());

        let bytes = crate::to_bytes(&input)?;

        assert_eq!(bytes, vec![4, 3, 0, 0, 0, 102, 111, 111]);

        let output: (u8, String) = crate::from_slice(&bytes)?;

        assert_eq!(output, input);

        Ok(())
    }
}
