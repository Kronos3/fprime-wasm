//! A boolean argument.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.SEND_BOOL(true);
}
