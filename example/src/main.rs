#![no_std]
#![no_main]

mod RefTopologyDictionary;

pub use RefTopologyDictionary::*;

fn entry() {
    cdh_core::cmd_disp::cmd_no_op();
    cdh_core::cmd_disp::cmd_no_op();
    cdh_core::cmd_disp::cmd_no_op();
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    entry();
}
