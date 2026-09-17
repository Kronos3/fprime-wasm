//! An array of booleans.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.SEND_BOOL_ARRAY([true, false]);
}
