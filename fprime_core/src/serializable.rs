use crate::String;

pub trait Serializable: Sized {
    const SIZE: usize;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize);
    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self;
    fn deserialize(from: &[u8]) -> Self {
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
                let Some(dst) = to.get_mut(*offset..*offset + Self::SIZE) else {
                    crate::panic(crate::PanicCode::Truncated)
                };

                dst.copy_from_slice(&bytes);
                *offset += Self::SIZE;
            }

            fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
                let Some(bytes) = from.get(*offset..*offset + Self::SIZE) else {
                    crate::panic(crate::PanicCode::Truncated)
                };

                let Ok(bytes) = <[u8; size_of::<$primitive>()]>::try_from(bytes) else {
                    crate::panic(crate::PanicCode::Truncated)
                };
                *offset += Self::SIZE;
                Self::from_be_bytes(bytes)
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

    #[inline(always)]
    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.as_bytes();
        let n = bytes.len();
        (n as u16).serialize_to(to, offset);
        let Some(dst) = to.get_mut(*offset..*offset + n) else {
            crate::panic(crate::PanicCode::Truncated)
        };

        dst.copy_from_slice(bytes);
        *offset += n;
    }

    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
        let n = u16::deserialize_from(from, offset) as usize;

        if n > N {
            crate::panic(crate::PanicCode::Truncated);
        }

        let Some(src) = from.get(*offset..*offset + n) else {
            crate::panic(crate::PanicCode::Truncated)
        };

        let mut out: heapless::Vec<u8, N, u16> = heapless::Vec::new();
        unsafe {
            core::ptr::copy_nonoverlapping(src.as_ptr(), out.as_mut_ptr(), n);
            out.set_len(n);
        }

        *offset += n;

        // The host serializes an Fw::String, so these bytes are already valid UTF-8.
        // Validating pulls in `core::str` validation machinery, so only pay for it
        // in debug builds.
        if cfg!(debug_assertions) && core::str::from_utf8(&out).is_err() {
            crate::panic(crate::PanicCode::InvalidStatus);
        }

        unsafe { String::from_utf8_unchecked(out) }
    }
}

/// Serialize a `&str` into a `String<N>` wire field, truncated to `N` bytes.
#[inline(always)]
pub fn serialize_str<const N: usize>(s: &str, to: &mut [u8], offset: &mut usize) {
    let bytes = s.as_bytes();
    let n = core::cmp::min(bytes.len(), N);

    (n as u16).serialize_to(to, offset);

    let Some(dst) = to.get_mut(*offset..*offset + n) else {
        crate::panic(crate::PanicCode::Truncated)
    };

    dst.copy_from_slice(&bytes[..n]);
    *offset += n;
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
