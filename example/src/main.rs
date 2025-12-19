#![no_std]
#![no_main]

mod RefTopologyDictionary;

use core::fmt::{Display, Formatter};
use fprime_core::*;
pub use RefTopologyDictionary::*;

impl Display for fw::TimeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "{:?},{} {:05}.{:06}",
            self.time_base, self.time_context, self.seconds, self.useconds
        ))
    }
}

fn seq() -> FprimeResult<()> {
    cdh_core::cmd_disp::cmd_no_op();

    loop {
        let (val, time) = r#ref::system_resources::cpu()?;
        println!("[{}] {}", time, val);
        sys::sleep(1_000_000)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    seq().expect("sequence failed")
}
