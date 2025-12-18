#![no_std]
#![no_main]

mod RefTopologyDictionary;

pub use RefTopologyDictionary::*;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    cdh_core::cmd_disp::cmd_no_op();
    cdh_core::cmd_disp::cmd_no_op();
    cdh_core::cmd_disp::cmd_no_op();
}
