#[allow(nonstandard_style)]
#[allow(dead_code)]
mod inner {
    include!(concat!(env!("OUT_DIR"), "/dictionary.rs"));
}

pub use inner::*;

impl Defs::Fw::CmdResponse {
    #[allow(dead_code)]
    pub fn check(&self) {
        match self {
            Defs::Fw::CmdResponse::OK => {}
            _ => fprime_core::panic(fprime_core::PanicCode::CmdFailed),
        }
    }
}
