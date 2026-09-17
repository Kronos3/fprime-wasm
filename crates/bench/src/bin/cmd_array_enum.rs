//! An array of enumerations.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.CHOICES([ONE, TWO]);
}
