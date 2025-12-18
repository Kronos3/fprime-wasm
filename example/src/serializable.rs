pub trait Serializable {
    const SIZE: usize;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize);
}

impl Serializable for u8 {
    const SIZE: usize = 1;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        to[*offset] = *self;
        *offset += 1;
    }
}

impl Serializable for u16 {
    const SIZE: usize = 2;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }
}

impl Serializable for u32 {
    const SIZE: usize = 4;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }
}

impl Serializable for u64 {
    const SIZE: usize = 8;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }
}

impl Serializable for i8 {
    const SIZE: usize = 1;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        to[*offset] = *self as u8;
        *offset += 1;
    }
}

impl Serializable for i16 {
    const SIZE: usize = 2;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }
}

impl Serializable for i32 {
    const SIZE: usize = 4;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }
}

impl Serializable for i64 {
    const SIZE: usize = 8;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }
}

impl Serializable for f32 {
    const SIZE: usize = 4;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }
}

impl Serializable for f64 {
    const SIZE: usize = 8;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let bytes = self.to_be_bytes();
        to[*offset..].copy_from_slice(&bytes);
        *offset += Self::SIZE;
    }
}

impl<const N: usize> Serializable for crate::String<N> {
    const SIZE: usize = 2 + N;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize) {
        let n = self.len();
        (n as u16).serialize_to(to, offset);
        to[*offset..].copy_from_slice(self.as_bytes());
        *offset += n;
    }
}
