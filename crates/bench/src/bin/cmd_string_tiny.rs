//! A `string size 8` argument.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.SEND_TINY_STRING("tiny");
}
