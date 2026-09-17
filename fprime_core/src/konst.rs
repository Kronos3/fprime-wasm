//! Const-evaluable serialization primitives.

pub const fn put_u8(to: &mut [u8], offset: usize, value: u8) -> usize {
    to[offset] = value;
    offset + 1
}

pub const fn put_i8(to: &mut [u8], offset: usize, value: i8) -> usize {
    put_u8(to, offset, value as u8)
}

macro_rules! put_be {
    ($name:ident, $ty:ty) => {
        /// Big-endian `
        #[doc = stringify!($ty)]
        ///` at `offset`, returning the offset past it.
        pub const fn $name(to: &mut [u8], offset: usize, value: $ty) -> usize {
            let bytes = value.to_be_bytes();

            let mut i = 0;
            while i < bytes.len() {
                to[offset + i] = bytes[i];
                i += 1;
            }

            offset + bytes.len()
        }
    };
}

put_be!(put_u16, u16);
put_be!(put_i16, i16);
put_be!(put_u32, u32);
put_be!(put_i32, i32);
put_be!(put_u64, u64);
put_be!(put_i64, i64);
put_be!(put_f32, f32);
put_be!(put_f64, f64);

/// The length of `s` in a `string size cap` field: the `u16` length prefix
/// plus the bytes that survive truncation.
pub const fn str_len(s: &str, cap: usize) -> usize {
    let n = s.len();

    2 + if n < cap { n } else { cap }
}

/// A `string size cap` field at `offset`, returning the offset past it.
pub const fn put_str(to: &mut [u8], offset: usize, s: &str, cap: usize) -> usize {
    let bytes = s.as_bytes();
    let n = if bytes.len() < cap { bytes.len() } else { cap };

    let mut offset = put_u16(to, offset, n as u16);

    let mut i = 0;
    while i < n {
        to[offset + i] = bytes[i];
        i += 1;
    }

    offset += n;
    offset
}
