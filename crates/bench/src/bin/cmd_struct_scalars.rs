//! A structure holding every scalar width.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.SEND_SCALARS(ScalarStruct {
        i8: -1,
        i16: -2,
        i32: -3,
        i64: -4,
        u8: 5,
        u16: 6,
        u32: 7,
        u64: 8,
        f32: 9.5,
        f64: 10.5,
    });
}
