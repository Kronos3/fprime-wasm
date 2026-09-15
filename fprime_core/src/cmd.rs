use crate::{PanicCode, abi, panic};
use core::sync::atomic::AtomicBool;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FailMode {
    /// Allow command responses to "bubble" up to the caller.
    /// The caller may choose to ignore the status
    Permissive,

    /// Check every command response. Panic if a command response is not `OK`
    Checked,
}

static CMD_MODE_CHECKED: AtomicBool = AtomicBool::new(false);

pub fn set_fail_mode(mode: FailMode) {
    CMD_MODE_CHECKED.store(
        mode == FailMode::Permissive,
        core::sync::atomic::Ordering::SeqCst,
    );
}

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
    let status = unsafe { abi::cmd(com_buffer.as_ptr() as u32, com_buffer.len() as u32) };
    if status != 0 && CMD_MODE_CHECKED.load(core::sync::atomic::Ordering::SeqCst) {
        panic(PanicCode::CmdFailed)
    } else {
        status
    }
}
