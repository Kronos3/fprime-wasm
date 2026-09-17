//! A structure with a string member, the other half of the nested-string case.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.SEND_COLOR_INFO(ColorInfoStruct {
        color: RED,
        name: StrTruncate::truncate("crimson"),
    });
}
