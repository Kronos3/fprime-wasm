#![no_std]
#![no_main]

mod dictionary;

use core::fmt::{Display, Formatter};
pub use dictionary::*;
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
    loop {
        let (val, time) = unsafe { r#ref::system_resources::cpu().unwrap_unchecked() };
        println!("[{}] {}", time, val);
        sys::sleep(1_000_000)
    }
}
