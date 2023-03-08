use super::{ensure, Error, Readable};
use std::fmt::Debug;
use std::{
    io::{self, Read},
    mem::MaybeUninit,
};

impl Readable for () {
    fn read_from(_read: &mut impl io::Read) -> Result<Self, Error> {
        Ok(())
    }
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

        let value = lift_errors((0..length).map(|_| T::read_from(read)))?.collect();

        Ok(value)
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

macro_rules! impl_for_tuple {
    ($($name:ident),+) => {
        impl <$($name: Readable),+> Readable for ($($name,)+) {
            fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
                $(
                    #[allow(non_snake_case)]
                    let $name = $name::read_from(read)?;
                )+

                Ok( ($($name,)+) )
            }
        }
    }
}

impl_for_tuple!(A0);
impl_for_tuple!(A0, A1);
impl_for_tuple!(A0, A1, A2);
impl_for_tuple!(A0, A1, A2, A3);
impl_for_tuple!(A0, A1, A2, A3, A4);
impl_for_tuple!(A0, A1, A2, A3, A4, A5);
impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6);
impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7);
impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8);
impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11);
impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12);
impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13);
impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14);
impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15);
impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16);

fn lift_errors<T, E>(
    iter: impl Iterator<Item = Result<T, E>>,
) -> Result<impl Iterator<Item = T>, Error>
where
    E: Into<Error> + Debug,
{
    let (types, errors): (Vec<_>, Vec<_>) = iter.partition(Result::is_ok);

    let errors: Vec<_> = errors
        .into_iter()
        // we use unwrap_err_unchecked() here so we don't have to impose `T: Debug`
        .map(|err| unsafe { err.unwrap_err_unchecked().into() })
        .collect();
    if !errors.is_empty() {
        return Err(Error::Multiple(errors));
    }

    Ok(types.into_iter().map(Result::unwrap))
}
