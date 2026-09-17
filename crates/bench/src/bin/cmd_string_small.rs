//! A `string size 40` argument.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    CdhCore.cmdDisp.CMD_NO_OP_STRING("a small string argument");
}
