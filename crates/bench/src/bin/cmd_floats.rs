//! Both float widths.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.SEND_FLOATS(1.5, -2.5);
}
