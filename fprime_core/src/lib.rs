#![no_std]

use core::fmt::Write;
mod serializable;

pub use fprime_macros::Serializable;

/// Stack-backed String type
/// The capacity specifies the
pub type String<const N: usize> = heapless::String<N, u16>;

pub trait Serializable {
    const SIZE: usize;

    fn serialize_to(&self, to: &mut [u8], offset: &mut usize);
}

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
    fn command(com_buffer_ptr: u32, size: u32) -> i32;

    // pub fn telemetry(id: u32, out_buffer: *mut u8, size: u32) -> Time;

    // pub fn exit(code: i32) -> !;

    /// Panic the runtime with a message.
    /// This function should not return and the runtime should be stopped
    ///
    /// # Arguments
    ///
    /// * `msg_ptr`: Pointer to the panic string
    /// * `size`: length of the panic string
    fn panic(msg_ptr: u32, size: u32) -> !;
}

pub mod internal {
    pub unsafe fn command(com_buffer: &[u8]) -> i32 {
        let ptr = com_buffer.as_ptr();
        let size = com_buffer.len();
        unsafe { crate::command(ptr as u32, size as u32) }
    }
}

#[panic_handler]
fn panic_impl(info: &core::panic::PanicInfo) -> ! {
    let mut host_stderr: String<120> = String::new();

    // logs "panicked at '$reason', src/main.rs:27:4" to the host stderr
    writeln!(host_stderr, "{}", info).ok();

    unsafe {
        panic(host_stderr.as_ptr() as u32, host_stderr.len() as u32);
    }
}
