//! A handful of shapes together, the size of a small sequence.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    CdhCore.cmdDisp.CMD_NO_OP();
    CdhCore.cmdDisp.CMD_NO_OP_STRING("starting");
    Ref.dpDemo.SelectColor(BLUE);
    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
}
