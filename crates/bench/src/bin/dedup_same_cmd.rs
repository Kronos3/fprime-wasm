//! The same const-encoded command eight times. The buffer should be merged to a
//! single address, so this should cost barely more than one call.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
}
