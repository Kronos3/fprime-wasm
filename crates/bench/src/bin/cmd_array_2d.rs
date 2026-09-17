//! An array of arrays.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.SEND_2D([[ONE, TWO], [TWO, ONE]]);
}
