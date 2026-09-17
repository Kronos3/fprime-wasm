//! An array of floats.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.SEND_FLOAT_ARRAY([1.0, 2.5, -3.5]);
}
