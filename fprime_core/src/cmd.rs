use crate::{PanicCode, abi, panic};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FailMode {
    /// Allow command responses to "bubble" up to the caller.
    /// The caller may choose to ignore the status
    Permissive,

    /// Check every command response. Panic if a command response is not `OK`
    Checked,
}

static mut CMD_MODE_CHECKED: FailMode = FailMode::Checked;

pub fn set_fail_mode(mode: FailMode) {
    unsafe {
        CMD_MODE_CHECKED = mode;
    }
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
    if status != 0 && unsafe { CMD_MODE_CHECKED == FailMode::Checked } {
        panic(PanicCode::CmdFailed)
    } else {
        status
    }
}
