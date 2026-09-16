#[allow(nonstandard_style)]
#[allow(dead_code)]
mod inner {
    include!(concat!(env!("OUT_DIR"), "/dictionary.rs"));
}

use fprime_core::Serializable;
pub use inner::Defs::*;
pub use inner::*;

impl Fw::TimeValue {
    #[allow(dead_code)]
    pub fn now() -> Defs::Fw::TimeValue {
        let mut buf: [u8; Defs::Fw::TimeValue::SIZE] = [0; Defs::Fw::TimeValue::SIZE];

        unsafe {
            fprime_core::time::time_read(&mut buf);
        }

        Defs::Fw::TimeValue::deserialize(&buf)
    }
}
