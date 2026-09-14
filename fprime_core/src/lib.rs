#![no_std]

mod internal;
mod serializable;

pub use fprime_macros::Serializable;
pub use serializable::*;

pub use heapless;

/// Stack-backed String type
/// The capacity specifies the
pub type String<const N: usize> = heapless::String<N, u16>;

pub trait StrTruncate<const N: usize> {
    fn truncate(s: &str) -> Self;
}

impl<const N: usize> StrTruncate<N> for String<N> {
    fn truncate(s: &str) -> String<N> {
        let mut out: heapless::Vec<u8, N, u16> = heapless::Vec::new();
        let n = core::cmp::min(s.len(), N);
        unsafe {
            for (i, c) in s.get_unchecked(..n).as_bytes().iter().enumerate() {
                *out.get_unchecked_mut(i) = *c;
            }
            out.set_len(n);
            heapless::String::from_utf8_unchecked(out)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum FprimeErr {
    /// Telemetry value is invalid
    InvalidTelemetry,
}

pub type FprimeResult<T> = Result<T, FprimeErr>;

pub struct FprimeEvents;
pub use core::fmt::Write;

#[repr(i32)]
pub enum EventSeverity {
    Fatal = 1,
    WarningHi = 2,
    WarningLow = 3,
    Command = 4,
    ActivityHigh = 5,
    ActivityLo = 6,
    Diagnostic = 7,
}

#[macro_export]
macro_rules! print_event {
    ($sev:expr, $($arg:tt)+) => {
        fprime_core::sys::messagef($sev, format_args!($($arg)+));
    };
}

#[macro_export]
macro_rules! format {
    ($size:expr, $($arg:tt)+) => {
        fprime_core::heapless::string::format::<$size, u16>(format_args!($($arg)+)).unwrap()
    };
}

pub mod sys {
    use crate::{FprimeErr, FprimeResult, internal};
    use core::fmt::Arguments;

    /// Dispatch a command given a Fw::ComBuffer
    /// This command should be run synchronously and return the response
    /// once the command has finished.
    ///
    /// Note: This function should not be explicit called as an incorrect buffer
    /// will result in a format response from the F Prime command dispatcher.
    /// The safe auto-coded dictionary should be used instead of this.
    ///
    /// The com_buffer should be formatted with this struct:
    /// struct CmdComBuffer {
    ///     opcode: FwOpcodeType,
    ///     args: []args
    /// }
    ///
    /// # Arguments
    ///
    /// * `com_buffer`: Fw::ComBuffer encoded F Prime command
    ///
    /// returns: i32 (Fw::CmdResponse)
    pub unsafe fn command(com_buffer: &[u8]) -> i32 {
        unsafe { internal::cmd(com_buffer.as_ptr() as u32, com_buffer.len() as u32) }
    }

    /// Request last reported telemetry value
    ///
    /// # Arguments
    ///
    /// * `id`: Telemetry ID
    /// * `time`: Buffer for holding last write time of telemetry
    /// * `value`: Buffer for holding the telemetry value
    ///
    /// returns: FprimeResult noting if the telemetry is valid or not
    pub unsafe fn telemetry(
        id: u32,
        time_buf: &mut [u8],
        value_buf: &mut [u8],
    ) -> FprimeResult<()> {
        match unsafe {
            internal::tlm(
                id,
                time_buf.as_ptr() as u32,
                time_buf.len() as u32,
                value_buf.as_ptr() as u32,
                value_buf.len() as u32,
            )
        } {
            0 => Ok(()),
            _ => Err(FprimeErr::InvalidTelemetry),
        }
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
        unsafe { internal::exit(code) }
    }

    /// Emit a message via the F Prime event system.
    /// The message should be serialized into a UTF-8 buffer
    ///
    /// # Arguments
    ///
    /// * `msg`: message string to emit via F Prime event
    ///
    /// returns: ()
    pub fn message(severity: crate::EventSeverity, msg: &str) {
        let ptr = msg.as_ptr() as u32;
        let len = msg.len() as u32;
        unsafe { internal::event(severity as i32, ptr, len) }
    }

    #[inline]
    pub fn messagef(severity: crate::EventSeverity, args: Arguments<'_>) {
        let s: crate::String<120> = heapless::string::format(args).unwrap();
        message(severity, &s)
    }

    /// Pause the runtime for a specified time
    ///
    /// # Arguments
    ///
    /// * `us`: Time in microseconds to pause the runtime
    ///
    /// returns: ()
    pub fn rsleep(us: u64) {
        unsafe { internal::rsleep(us) }
    }

    /// Pause the runtime until a specified time
    ///
    /// # Arguments
    ///
    /// * `us`: Microseconds from system epoch to pause until
    ///
    /// returns: ()
    pub fn asleep(time: u64) {
        unsafe { internal::asleep(time) }
    }

    #[cfg(target_arch = "wasm32")]
    #[cfg(not(test))]
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        unsafe {
            internal::panic(1);
        }
    }
}
