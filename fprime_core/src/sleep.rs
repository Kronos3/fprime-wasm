use crate::abi;

/// Pause the runtime for a specified time
///
/// # Arguments
///
/// * `us`: Time in microseconds to pause the runtime
///
/// returns: ()
pub fn rsleep(us: u64) {
    unsafe { abi::rsleep(us) }
}

/// Pause the runtime until a specified time
///
/// # Arguments
///
/// * `us`: Microseconds from system epoch to pause until
///
/// returns: ()
pub fn asleep(time: u64) {
    unsafe { abi::asleep(time) }
}
