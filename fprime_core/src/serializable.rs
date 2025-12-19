use crate::{Serializable, String};

impl Serializable for u8 {
    const SIZE: usize = 1;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        to[*offset] = *self;
        *offset += 1;
    }

    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
        let out = from[*offset];
        *offset += 1;
        out
    }
}

impl Serializable for u16 {
    const SIZE: usize = 2;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..*offset + Self::SIZE].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }

    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
        let out = Self::from_be_bytes(from[*offset..*offset + Self::SIZE].try_into().unwrap());
        *offset += Self::SIZE;
        out
    }
}

impl Serializable for u32 {
    const SIZE: usize = 4;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..*offset + Self::SIZE].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }

    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
        let out = Self::from_be_bytes(from[*offset..*offset + Self::SIZE].try_into().unwrap());
        *offset += Self::SIZE;
        out
    }
}

impl Serializable for u64 {
    const SIZE: usize = 8;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..*offset + Self::SIZE].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }

    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
        let out = Self::from_be_bytes(from[*offset..*offset + Self::SIZE].try_into().unwrap());
        *offset += Self::SIZE;
        out
    }
}

impl Serializable for i8 {
    const SIZE: usize = 1;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        to[*offset] = *self as u8;
        *offset += 1;
    }

    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
        let out = from[*offset] as i8;
        *offset += 1;
        out
    }
}

impl Serializable for i16 {
    const SIZE: usize = 2;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..*offset + Self::SIZE].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }

    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
        let out = Self::from_be_bytes(from[*offset..*offset + Self::SIZE].try_into().unwrap());
        *offset += Self::SIZE;
        out
    }
}

impl Serializable for i32 {
    const SIZE: usize = 4;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..*offset + Self::SIZE].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }

    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
        let out = Self::from_be_bytes(from[*offset..*offset + Self::SIZE].try_into().unwrap());
        *offset += Self::SIZE;
        out
    }
}

impl Serializable for i64 {
    const SIZE: usize = 8;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..*offset + Self::SIZE].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }

    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
        let out = Self::from_be_bytes(from[*offset..*offset + Self::SIZE].try_into().unwrap());
        *offset += Self::SIZE;
        out
    }
}

impl Serializable for f32 {
    const SIZE: usize = 4;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..*offset + Self::SIZE].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }

    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
        let out = Self::from_be_bytes(from[*offset..*offset + Self::SIZE].try_into().unwrap());
        *offset += Self::SIZE;
        out
    }
}

impl Serializable for f64 {
    const SIZE: usize = 8;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..*offset + Self::SIZE].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }

    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self {
        let out = Self::from_be_bytes(from[*offset..*offset + Self::SIZE].try_into().unwrap());
        *offset += Self::SIZE;
        out
    }
}

impl<const N: usize> Serializable for crate::String<N> {
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
