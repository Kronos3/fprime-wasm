//! The deepest shape in the dictionary: a structure containing a nested array,
//! a nested structure and a member array.
#![no_std]
#![no_main]

use bench::*;

#[fprime_main]
pub fn main() {
    Ref.typeDemo.GLUTTON_OF_CHOICE(ChoiceSlurry {
        tooManyChoices: [[BLUE, RED], [TWO, TWO]],
        choiceAsMemberArray: [2, 3],
        choicePair: ChoicePair {
            firstChoice: RED,
            secondChoice: BLUE,
        },
        separateChoice: ONE,
    });
}
