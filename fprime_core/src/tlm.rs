use crate::{PanicCode, abi, panic};

/// Request last reported telemetry value
///
/// # Arguments
///
/// * `id`: Telemetry ID
/// * `time`: Buffer for holding last write time of telemetry
/// * `value`: Buffer for holding the telemetry value
///
/// returns: FprimeResult noting if the telemetry is valid or not
pub unsafe fn telemetry(id: i64, time_buf: &mut [u8], value_buf: &mut [u8]) {
    match unsafe {
        abi::tlm(
            id,
            time_buf.as_ptr() as u32,
            time_buf.len() as u32,
            value_buf.as_ptr() as u32,
            value_buf.len() as u32,
        )
    } {
        0 => {}
        _ => panic(PanicCode::TlmInvalid),
    }
}
