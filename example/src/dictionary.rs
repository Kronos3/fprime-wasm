include!(concat!(env!("OUT_DIR"), "/dictionary.rs"));

impl Defs::Fw::CmdResponse {
    pub fn check(&self) {
        match self {
            Defs::Fw::CmdResponse::OK => {}
            _ => fprime_core::panic(fprime_core::PanicCode::CmdFailed),
        }
    }
}
