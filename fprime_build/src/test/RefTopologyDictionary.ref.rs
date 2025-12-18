#[allow(unused_imports)]
use fprime_core::*;
/// The type of a telemetry packet identifier
pub type FwTlmPacketizeIdType = u16;
/// The width of packet descriptors when they are serialized by the framework
pub type FwPacketDescriptorType = u16;
/// The type of a parameter identifier
pub type FwPrmIdType = crate::FwIdType;
/// The type of a data product priority
pub type FwDpPriorityType = u32;
/// The unsigned type of larger sizes internal to the software,
/// e.g., memory buffer sizes, file sizes. Must be unsigned.
/// Supplied by platform, overridable by project.
pub type PlatformSizeType = u64;
/// The type of a telemetry channel identifier
pub type FwChanIdType = crate::FwIdType;
/// The type used to serialize a time context value
pub type FwTimeContextStoreType = u8;
/// The type of a data product identifier
pub type FwDpIdType = crate::FwIdType;
/// The id type.
pub type FwIdType = u32;
/// The type of a command opcode
pub type FwOpcodeType = crate::FwIdType;
/// The unsigned type of larger sizes internal to the software,
/// e.g., memory buffer sizes, file sizes. Must be unsigned.
pub type FwSizeType = crate::PlatformSizeType;
/// The type used to serialize a size value
pub type FwSizeStoreType = u16;
/// The type used to serialize a time base value
pub type FwTimeBaseStoreType = u16;
/// The type of an event identifier
pub type FwEventIdType = crate::FwIdType;
pub mod cdh_core {
    #[allow(unused_imports)]
    use fprime_core::*;
    pub mod cmd_disp {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// No-op command
        pub fn cmd_no_op() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// No-op string command
        pub fn cmd_no_op_string(arg_1: String<40>) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<40> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            arg_1.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// No-op command
        pub fn cmd_test_cmd_1(
            arg_1: i32,
            arg_2: f32,
            arg_3: u8,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + i32::SIZE + f32::SIZE
                + u8::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            arg_1.serialize_to(&mut __encoded, &mut __offset);
            arg_2.serialize_to(&mut __encoded, &mut __offset);
            arg_3.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Clear command tracking info to recover from components not returning status
        pub fn cmd_clear_tracking() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod events {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Set filter for reporting events. Events are not stored in component.
        pub fn set_event_filter(
            filter_level: crate::svc::event_manager::FilterSeverity,
            filter_enabled: crate::svc::event_manager::Enabled,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::svc::event_manager::FilterSeverity::SIZE
                + crate::svc::event_manager::Enabled::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            filter_level.serialize_to(&mut __encoded, &mut __offset);
            filter_enabled.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Filter a particular ID
        pub fn set_id_filter(
            id: crate::FwEventIdType,
            id_filter_enabled: crate::svc::event_manager::Enabled,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::FwEventIdType::SIZE
                + crate::svc::event_manager::Enabled::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            id.serialize_to(&mut __encoded, &mut __offset);
            id_filter_enabled.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Dump the filter states via events
        pub fn dump_filter_state() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod health {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// A command to enable or disable health checks
        pub fn hlth_enable(enable: crate::fw::Enabled) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::fw::Enabled::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1002000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            enable.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Ignore a particular ping entry
        pub fn hlth_ping_enable(
            entry: String<40>,
            enable: crate::fw::Enabled,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<40> as Serializable>::SIZE + crate::fw::Enabled::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1002001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            entry.serialize_to(&mut __encoded, &mut __offset);
            enable.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Change ping value
        pub fn hlth_chng_ping(
            entry: String<40>,
            warning_value: u32,
            fatal_value: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<40> as Serializable>::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1002002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            entry.serialize_to(&mut __encoded, &mut __offset);
            warning_value.serialize_to(&mut __encoded, &mut __offset);
            fatal_value.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod version {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// A command to enable or disable Event verbosity and Telemetry
        pub fn enable(enable: crate::svc::VersionEnabled) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::svc::VersionEnabled::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1003000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            enable.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Report version as Event
        pub fn version(version_type: crate::svc::VersionType) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::svc::VersionType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1003001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            version_type.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
}
pub mod data_products {
    #[allow(unused_imports)]
    use fprime_core::*;
    pub mod dp_cat {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Build catalog from data product directory. Will block until complete
        pub fn build_catalog() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4000000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Start transmitting catalog
        pub fn start_xmit_catalog(wait: crate::fw::Wait) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + crate::fw::Wait::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4000001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            wait.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Stop transmitting catalog
        pub fn stop_xmit_catalog() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4000002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// clear existing catalog
        pub fn clear_catalog() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4000003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod dp_mgr {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Clear event throttling
        pub fn clear_event_throttle() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4001000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod dp_writer {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Clear event throttling
        pub fn clear_event_throttle() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4002000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
}
pub mod file_handling {
    #[allow(unused_imports)]
    use fprime_core::*;
    pub mod file_downlink {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Read a named file off the disk. Divide it into packets and send the packets for transmission to the ground.
        pub fn send_file(
            source_file_name: String<100>,
            dest_file_name: String<100>,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<100> as Serializable>::SIZE
                + <String<100> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5001000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            source_file_name.serialize_to(&mut __encoded, &mut __offset);
            dest_file_name.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Cancel the downlink in progress, if any
        pub fn cancel() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5001001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Read a named file off the disk from a starting position. Divide it into packets and send the packets for transmission to the ground.
        pub fn send_partial(
            source_file_name: String<100>,
            dest_file_name: String<100>,
            start_offset: u32,
            length: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<100> as Serializable>::SIZE
                + <String<100> as Serializable>::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5001002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            source_file_name.serialize_to(&mut __encoded, &mut __offset);
            dest_file_name.serialize_to(&mut __encoded, &mut __offset);
            start_offset.serialize_to(&mut __encoded, &mut __offset);
            length.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod file_manager {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Create a directory
        pub fn create_directory(dir_name: String<200>) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            dir_name.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Move a file
        pub fn move_file(
            source_file_name: String<200>,
            dest_file_name: String<200>,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            source_file_name.serialize_to(&mut __encoded, &mut __offset);
            dest_file_name.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Remove a directory, which must be empty
        pub fn remove_directory(dir_name: String<200>) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            dir_name.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Remove a file
        pub fn remove_file(
            file_name: String<200>,
            ignore_errors: bool,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE + bool::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            file_name.serialize_to(&mut __encoded, &mut __offset);
            ignore_errors.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Perform a Linux shell command and write the output to a log file.
        pub fn shell_command(
            command: String<256>,
            log_file_name: String<200>,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<256> as Serializable>::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002004;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            command.serialize_to(&mut __encoded, &mut __offset);
            log_file_name.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Append 1 file's contents to the end of another.
        pub fn append_file(
            source: String<200>,
            target: String<200>,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002005;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            source.serialize_to(&mut __encoded, &mut __offset);
            target.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        pub fn file_size(file_name: String<200>) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002006;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            file_name.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// List the contents of a directory
        pub fn list_directory(dir_name: String<200>) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002007;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            dir_name.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod prm_db {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Command to save parameter image to file. Uses file name passed to constructor
        pub fn prm_save_file() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5003000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
}
pub mod fw {
    #[allow(unused_imports)]
    use fprime_core::*;
    /// Enum representing a command response
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(i32)]
    pub enum CmdResponse {
        /// Command successfully executed
        Ok = 0,
        /// Invalid opcode dispatched
        InvalidOpcode = 1,
        /// Command failed validation
        ValidationError = 2,
        /// Command failed to deserialize
        FormatError = 3,
        /// Command had execution error
        ExecutionError = 4,
        /// Component busy
        Busy = 5,
    }
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(u8)]
    pub enum DpState {
        /// The untransmitted state
        Untransmitted = 0,
        /// The partially transmitted state
        /// A data product is in this state from the start of transmission
        /// until transmission is complete.
        Partial = 1,
        /// The transmitted state
        Transmitted = 2,
    }
    /// Enum representing parameter validity
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(i32)]
    pub enum ParamValid {
        Uninit = 0,
        Valid = 1,
        Invalid = 2,
        Default = 3,
    }
    /// Deserialization status
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(u8)]
    pub enum DeserialStatus {
        Ok = 0,
        /// Deserialization buffer was empty when trying to read data
        BufferEmpty = 3,
        /// Deserialization data had incorrect values (unexpected data types)
        FormatError = 4,
        /// Data was left in in the buffer, but not enough to deserialize
        SizeMismatch = 5,
        /// Deserialized type ID didn't match
        TypeMismatch = 6,
    }
    /// Wait or don't wait for something
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(u8)]
    pub enum Wait {
        /// Wait for something
        Wait = 0,
        /// Don't wait for something
        NoWait = 1,
    }
    /// Enabled and disabled states
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(u8)]
    pub enum Enabled {
        /// Disabled state
        Disabled = 0,
        /// Enabled state
        Enabled = 1,
    }
    pub mod dp_cfg {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// A bit mask for selecting the type of processing to perform on
        /// a container before writing it to disk.
        #[derive(Clone, Copy, Debug, Serializable)]
        #[repr(u8)]
        pub enum ProcType {
            /// Processing type 0
            ProcTypeZero = 1,
            /// Processing type 1
            ProcTypeOne = 2,
            /// Processing type 2
            ProcTypeTwo = 4,
        }
    }
}
pub mod r#ref {
    #[allow(unused_imports)]
    use fprime_core::*;
    #[derive(Clone, Debug, Serializable)]
    pub struct SignalPair {
        pub time: f32,
        pub value: f32,
    }
    /// Array of array
    pub type TooManyChoices = [crate::r#ref::ManyChoices; 2];
    /// Some Packet Statistics
    #[derive(Clone, Debug, Serializable)]
    pub struct PacketStat {
        /// Number of buffers received
        pub buff_recv: u32,
        /// Number of buffers received with errors
        pub buff_err: u32,
        /// Packet Status
        pub packet_status: crate::r#ref::PacketRecvStatus,
    }
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(i32)]
    pub enum SignalType {
        Triangle = 0,
        Square = 1,
        Sine = 2,
        Noise = 3,
    }
    pub type SignalPairSet = [crate::r#ref::SignalPair; 4];
    /// Enumeration type for use later
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(i32)]
    pub enum Choice {
        One = 0,
        Two = 1,
        Red = 2,
        Blue = 3,
    }
    /// Enumeration array
    pub type ManyChoices = [crate::r#ref::Choice; 2];
    /// Packet receive status
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(i32)]
    pub enum PacketRecvStatus {
        PacketStateNoPackets = 0,
        PacketStateOk = 1,
        /// Receiver has seen errors
        PacketStateErrors = 3,
    }
    /// Structure of enums (with an multi-dimensional array and structure)
    #[derive(Clone, Debug, Serializable)]
    pub struct ChoiceSlurry {
        /// A large set of disorganized choices
        pub too_many_choices: crate::r#ref::TooManyChoices,
        /// A singular choice
        pub separate_choice: crate::r#ref::Choice,
        /// A pair of choices
        pub choice_pair: crate::r#ref::ChoicePair,
        /// An array of choices defined as member array
        pub choice_as_member_array: [u8; 2],
    }
    /// Set of floating points to emit
    pub type FloatSet = [f32; 3];
    /// Structure of enums
    #[derive(Clone, Debug, Serializable)]
    pub struct ChoicePair {
        /// The first choice to make
        pub first_choice: crate::r#ref::Choice,
        /// The second choice to make
        pub second_choice: crate::r#ref::Choice,
    }
    /// All scalar inputs
    #[derive(Clone, Debug, Serializable)]
    pub struct ScalarStruct {
        pub i_8: i8,
        pub i_16: i16,
        pub i_32: i32,
        pub i_64: i64,
        pub u_8: u8,
        pub u_16: u16,
        pub u_32: u32,
        pub u_64: u64,
        pub f_32: f32,
        pub f_64: f64,
    }
    pub type SignalSet = [f32; 4];
    #[derive(Clone, Debug, Serializable)]
    pub struct SignalInfo {
        pub r#type: crate::r#ref::SignalType,
        pub history: crate::r#ref::SignalSet,
        pub pair_history: crate::r#ref::SignalPairSet,
    }
    pub mod cmd_seq {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Run a command sequence file
        pub fn cs_run(
            file_name: String<240>,
            block: crate::svc::cmd_sequencer::BlockState,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<240> as Serializable>::SIZE
                + crate::svc::cmd_sequencer::BlockState::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            file_name.serialize_to(&mut __encoded, &mut __offset);
            block.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Validate a command sequence file
        pub fn cs_validate(file_name: String<240>) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<240> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            file_name.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Cancel a command sequence
        pub fn cs_cancel() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Start running a command sequence
        pub fn cs_start() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Perform one step in a command sequence. Valid only if CmdSequencer is in MANUAL run mode.
        pub fn cs_step() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006004;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Set the run mode to AUTO.
        pub fn cs_auto() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006005;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Set the run mode to MANUAL.
        pub fn cs_manual() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006006;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Wait for sequences that are running to finish. Allow user to run multiple seq files in SEQ_NO_BLOCK mode then wait for them to finish before allowing more seq run request.
        pub fn cs_join_wait() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006007;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod dp_demo {
        #[allow(unused_imports)]
        use fprime_core::*;
        pub type BoolAlias = bool;
        #[derive(Clone, Copy, Debug, Serializable)]
        #[repr(i32)]
        pub enum ColorEnum {
            Red = 0,
            Green = 1,
            Blue = 2,
        }
        #[derive(Clone, Copy, Debug, Serializable)]
        #[repr(i32)]
        pub enum DpReqType {
            Immediate = 0,
            Async = 1,
        }
        #[derive(Clone, Debug, Serializable)]
        pub struct StructWithEverything {
            pub integer_member: crate::r#ref::dp_demo::I32Alias,
            pub float_member: f32,
            pub string_member: String<80>,
            pub boolean_member: bool,
            pub enum_member: crate::r#ref::dp_demo::ColorEnum,
            pub array_member_u_32: [crate::r#ref::dp_demo::U32Array; 2],
            pub f_32_array: crate::r#ref::dp_demo::F32Array,
            pub u_32_array: crate::r#ref::dp_demo::U32Array,
            pub enum_array: crate::r#ref::dp_demo::EnumArray,
            pub string_array: crate::r#ref::dp_demo::StringArray,
            pub boolean_array: crate::r#ref::dp_demo::BooleanArray,
            pub struct_with_strings: crate::r#ref::dp_demo::StructWithStringMembers,
            pub nested_arrays: crate::r#ref::dp_demo::ArrayOfStringArray,
        }
        /// Array of integers
        pub type U32Array = [u32; 5];
        pub type F64Alias = f64;
        pub type ArrayOfStructs = [crate::r#ref::dp_demo::StructWithStringMembers; 3];
        #[derive(Clone, Debug, Serializable)]
        pub struct ColorInfoStruct {
            pub color: crate::r#ref::dp_demo::ColorEnum,
        }
        /// Array of strings
        pub type StringArray = [String<80>; 2];
        pub type StringAlias = String<80>;
        pub type I32Alias = i32;
        #[derive(Clone, Debug, Serializable)]
        pub struct StructWithStringMembers {
            pub string_member: String<80>,
            pub string_array_member: crate::r#ref::dp_demo::StringArray,
        }
        /// Array of floats
        pub type F32Array = [f32; 3];
        /// Array of enumerations
        pub type EnumArray = [crate::r#ref::dp_demo::ColorEnum; 3];
        /// Array of booleans
        pub type BooleanArray = [bool; 2];
        /// Array of array of strings
        pub type ArrayOfStringArray = [crate::r#ref::dp_demo::StringArray; 3];
        /// Select color
        pub fn select_color(
            color: crate::r#ref::dp_demo::ColorEnum,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::dp_demo::ColorEnum::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0xA10;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            color.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Command for generating a DP
        pub fn dp(
            req_type: crate::r#ref::dp_demo::DpReqType,
            priority: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::dp_demo::DpReqType::SIZE + u32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0xA11;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            req_type.serialize_to(&mut __encoded, &mut __offset);
            priority.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod ping_rcvr {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Command to disable ping response
        pub fn pr_stop_pings() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10004000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod recv_buff_comp {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// A test parameter
        pub fn parameter_1_prm_set(val: u32) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10022000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// A test parameter
        pub fn parameter_1_prm_save() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10022001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// A test parameter
        pub fn parameter_2_prm_set(val: i16) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + i16::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10022002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// A test parameter
        pub fn parameter_2_prm_save() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10022003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod send_buff {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Active state
        #[derive(Clone, Copy, Debug, Serializable)]
        #[repr(i32)]
        pub enum ActiveState {
            SendIdle = 0,
            SendActive = 1,
        }
    }
    pub mod send_buff_comp {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Command to start sending packets
        pub fn sb_start_pkts() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10010000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Send a bad packet
        pub fn sb_inject_pkt_error() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10010001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Generate a FATAL EVR
        pub fn sb_gen_fatal(
            arg_1: u32,
            arg_2: u32,
            arg_3: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE + u32::SIZE
                + u32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10010002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            arg_1.serialize_to(&mut __encoded, &mut __offset);
            arg_2.serialize_to(&mut __encoded, &mut __offset);
            arg_3.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Generate an ASSERT
        pub fn sb_gen_assert(
            arg_1: u32,
            arg_2: u32,
            arg_3: u32,
            arg_4: u32,
            arg_5: u32,
            arg_6: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE + u32::SIZE
                + u32::SIZE + u32::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10010003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            arg_1.serialize_to(&mut __encoded, &mut __offset);
            arg_2.serialize_to(&mut __encoded, &mut __offset);
            arg_3.serialize_to(&mut __encoded, &mut __offset);
            arg_4.serialize_to(&mut __encoded, &mut __offset);
            arg_5.serialize_to(&mut __encoded, &mut __offset);
            arg_6.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// A test parameter
        pub fn parameter_3_prm_set(val: u8) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u8::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001000A;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// A test parameter
        pub fn parameter_3_prm_save() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001000B;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// A test parameter
        pub fn parameter_4_prm_set(val: f32) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + f32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001000C;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// A test parameter
        pub fn parameter_4_prm_save() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001000D;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod sg_1 {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Signal Generator Settings
        pub fn settings(
            frequency: u32,
            amplitude: f32,
            phase: f32,
            sig_type: crate::r#ref::SignalType,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE + f32::SIZE
                + f32::SIZE + crate::r#ref::SignalType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10011000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            frequency.serialize_to(&mut __encoded, &mut __offset);
            amplitude.serialize_to(&mut __encoded, &mut __offset);
            phase.serialize_to(&mut __encoded, &mut __offset);
            sig_type.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Toggle Signal Generator On/Off.
        pub fn toggle() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10011001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Skip next sample
        pub fn skip() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10011002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Signal Generator Settings
        pub fn dp(
            req_type: crate::r#ref::signal_gen::DpReqType,
            records: u32,
            priority: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::signal_gen::DpReqType::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10011003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            req_type.serialize_to(&mut __encoded, &mut __offset);
            records.serialize_to(&mut __encoded, &mut __offset);
            priority.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod sg_2 {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Signal Generator Settings
        pub fn settings(
            frequency: u32,
            amplitude: f32,
            phase: f32,
            sig_type: crate::r#ref::SignalType,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE + f32::SIZE
                + f32::SIZE + crate::r#ref::SignalType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10012000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            frequency.serialize_to(&mut __encoded, &mut __offset);
            amplitude.serialize_to(&mut __encoded, &mut __offset);
            phase.serialize_to(&mut __encoded, &mut __offset);
            sig_type.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Toggle Signal Generator On/Off.
        pub fn toggle() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10012001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Skip next sample
        pub fn skip() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10012002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Signal Generator Settings
        pub fn dp(
            req_type: crate::r#ref::signal_gen::DpReqType,
            records: u32,
            priority: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::signal_gen::DpReqType::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10012003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            req_type.serialize_to(&mut __encoded, &mut __offset);
            records.serialize_to(&mut __encoded, &mut __offset);
            priority.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod sg_3 {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Signal Generator Settings
        pub fn settings(
            frequency: u32,
            amplitude: f32,
            phase: f32,
            sig_type: crate::r#ref::SignalType,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE + f32::SIZE
                + f32::SIZE + crate::r#ref::SignalType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10013000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            frequency.serialize_to(&mut __encoded, &mut __offset);
            amplitude.serialize_to(&mut __encoded, &mut __offset);
            phase.serialize_to(&mut __encoded, &mut __offset);
            sig_type.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Toggle Signal Generator On/Off.
        pub fn toggle() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10013001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Skip next sample
        pub fn skip() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10013002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Signal Generator Settings
        pub fn dp(
            req_type: crate::r#ref::signal_gen::DpReqType,
            records: u32,
            priority: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::signal_gen::DpReqType::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10013003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            req_type.serialize_to(&mut __encoded, &mut __offset);
            records.serialize_to(&mut __encoded, &mut __offset);
            priority.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod sg_4 {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Signal Generator Settings
        pub fn settings(
            frequency: u32,
            amplitude: f32,
            phase: f32,
            sig_type: crate::r#ref::SignalType,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE + f32::SIZE
                + f32::SIZE + crate::r#ref::SignalType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10014000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            frequency.serialize_to(&mut __encoded, &mut __offset);
            amplitude.serialize_to(&mut __encoded, &mut __offset);
            phase.serialize_to(&mut __encoded, &mut __offset);
            sig_type.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Toggle Signal Generator On/Off.
        pub fn toggle() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10014001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Skip next sample
        pub fn skip() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10014002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Signal Generator Settings
        pub fn dp(
            req_type: crate::r#ref::signal_gen::DpReqType,
            records: u32,
            priority: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::signal_gen::DpReqType::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10014003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            req_type.serialize_to(&mut __encoded, &mut __offset);
            records.serialize_to(&mut __encoded, &mut __offset);
            priority.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod sg_5 {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Signal Generator Settings
        pub fn settings(
            frequency: u32,
            amplitude: f32,
            phase: f32,
            sig_type: crate::r#ref::SignalType,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE + f32::SIZE
                + f32::SIZE + crate::r#ref::SignalType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10015000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            frequency.serialize_to(&mut __encoded, &mut __offset);
            amplitude.serialize_to(&mut __encoded, &mut __offset);
            phase.serialize_to(&mut __encoded, &mut __offset);
            sig_type.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Toggle Signal Generator On/Off.
        pub fn toggle() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10015001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Skip next sample
        pub fn skip() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10015002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Signal Generator Settings
        pub fn dp(
            req_type: crate::r#ref::signal_gen::DpReqType,
            records: u32,
            priority: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::signal_gen::DpReqType::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10015003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            req_type.serialize_to(&mut __encoded, &mut __offset);
            records.serialize_to(&mut __encoded, &mut __offset);
            priority.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod signal_gen {
        #[allow(unused_imports)]
        use fprime_core::*;
        #[derive(Clone, Copy, Debug, Serializable)]
        #[repr(i32)]
        pub enum DpReqType {
            Immediate = 0,
            Async = 1,
        }
    }
    pub mod system_resources {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// A command to enable or disable system resource telemetry
        pub fn enable(
            enable: crate::svc::SystemResourceEnabled,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::svc::SystemResourceEnabled::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10023000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            enable.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
    pub mod type_demo {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Single choice command
        pub fn choice(choice: crate::r#ref::Choice) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::Choice::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            choice.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Single enumeration parameter
        pub fn choice_prm_prm_set(val: crate::r#ref::Choice) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::Choice::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Single enumeration parameter
        pub fn choice_prm_prm_save() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Multiple choice command via Array
        pub fn choices(choices: crate::r#ref::ManyChoices) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ManyChoices::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Multiple choice command via Array with a preceding and following argument
        pub fn choices_with_friends(
            repeat: u8,
            choices: crate::r#ref::ManyChoices,
            repeat_max: u8,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u8::SIZE
                + crate::r#ref::ManyChoices::SIZE + u8::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005004;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            repeat.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            repeat_max.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Multiple enumeration parameter via Array
        pub fn choices_prm_prm_set(
            val: crate::r#ref::ManyChoices,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ManyChoices::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005005;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Multiple enumeration parameter via Array
        pub fn choices_prm_prm_save() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005006;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Too many choice command via Array
        pub fn extra_choices(
            choices: crate::r#ref::TooManyChoices,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::TooManyChoices::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005007;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Too many choices command via Array with a preceding and following argument
        pub fn extra_choices_with_friends(
            repeat: u8,
            choices: crate::r#ref::TooManyChoices,
            repeat_max: u8,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u8::SIZE
                + crate::r#ref::TooManyChoices::SIZE + u8::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005008;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            repeat.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            repeat_max.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Too many enumeration parameter via Array
        pub fn extra_choices_prm_prm_set(
            val: crate::r#ref::ManyChoices,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ManyChoices::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005009;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Too many enumeration parameter via Array
        pub fn extra_choices_prm_prm_save() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500A;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Multiple choice command via Structure
        pub fn choice_pair(choices: crate::r#ref::ChoicePair) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ChoicePair::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500B;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Multiple choices command via Structure with a preceding and following argument
        pub fn choice_pair_with_friends(
            repeat: u8,
            choices: crate::r#ref::ChoicePair,
            repeat_max: u8,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u8::SIZE
                + crate::r#ref::ChoicePair::SIZE + u8::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500C;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            repeat.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            repeat_max.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Multiple enumeration parameter via Structure
        pub fn choice_pair_prm_prm_set(
            val: crate::r#ref::ChoicePair,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ChoicePair::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500D;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Multiple enumeration parameter via Structure
        pub fn choice_pair_prm_prm_save() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500E;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Multiple choice command via Complex Structure
        pub fn glutton_of_choice(
            choices: crate::r#ref::ChoiceSlurry,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ChoiceSlurry::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500F;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Multiple choices command via Complex Structure with a preceding and following argument
        pub fn glutton_of_choice_with_friends(
            repeat: u8,
            choices: crate::r#ref::ChoiceSlurry,
            repeat_max: u8,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u8::SIZE
                + crate::r#ref::ChoiceSlurry::SIZE + u8::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005010;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            repeat.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            repeat_max.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Multiple enumeration parameter via Complex Structure
        pub fn glutton_of_choice_prm_prm_set(
            val: crate::r#ref::ChoiceSlurry,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ChoiceSlurry::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005011;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Multiple enumeration parameter via Complex Structure
        pub fn glutton_of_choice_prm_prm_save() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005012;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Dump the typed parameters
        pub fn dump_typed_parameters() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005013;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Dump the float values
        pub fn dump_floats() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005014;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
        /// Send scalars
        pub fn send_scalars(
            scalar_input: crate::r#ref::ScalarStruct,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ScalarStruct::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005015;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            scalar_input.serialize_to(&mut __encoded, &mut __offset);
            unsafe { core::mem::transmute(internal::command(&__encoded[0..__offset])) }
        }
    }
}
pub mod svc {
    #[allow(unused_imports)]
    use fprime_core::*;
    /// An enumeration for Version Type
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(i32)]
    pub enum VersionType {
        /// project version
        Project = 0,
        /// framework version
        Framework = 1,
        /// library version
        Library = 2,
        /// custom version
        Custom = 3,
        /// all above versions
        All = 4,
    }
    /// Array of queue depths for Fw::Com types
    pub type ComQueueDepth = [u32; 2];
    /// Array of queue depths for Fw::Buffer types
    pub type BuffQueueDepth = [u32; 1];
    /// An enumeration for version status
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(i32)]
    pub enum VersionStatus {
        /// Version was good
        Ok = 0,
        /// Failure to get version
        Failure = 1,
    }
    /// Tracks versions for project, framework and user defined versions etc
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(i32)]
    pub enum VersionEnabled {
        /// verbosity disabled
        Disabled = 0,
        /// verbosity enabled
        Enabled = 1,
    }
    /// Data structure representing a data product.
    #[derive(Clone, Debug, Serializable)]
    pub struct DpRecord {
        pub id: crate::FwDpIdType,
        pub t_sec: u32,
        pub t_sub: u32,
        pub priority: u32,
        pub size: u64,
        pub blocks: u32,
        pub state: crate::fw::DpState,
    }
    /// Send file status enum
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(i32)]
    pub enum SendFileStatus {
        StatusOk = 0,
        StatusError = 1,
        StatusInvalid = 2,
        StatusBusy = 3,
    }
    /// An enumeration of queue data types
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(i32)]
    pub enum QueueType {
        ComQueue = 0,
        BufferQueue = 1,
    }
    /// Header validation error
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(i32)]
    pub enum DpHdrField {
        Descriptor = 0,
        Id = 1,
        Priority = 2,
        Crc = 3,
    }
    #[derive(Clone, Copy, Debug, Serializable)]
    #[repr(i32)]
    pub enum SystemResourceEnabled {
        Disabled = 0,
        Enabled = 1,
    }
    /// Data Structure for custom version Tlm
    #[derive(Clone, Debug, Serializable)]
    pub struct CustomVersionDb {
        /// enumeration/name of the custom version
        pub version_enum: crate::svc::version_cfg::VersionEnum,
        /// string containing custom version
        pub version_value: String<80>,
        /// status of the custom version
        pub version_status: crate::svc::VersionStatus,
    }
    pub mod cmd_sequencer {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// The stage of the file read operation
        #[derive(Clone, Copy, Debug, Serializable)]
        #[repr(i32)]
        pub enum FileReadStage {
            ReadHeader = 0,
            ReadHeaderSize = 1,
            DeserSize = 2,
            DeserNumRecords = 3,
            DeserTimeBase = 4,
            DeserTimeContext = 5,
            ReadSeqCrc = 6,
            ReadSeqData = 7,
            ReadSeqDataSize = 8,
        }
        /// Sequencer blocking state
        #[derive(Clone, Copy, Debug, Serializable)]
        #[repr(i32)]
        pub enum BlockState {
            Block = 0,
            NoBlock = 1,
        }
        /// The sequencer mode
        #[derive(Clone, Copy, Debug, Serializable)]
        #[repr(i32)]
        pub enum SeqMode {
            Step = 0,
            Auto = 1,
        }
    }
    pub mod event_manager {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Severity level for event filtering
        /// Similar to Fw::LogSeverity, but no FATAL event
        #[derive(Clone, Copy, Debug, Serializable)]
        #[repr(i32)]
        pub enum FilterSeverity {
            /// Filter WARNING_HI events
            WarningHi = 0,
            /// Filter WARNING_LO events
            WarningLo = 1,
            /// Filter COMMAND events
            Command = 2,
            /// Filter ACTIVITY_HI events
            ActivityHi = 3,
            /// Filter ACTIVITY_LO events
            ActivityLo = 4,
            /// Filter DIAGNOSTIC events
            Diagnostic = 5,
        }
        /// Enabled and disabled state
        #[derive(Clone, Copy, Debug, Serializable)]
        #[repr(i32)]
        pub enum Enabled {
            /// Enabled state
            Enabled = 0,
            /// Disabled state
            Disabled = 1,
        }
    }
    pub mod prm_db {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Parameter read error
        #[derive(Clone, Copy, Debug, Serializable)]
        #[repr(i32)]
        pub enum PrmReadError {
            Open = 0,
            Delimiter = 1,
            DelimiterSize = 2,
            DelimiterValue = 3,
            RecordSize = 4,
            RecordSizeSize = 5,
            RecordSizeValue = 6,
            ParameterId = 7,
            ParameterIdSize = 8,
            ParameterValue = 9,
            ParameterValueSize = 10,
        }
        /// Parameter write error
        #[derive(Clone, Copy, Debug, Serializable)]
        #[repr(i32)]
        pub enum PrmWriteError {
            Open = 0,
            Delimiter = 1,
            DelimiterSize = 2,
            RecordSize = 3,
            RecordSizeSize = 4,
            ParameterId = 5,
            ParameterIdSize = 6,
            ParameterValue = 7,
            ParameterValueSize = 8,
        }
    }
    pub mod version_cfg {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Define a set of Version entries on a project-specific
        /// basis.
        #[derive(Clone, Copy, Debug, Serializable)]
        #[repr(u32)]
        pub enum VersionEnum {
            /// Entry 0
            ProjectVersion00 = 0,
            /// Entry 1
            ProjectVersion01 = 1,
            /// Entry 2
            ProjectVersion02 = 2,
            /// Entry 3
            ProjectVersion03 = 3,
            /// Entry 4
            ProjectVersion04 = 4,
            /// Entry 5
            ProjectVersion05 = 5,
            /// Entry 6
            ProjectVersion06 = 6,
            /// Entry 7
            ProjectVersion07 = 7,
            /// Entry 8
            ProjectVersion08 = 8,
            /// Entry 9
            ProjectVersion09 = 9,
        }
    }
}
