use crate::{PanicCode, abi, panic};

const FPRIME_PARAM_VALID_UNINIT: i32 = 0;
const FPRIME_PARAM_VALID_VALID: i32 = 1;
const FPRIME_PARAM_VALID_INVALID: i32 = 2;
const FPRIME_PARAM_VALID_DEFAULT: i32 = 3;

/// Read a parameter value
///
/// # Arguments
///
/// * `id`: Parameter ID to read
/// * `value`: Buffer for holding the parameter value
///
/// returns: FprimeResult noting if the telemetry is valid or not
pub unsafe fn parameter(id: i64, value_buf: &mut [u8]) {
    let status = unsafe { abi::prm(id, value_buf.as_ptr() as u32, value_buf.len() as u32) };

    match status {
        FPRIME_PARAM_VALID_UNINIT => panic(PanicCode::PrmUninit),
        FPRIME_PARAM_VALID_VALID => {}
        FPRIME_PARAM_VALID_INVALID => {
            panic(PanicCode::PrmInvalid);
        }
        FPRIME_PARAM_VALID_DEFAULT => {}
        _ => panic(PanicCode::InvalidStatus),
    }
}
