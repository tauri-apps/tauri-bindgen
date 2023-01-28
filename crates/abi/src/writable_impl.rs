use super::{Error, Writable};
use std::{io, mem};

impl Writable for () {
    fn write_to(&self, _write: &mut impl io::Write) -> Result<(), Error> {
        Ok(())
    }

    fn size_hint(&self) -> usize {
        0
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

impl<'a, T: Writable> Writable for &'a [T] {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        self.len().write_to(write)?;

        for el in self.iter() {
            el.write_to(write)?;
        }

        Ok(())
    }

    fn size_hint(&self) -> usize {
        let mut size = self.len().size_hint();

        for el in self.iter() {
            size += el.size_hint();
        }

        size
    }
}

impl<T: Writable> Writable for Vec<T> {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        self.as_slice().write_to(write)
    }

    fn size_hint(&self) -> usize {
        self.as_slice().size_hint()
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

impl<'a> Writable for &'a str {
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

impl<'a, T: Writable> Writable for &'a T {
    fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
        (*self).write_to(write)
    }

    fn size_hint(&self) -> usize {
        (*self).size_hint()
    }
}

macro_rules! impl_for_tuple {
    ($($name:ident),+) => {
        impl <$($name: Writable),+> Writable for ($($name,)+) {
            fn write_to(&self, write: &mut impl io::Write) -> Result<(), Error> {
                #[allow(non_snake_case)]
                let &($(ref $name,)+) = self;
                $(
                    $name.write_to(write)?;
                )+
                Ok(())
            }

            fn size_hint(&self) -> usize {
                #[allow(non_snake_case)]
                let &($(ref $name,)+) = self;
                let mut size = 0;
                $(
                    size += $name.size_hint();
                )+
                size
            }
        }
    }
}

impl_for_tuple!( A0 );
impl_for_tuple!( A0, A1 );
impl_for_tuple!( A0, A1, A2 );
impl_for_tuple!( A0, A1, A2, A3 );
impl_for_tuple!( A0, A1, A2, A3, A4 );
impl_for_tuple!( A0, A1, A2, A3, A4, A5 );
impl_for_tuple!( A0, A1, A2, A3, A4, A5, A6 );
impl_for_tuple!( A0, A1, A2, A3, A4, A5, A6, A7 );
impl_for_tuple!( A0, A1, A2, A3, A4, A5, A6, A7, A8 );
impl_for_tuple!( A0, A1, A2, A3, A4, A5, A6, A7, A8, A9 );
impl_for_tuple!( A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10 );
impl_for_tuple!( A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11 );
impl_for_tuple!( A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12 );
impl_for_tuple!( A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13 );
impl_for_tuple!( A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14 );
impl_for_tuple!( A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15 );
impl_for_tuple!( A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11, A12, A13, A14, A15, A16 );
