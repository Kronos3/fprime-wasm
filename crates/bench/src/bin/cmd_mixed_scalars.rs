//! A signed, a float and an unsigned argument together.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    CdhCore.cmdDisp.CMD_TEST_CMD_1(-1, -2.5, 3);
}
