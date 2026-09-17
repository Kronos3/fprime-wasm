//! An array of scalars.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.SEND_U32_ARRAY([1, 2, 3, 4, 5]);
}
