#![no_std]
#![no_main]

mod RefTopologyDictionary;

use fprime_core::*;
pub use RefTopologyDictionary::*;

fn seq() -> FprimeResult<()> {
    cdh_core::cmd_disp::cmd_no_op();
    r#ref::system_resources::cpu()?;

    Ok(())
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    cdh_core::cmd_disp::cmd_no_op();
    cdh_core::cmd_disp::cmd_no_op();
    cdh_core::cmd_disp::cmd_no_op();
}
