//! An array written as a repeat rather than element by element.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.CHOICES([ONE; 2]);
}
