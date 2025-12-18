#![no_std]

use core::fmt::{Display, Formatter};

mod dict;

#[repr(i32)]
pub enum CommandResponse {
    /// Command successfully executed
    Ok = 0,
    /// Invalid opcode dispatched
    InvalidOpcode = 1,
    /// Command failed validation
    ValidationError = 2,
    /// Command failed to deserialize
    FormatError = 3,
    /// Command had execution error
    ExecutionError = 4,
    /// Component busy
    Busy = 5,
}

#[repr(C)]
pub struct Time {
    seconds: Seconds,
    useconds: USeconds,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Seconds(u32);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct USeconds(u32);

impl Time {
    pub fn seconds(&self) -> Seconds {
        self.seconds
    }

    pub fn useconds(&self) -> USeconds {
        self.useconds
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        // TODO(tumbar) This is not correct, take a look
        f.write_fmt(format_args!("{}.{}", self.seconds.0, self.useconds.0))
    }
}
