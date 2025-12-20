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
