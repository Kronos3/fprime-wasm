//! A string longer than its wire capacity, so truncation is what lands in the
//! const buffer.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.SEND_TINY_STRING("far longer than eight bytes");
}
