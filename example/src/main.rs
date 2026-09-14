#![no_std]
#![no_main]

mod dictionary;
pub use dictionary::*;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    cdh_core::cmd_disp::cmd_no_op_string("Hello");

    CdhCore.cmd_no_op();

    10i32.pow(10);

}
