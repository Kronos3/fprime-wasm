use crate::abi;

pub unsafe fn time_read(buf: &mut [u8]) {
    unsafe { abi::time(buf.as_ptr() as u32, buf.len() as u32) };
}
