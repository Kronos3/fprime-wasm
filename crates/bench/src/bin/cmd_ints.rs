//! Every integer width as a separate argument.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.SEND_INTS(1, -2, 3, -4, 5, -6, 7, -8);
}
