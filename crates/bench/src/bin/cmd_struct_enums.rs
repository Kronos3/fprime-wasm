//! A structure of enumerations.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.CHOICE_PAIR(ChoicePair {
        firstChoice: RED,
        secondChoice: BLUE,
    });
}
