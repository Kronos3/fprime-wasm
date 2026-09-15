#![no_std]

mod abi;
mod cmd;
mod log;
mod serializable;
mod tlm;

pub use cmd::*;
pub use log::*;
pub use serializable::*;
pub use tlm::*;

pub use fprime_macros::Serializable;
pub use heapless;

pub struct FprimeEvents;
pub use core::fmt::Write;

#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum EventSeverity {
    WarningHi = 2,
    WarningLow = 3,
    ActivityHigh = 5,
    ActivityLo = 6,
    Diagnostic = 7,
}

/// Exit the runtime given a status.
/// This function should not return and should stop the WASM runtime
///
/// # Arguments
///
/// * `code`: Exit code signaling status
///
/// returns: ! Never returns
pub fn exit(code: i32) -> ! {
    unsafe { abi::exit(code) }
}

/// Pause the runtime for a specified time
///
/// # Arguments
///
/// * `us`: Time in microseconds to pause the runtime
///
/// returns: ()
pub fn rsleep(us: u64) {
    unsafe { abi::rsleep(us) }
}

/// Pause the runtime until a specified time
///
/// # Arguments
///
/// * `us`: Microseconds from system epoch to pause until
///
/// returns: ()
pub fn asleep(time: u64) {
    unsafe { abi::asleep(time) }
}

#[derive(Copy, Clone, Debug)]
#[repr(i32)]
pub enum PanicCode {
    /// Telemetry value is invalid
    TlmInvalid = 0,
    /// A command failed
    CmdFailed = 1,
    /// Code reserved for Rust global panic handler (abort)
    RustPanic = 2,
}

/// Exit the runtime due to a system/response failure
///
/// There are a fixed set of failure codes reserved. If the program needs to
/// exit with failure use [exit] with a non-zero status.
pub fn panic(code: PanicCode) -> ! {
    unsafe {
        abi::panic(code as i32);
    }
}
