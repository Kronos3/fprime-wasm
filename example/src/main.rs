#![no_std]
#![no_main]

mod dictionary;

use core::fmt::{Display, Formatter};
pub use dictionary::r#ref::*;
pub use dictionary::*;
use fprime_core::sys::sleep;
use fprime_core::*;

impl Display for fw::TimeValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "{:?},{} {:05}.{:06}",
            self.time_base, self.time_context, self.seconds, self.useconds
        ))
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    cdh_core::cmd_disp::cmd_no_op_string("Hello");
    cdh_core::cmd_disp::cmd_no_op();
    cdh_core::cmd_disp::cmd_no_op();
    cdh_core::cmd_disp::cmd_no_op();
    cdh_core::cmd_disp::cmd_no_op();
    cdh_core::cmd_disp::cmd_no_op();

    // Poll the CPU telemetry every second
    for _ in 0..10 {
        let (val, time) = system_resources::cpu().unwrap();
        // print_event!("CPU {time} {}", val as u8);

        // Sleep 1 second
        sleep(1_000_000)
    }
}
