//! Eight const-encoded calls that differ, so nothing can be merged. Read against
//! `dedup_same_cmd` to see what deduplication is worth.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);
    Ref.dpDemo.Dp(IMMEDIATE, 1, PROC_TYPE_NONE);
    Ref.dpDemo.Dp(IMMEDIATE, 2, PROC_TYPE_NONE);
    Ref.dpDemo.Dp(IMMEDIATE, 3, PROC_TYPE_NONE);
    Ref.dpDemo.Dp(IMMEDIATE, 4, PROC_TYPE_NONE);
    Ref.dpDemo.Dp(IMMEDIATE, 5, PROC_TYPE_NONE);
    Ref.dpDemo.Dp(IMMEDIATE, 6, PROC_TYPE_NONE);
    Ref.dpDemo.Dp(IMMEDIATE, 7, PROC_TYPE_NONE);
}
