#![no_std]

mod serializable;

pub use fprime_macros::Serializable;

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

pub trait Serializable: Sized {
    const SIZE: usize;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize);
    fn deserialize_from(from: &[u8], offset: &mut usize) -> Self;

    fn deserialize<const SIZE: usize>(from: [u8; SIZE]) -> Self {
        // Rust doesn't support complex generic evaluation statically
        // https://github.com/rust-lang/rust/issues/76560
        assert_eq!(Self::SIZE, SIZE);

        let mut offset: usize = 0;
        Self::deserialize_from(&from, &mut offset)
    }
}

mod internal {
    #[link(wasm_import_module = "fprime_core")]
    unsafe extern "C" {

        /// Dispatch a command given a Fw::ComBuffer
        /// This command should be run synchronously and return the response
        /// once the command has finished
        ///
        /// The com_buffer should be formatted with this struct:
        /// struct CmdComBuffer {
        ///     opcode: FwOpcodeType,
        ///     args: []args
        /// }
        ///
        /// # Arguments
        ///
        /// * `com_buffer_ptr`: Pointer to the com buffer data
        /// * `size`: Length of the com buffer
        ///
        /// returns: i32 (Fw::CmdResponse)
        pub(crate) fn command(com_buffer_ptr: u32, size: u32) -> i32;

        /// Request last reported telemetry value
        ///
        /// # Arguments
        ///
        /// * `id`: Telemetry ID
        /// * `time_ptr`: Pointer to the writable time buffer
        /// * `time_size`: Length of the time buffer
        /// * `value_ptr`: Pointer to the writable value buffer
        /// * `value_size`: Length of the value buffer
        ///
        /// returns: i32 (Fw::TlmValid)
        pub(crate) fn telemetry(
            id: u32,
            time_ptr: u32,
            time_size: u32,
            value_ptr: u32,
            value_size: u32,
        ) -> i32;

        /// Emit a message via the F Prime event system.
        /// The message should be serialized into a UTF-8 buffer
        ///
        /// # Arguments
        ///
        /// * `str_ptr`: pointer to the message string
        /// * `size`: length of the message string
        ///
        pub(crate) fn message(str_ptr: u32, size: u32);

        /// Exit the runtime given a status.
        /// This function should not return and should stop the WASM runtime
        ///
        /// # Arguments
        ///
        /// * `code`: Exit code signaling status
        ///
        /// returns: ! Never returns
        pub(crate) fn exit(code: i32) -> !;

        /// Pause the runtime for a specified time
        ///
        /// # Arguments
        ///
        /// * `us`: Time in microseconds to pause the runtime
        ///
        /// returns: ()
        pub(crate) fn rsleep(us: u64);

        /// Panic the runtime with a message.
        /// This function should not return and the runtime should be stopped
        ///
        /// # Arguments
        ///
        /// * `msg_ptr`: Pointer to the panic string
        /// * `size`: length of the panic string
        #[cfg(target_arch = "wasm32")]
        #[cfg(not(test))]
        pub(crate) fn panic(msg_ptr: u32, size: u32) -> !;
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

impl Write for FprimeEvents {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        sys::message(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        sys::messagef(core::format_args!($($arg)*));
    };
}

pub mod sys {
    use crate::{internal, FprimeErr, FprimeResult};
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
        unsafe { internal::command(com_buffer.as_ptr() as u32, com_buffer.len() as u32) }
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
            internal::telemetry(
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
    /// ```
    pub fn message(msg: &str) {
        let ptr = msg.as_ptr() as u32;
        let len = msg.len() as u32;
        unsafe { internal::message(ptr, len) }
    }

    #[inline]
    pub fn messagef(args: Arguments<'_>) {
        let s: crate::String<120> = heapless::string::format(args).unwrap();
        message(&s)
    }

    /// Pause the runtime for a specified time
    ///
    /// # Arguments
    ///
    /// * `us`: Time in microseconds to pause the runtime
    ///
    /// returns: ()
    pub fn sleep(us: u64) {
        unsafe { internal::rsleep(us) }
    }

    #[cfg(target_arch = "wasm32")]
    #[cfg(not(test))]
    #[panic_handler]
    fn panic(info: &core::panic::PanicInfo) -> ! {
        // TODO(tumbar) Panic should transfer location information without formatting in-place
        // This can explode code-size so we avoid fmt here
        let filename = match info.location() {
            None => "no location",
            Some(loc) => loc.file(),
        };

        unsafe {
            internal::panic(filename.as_ptr() as u32, filename.len() as u32);
        }
    }
}
