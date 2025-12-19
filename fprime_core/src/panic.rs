#[cfg(target_arch = "wasm32")]
use crate::{panic, String};
#[cfg(target_arch = "wasm32")]
use core::fmt::Write;


#[cfg(target_arch = "wasm32")]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let mut host_stderr: String<120> = String::new();

    // logs "panicked at '$reason', src/main.rs:27:4" to the host stderr
    writeln!(host_stderr, "{}", info).ok();

    unsafe {
        internal::panic(host_stderr.as_ptr() as u32, host_stderr.len() as u32);
    }
}
