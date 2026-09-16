#[allow(nonstandard_style)]
#[allow(dead_code)]
mod inner {
    include!(concat!(env!("OUT_DIR"), "/dictionary.rs"));
}

#[allow(unused_imports)]
pub use inner::Defs::*;
pub use inner::*;
