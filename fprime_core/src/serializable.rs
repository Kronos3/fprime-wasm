use crate::{String};

pub trait Serializable: Sized {
    const SIZE: usize;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize);
    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self;

    fn deserialize<const SIZE: usize>(from: [u8; SIZE]) -> Self {
        // Rust doesn't support complex generic evaluation statically
        // https://github.com/rust-lang/rust/issues/76560
        assert_eq!(Self::SIZE, SIZE);

        let mut offset: usize = 0;
        Self::deserialize_from(&from, &mut offset)
    }
}

macro_rules! primitive {
    ($primitive: ty) => {
        impl Serializable for $primitive {
            const SIZE: usize = size_of::<$primitive>();

            fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
                let bytes = self.to_be_bytes();
                to[*offset..*offset + Self::SIZE].copy_from_slice(&bytes);
                *offset += Self::SIZE;
            }

            fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
                let out =
                    Self::from_be_bytes(from[*offset..*offset + Self::SIZE].try_into().unwrap());
                *offset += Self::SIZE;
                out
            }
        }
    };
}

primitive!(u8);
primitive!(i8);
primitive!(u16);
primitive!(i16);
primitive!(u32);
primitive!(i32);
primitive!(u64);
primitive!(i64);
primitive!(f32);
primitive!(f64);

impl<const N: usize> Serializable for String<N> {
    const SIZE: usize = 2 + N;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.as_bytes();
        let n = bytes.len();
        (n as u16).serialize_to(to, offset);
        to[*offset..*offset + n].copy_from_slice(bytes);
        *offset += n;
    }

    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
        let n = u16::deserialize_from(from, offset) as usize;
        let out =
            String::from_utf8(heapless::Vec::from_slice(&from[*offset..*offset + n]).unwrap())
                .unwrap();
        *offset += n;
        out
    }
}

impl<T: Serializable, const N: usize> Serializable for [T; N] {
    const SIZE: usize = T::SIZE * N;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        for i in self {
            i.serialize_to(to, offset);
        }
    }

    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
        let mut out: [T; N] = unsafe {
            #[allow(invalid_value)]
            core::mem::MaybeUninit::uninit().assume_init()
        };

        for i in 0..N {
            out[i] = T::deserialize_from(from, offset);
        }

        out
    }
}

impl Serializable for bool {
    const SIZE: usize = 1;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let val: u8 = if *self { 1 } else { 0 };
        val.serialize_to(to, offset);
    }

    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
        if u8::deserialize_from(from, offset) != 0 {
            true
        } else {
            false
        }
    }
}
