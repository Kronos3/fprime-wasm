//! A `string size 240` argument.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.wasmSeq.LOAD("a large string argument");
}
