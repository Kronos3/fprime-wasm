//! An argument whose type is an alias of a scalar.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.SEND_ALIAS(42);
}
