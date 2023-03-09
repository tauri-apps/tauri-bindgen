use super::{ensure, Error, Readable};
use std::fmt::Debug;
use std::io::{self, Read};

fn take_const<'a, const N: usize>(bytes: &mut &'a [u8]) -> Option<&'a [u8; N]> {
    if N > bytes.len() {
        return None;
    }
    let (first, rem) = bytes.split_at(N);
    *bytes = rem;

    Some(unsafe { &*(first.as_ptr() as *const [u8; N]) })
}

fn take<'a>(bytes: &mut &'a [u8], n: usize) -> Option<&'a [u8]> {
    if n > bytes.len() {
        return None;
    }
    let (first, rem) = bytes.split_at(n);
    *bytes = rem;

    Some(first)
}

impl Readable for () {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        Ok(&())
    }
}

impl Readable for u8 {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        let slice = take_const::<1>(bytes).ok_or(Error::UnexpectedEof)?;
        Ok(&slice[0])
    }
}

impl Readable for u16 {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        let slice = take_const(bytes).ok_or(Error::UnexpectedEof)?;
        Ok(&u16::from_le_bytes(*slice))
    }
}

impl Readable for u32 {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        let slice = take_const(bytes).ok_or(Error::UnexpectedEof)?;
        Ok(&u32::from_le_bytes(*slice))
    }
}

impl Readable for u64 {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        let slice = take_const(bytes).ok_or(Error::UnexpectedEof)?;
        Ok(&u64::from_le_bytes(*slice))
    }
}

// impl Readable for u128 {
//     fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
//         let mut value: MaybeUninit<u128> = MaybeUninit::uninit();
//         unsafe {
//             let bytes_read = read.read(std::slice::from_raw_parts_mut(
//                 value.as_mut_ptr().cast::<u8>(),
//                 16,
//             ))?;

//             ensure!(bytes_read == 16, Error::UnexpectedEof);

//             Ok(value.assume_init())
//         }
//     }
// }

// impl Readable for usize {
//     fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
//         let mut value: MaybeUninit<u32> = MaybeUninit::uninit();
//         unsafe {
//             let bytes_read = read.read(std::slice::from_raw_parts_mut(
//                 value.as_mut_ptr().cast::<u8>(),
//                 4,
//             ))?;

//             ensure!(bytes_read == 4, Error::UnexpectedEof);

//             Ok(value.assume_init() as usize)
//         }
//     }
// }

impl Readable for i8 {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        let slice = take_const::<1>(bytes).ok_or(Error::UnexpectedEof)?;
        Ok(&(slice[0] as i8))
    }
}

impl Readable for i16 {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        let slice = take_const(bytes).ok_or(Error::UnexpectedEof)?;
        Ok(&i16::from_le_bytes(*slice))
    }
}

impl Readable for i32 {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        let slice = take_const(bytes).ok_or(Error::UnexpectedEof)?;
        Ok(&i32::from_le_bytes(*slice))
    }
}

impl Readable for i64 {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        let slice = take_const(bytes).ok_or(Error::UnexpectedEof)?;
        Ok(&i64::from_le_bytes(*slice))
    }
}

// impl Readable for i128 {
//     fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
//         let mut value: MaybeUninit<i128> = MaybeUninit::uninit();
//         unsafe {
//             let bytes_read = read.read(std::slice::from_raw_parts_mut(
//                 value.as_mut_ptr().cast::<u8>(),
//                 16,
//             ))?;

//             ensure!(bytes_read == 16, Error::UnexpectedEof);

//             Ok(value.assume_init())
//         }
//     }
// }

// impl Readable for isize {
//     fn read_from(read: &mut impl io::Read) -> Result<Self, Error> {
//         let mut value: MaybeUninit<isize> = MaybeUninit::uninit();
//         unsafe {
//             let bytes_read = read.read(std::slice::from_raw_parts_mut(
//                 value.as_mut_ptr().cast::<u8>(),
//                 4,
//             ))?;

//             ensure!(bytes_read == 4, Error::UnexpectedEof);

//             Ok(value.assume_init())
//         }
//     }
// }

impl Readable for f32 {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        let slice = take_const(bytes).ok_or(Error::UnexpectedEof)?;
        Ok(&f32::from_le_bytes(*slice))
    }
}

impl Readable for f64 {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        let slice = take_const(bytes).ok_or(Error::UnexpectedEof)?;
        Ok(&f64::from_le_bytes(*slice))
    }
}

impl Readable for char {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        let slice = take_const(bytes).ok_or(Error::UnexpectedEof)?;
        Ok(&char::from_u32(u32::from_le_bytes(*slice)).ok_or(Error::InvalidChar)?)
    }
}

impl Readable for bool {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        let slice = take_const::<1>(bytes).ok_or(Error::UnexpectedEof)?;

        Ok(&(slice[0] != 0))
    }
}

// impl<T: Readable> Readable for Vec<&T> {
//     fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
//         let length = u32::read_from(bytes)?;

//         let value = lift_errors((0..length).map(|_| T::read_from(bytes)))?.collect();

//         Ok(value)
//     }
// }

impl Readable for String {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        let length = *u32::read_from(bytes)?;

        let mut value = String::with_capacity(length as usize);
        take(bytes, length as usize)
            .ok_or(Error::UnexpectedEof)?
            .read_to_string(&mut value)?;

        Ok(&value)
    }
}

impl Readable for str {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        let length = *u32::read_from(bytes)?;

        let slice = take(bytes, length as usize).ok_or(Error::UnexpectedEof)?;

        std::str::from_utf8(slice).map_err(Into::into)
    }
}

impl<T: Readable> Readable for Option<&T> {
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
        let tag = u8::read_from(bytes)?;

        match tag {
            0 => Ok(&None),
            1 => Ok(&Some(T::read_from(bytes)?)),
            _ => panic!(),
        }
    }
}

impl<T: Readable, E: Readable> Readable for Result<T, E> {
    type Out<'a> = Result<T, E> where E: 'a, T: 'a;
    
    fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<Self::Out<'a>, Error> {
        let tag = u8::read_from(bytes)?;

        match tag {
            0 => Ok(Ok(T::read_from(bytes)?)),
            1 => Ok(Err(E::read_from(bytes)?)),
            _ => panic!(),
        }
    }
}

// macro_rules! impl_for_tuple {
//     ($($name:ident),+) => {
//         impl <$($name: Readable),+> Readable for ($($name,)+) {
//             fn read_from<'a>(bytes: &mut &'a [u8]) -> Result<&'a Self, Error> {
//                 $(
//                     #[allow(non_snake_case)]
//                     let $name = $name::read_from(bytes)?;
//                 )+

//                 Ok( ($($name,)+) )
//             }
//         }
//     }
// }

// impl_for_tuple!(A0);
// impl_for_tuple!(A0, A1);
// impl_for_tuple!(A0, A1, A2);
// impl_for_tuple!(A0, A1, A2, A3);
// impl_for_tuple!(A0, A1, A2, A3, A4);
// impl_for_tuple!(A0, A1, A2, A3, A4, A5);
// impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6);
// impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7);
// impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8);
// impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9);
// impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);
// impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11);
// impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12);
// impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13);
// impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14);
// impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15);
// impl_for_tuple!(A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16);

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
