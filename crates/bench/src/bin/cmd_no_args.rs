//! A command with no arguments: opcode only.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    CdhCore.cmdDisp.CMD_NO_OP();
}
