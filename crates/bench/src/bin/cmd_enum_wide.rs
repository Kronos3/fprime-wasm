//! An enumeration wide enough to need an i64 on the wire.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.SEND_WIDE_CHOICE(WIDE_HIGH);
}
