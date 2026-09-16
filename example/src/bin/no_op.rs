#![no_std]
#![no_main]

use example::*;
use fprime_core::*;

#[fprime_main]
pub fn main() {
    set_fail_mode(FailMode::Checked);

    CdhCore.cmdDisp.CMD_NO_OP();
    CdhCore.cmdDisp.CMD_NO_OP_STRING("hello from no_op.wasm");
}
