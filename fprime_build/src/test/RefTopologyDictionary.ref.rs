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
    use fprime_core::*;
    pub mod cmd_disp {
        use fprime_core::*;
        /// No-op command
        pub fn CmdNoOp() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// No-op string command
        pub fn CmdNoOpString(arg_1: String<40>) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<40> as Serializable>::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            arg_1.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// No-op command
        pub fn CmdTestCmd1(arg_1: i32, arg_2: f32, arg_3: u8) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + i32::SIZE + f32::SIZE
                + u8::SIZE] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            arg_1.serialize_to(&mut __encoded, &mut __offset);
            arg_2.serialize_to(&mut __encoded, &mut __offset);
            arg_3.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Clear command tracking info to recover from components not returning status
        pub fn CmdClearTracking() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod events {
        use fprime_core::*;
        /// Set filter for reporting events. Events are not stored in component.
        pub fn SetEventFilter(
            filter_level: crate::svc::event_manager::FilterSeverity,
            filter_enabled: crate::svc::event_manager::Enabled,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::svc::event_manager::FilterSeverity::SIZE
                + crate::svc::event_manager::Enabled::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            filter_level.serialize_to(&mut __encoded, &mut __offset);
            filter_enabled.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Filter a particular ID
        pub fn SetIdFilter(
            id: crate::FwEventIdType,
            id_filter_enabled: crate::svc::event_manager::Enabled,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::FwEventIdType::SIZE
                + crate::svc::event_manager::Enabled::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            id.serialize_to(&mut __encoded, &mut __offset);
            id_filter_enabled.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Dump the filter states via events
        pub fn DumpFilterState() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod health {
        use fprime_core::*;
        /// A command to enable or disable health checks
        pub fn HlthEnable(enable: crate::fw::Enabled) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::fw::Enabled::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1002000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            enable.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Ignore a particular ping entry
        pub fn HlthPingEnable(
            entry: String<40>,
            enable: crate::fw::Enabled,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<40> as Serializable>::SIZE + crate::fw::Enabled::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1002001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            entry.serialize_to(&mut __encoded, &mut __offset);
            enable.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Change ping value
        pub fn HlthChngPing(
            entry: String<40>,
            warning_value: u32,
            fatal_value: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<40> as Serializable>::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1002002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            entry.serialize_to(&mut __encoded, &mut __offset);
            warning_value.serialize_to(&mut __encoded, &mut __offset);
            fatal_value.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod version {
        use fprime_core::*;
        /// A command to enable or disable Event verbosity and Telemetry
        pub fn Enable(enable: crate::svc::VersionEnabled) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::svc::VersionEnabled::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1003000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            enable.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Report version as Event
        pub fn Version(version_type: crate::svc::VersionType) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::svc::VersionType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1003001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            version_type.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
}
pub mod data_products {
    use fprime_core::*;
    pub mod dp_cat {
        use fprime_core::*;
        /// Build catalog from data product directory. Will block until complete
        pub fn BuildCatalog() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4000000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Start transmitting catalog
        pub fn StartXmitCatalog(wait: crate::fw::Wait) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + crate::fw::Wait::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4000001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            wait.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Stop transmitting catalog
        pub fn StopXmitCatalog() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4000002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// clear existing catalog
        pub fn ClearCatalog() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4000003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod dp_mgr {
        use fprime_core::*;
        /// Clear event throttling
        pub fn ClearEventThrottle() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4001000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod dp_writer {
        use fprime_core::*;
        /// Clear event throttling
        pub fn ClearEventThrottle() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4002000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
}
pub mod file_handling {
    use fprime_core::*;
    pub mod file_downlink {
        use fprime_core::*;
        /// Read a named file off the disk. Divide it into packets and send the packets for transmission to the ground.
        pub fn SendFile(
            source_file_name: String<100>,
            dest_file_name: String<100>,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<100> as Serializable>::SIZE
                + <String<100> as Serializable>::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5001000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            source_file_name.serialize_to(&mut __encoded, &mut __offset);
            dest_file_name.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Cancel the downlink in progress, if any
        pub fn Cancel() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5001001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Read a named file off the disk from a starting position. Divide it into packets and send the packets for transmission to the ground.
        pub fn SendPartial(
            source_file_name: String<100>,
            dest_file_name: String<100>,
            start_offset: u32,
            length: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<100> as Serializable>::SIZE
                + <String<100> as Serializable>::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5001002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            source_file_name.serialize_to(&mut __encoded, &mut __offset);
            dest_file_name.serialize_to(&mut __encoded, &mut __offset);
            start_offset.serialize_to(&mut __encoded, &mut __offset);
            length.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod file_manager {
        use fprime_core::*;
        /// Create a directory
        pub fn CreateDirectory(dir_name: String<200>) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            dir_name.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Move a file
        pub fn MoveFile(
            source_file_name: String<200>,
            dest_file_name: String<200>,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            source_file_name.serialize_to(&mut __encoded, &mut __offset);
            dest_file_name.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Remove a directory, which must be empty
        pub fn RemoveDirectory(dir_name: String<200>) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            dir_name.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Remove a file
        pub fn RemoveFile(
            file_name: String<200>,
            ignore_errors: bool,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE + bool::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            file_name.serialize_to(&mut __encoded, &mut __offset);
            ignore_errors.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Perform a Linux shell command and write the output to a log file.
        pub fn ShellCommand(
            command: String<256>,
            log_file_name: String<200>,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<256> as Serializable>::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002004;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            command.serialize_to(&mut __encoded, &mut __offset);
            log_file_name.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Append 1 file's contents to the end of another.
        pub fn AppendFile(
            source: String<200>,
            target: String<200>,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002005;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            source.serialize_to(&mut __encoded, &mut __offset);
            target.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        pub fn FileSize(file_name: String<200>) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002006;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            file_name.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// List the contents of a directory
        pub fn ListDirectory(dir_name: String<200>) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002007;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            dir_name.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod prm_db {
        use fprime_core::*;
        /// Command to save parameter image to file. Uses file name passed to constructor
        pub fn PrmSaveFile() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5003000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
}
pub mod fw {
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
        use fprime_core::*;
        /// Run a command sequence file
        pub fn CsRun(
            file_name: String<240>,
            block: crate::svc::cmd_sequencer::BlockState,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<240> as Serializable>::SIZE
                + crate::svc::cmd_sequencer::BlockState::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            file_name.serialize_to(&mut __encoded, &mut __offset);
            block.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Validate a command sequence file
        pub fn CsValidate(file_name: String<240>) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<240> as Serializable>::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            file_name.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Cancel a command sequence
        pub fn CsCancel() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Start running a command sequence
        pub fn CsStart() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Perform one step in a command sequence. Valid only if CmdSequencer is in MANUAL run mode.
        pub fn CsStep() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006004;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Set the run mode to AUTO.
        pub fn CsAuto() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006005;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Set the run mode to MANUAL.
        pub fn CsManual() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006006;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Wait for sequences that are running to finish. Allow user to run multiple seq files in SEQ_NO_BLOCK mode then wait for them to finish before allowing more seq run request.
        pub fn CsJoinWait() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006007;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod dp_demo {
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
        pub fn SelectColor(
            color: crate::r#ref::dp_demo::ColorEnum,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::dp_demo::ColorEnum::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0xA10;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            color.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Command for generating a DP
        pub fn Dp(
            req_type: crate::r#ref::dp_demo::DpReqType,
            priority: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::dp_demo::DpReqType::SIZE + u32::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0xA11;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            req_type.serialize_to(&mut __encoded, &mut __offset);
            priority.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod ping_rcvr {
        use fprime_core::*;
        /// Command to disable ping response
        pub fn PrStopPings() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10004000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod recv_buff_comp {
        use fprime_core::*;
        /// A test parameter
        pub fn Parameter1PrmSet(val: u32) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10022000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// A test parameter
        pub fn Parameter1PrmSave() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10022001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// A test parameter
        pub fn Parameter2PrmSet(val: i16) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + i16::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10022002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// A test parameter
        pub fn Parameter2PrmSave() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10022003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod send_buff {
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
        use fprime_core::*;
        /// Command to start sending packets
        pub fn SbStartPkts() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10010000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Send a bad packet
        pub fn SbInjectPktError() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10010001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Generate a FATAL EVR
        pub fn SbGenFatal(arg_1: u32, arg_2: u32, arg_3: u32) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE + u32::SIZE
                + u32::SIZE] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10010002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            arg_1.serialize_to(&mut __encoded, &mut __offset);
            arg_2.serialize_to(&mut __encoded, &mut __offset);
            arg_3.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Generate an ASSERT
        pub fn SbGenAssert(
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
                core::mem::MaybeUninit::uninit().assume_init()
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
            crate::fw::CmdResponse::Ok
        }
        /// A test parameter
        pub fn Parameter3PrmSet(val: u8) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u8::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001000A;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// A test parameter
        pub fn Parameter3PrmSave() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001000B;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// A test parameter
        pub fn Parameter4PrmSet(val: f32) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + f32::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001000C;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// A test parameter
        pub fn Parameter4PrmSave() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001000D;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod sg_1 {
        use fprime_core::*;
        /// Signal Generator Settings
        pub fn Settings(
            frequency: u32,
            amplitude: f32,
            phase: f32,
            sig_type: crate::r#ref::SignalType,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE + f32::SIZE
                + f32::SIZE + crate::r#ref::SignalType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10011000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            frequency.serialize_to(&mut __encoded, &mut __offset);
            amplitude.serialize_to(&mut __encoded, &mut __offset);
            phase.serialize_to(&mut __encoded, &mut __offset);
            sig_type.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Toggle Signal Generator On/Off.
        pub fn Toggle() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10011001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Skip next sample
        pub fn Skip() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10011002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Signal Generator Settings
        pub fn Dp(
            req_type: crate::r#ref::signal_gen::DpReqType,
            records: u32,
            priority: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::signal_gen::DpReqType::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10011003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            req_type.serialize_to(&mut __encoded, &mut __offset);
            records.serialize_to(&mut __encoded, &mut __offset);
            priority.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod sg_2 {
        use fprime_core::*;
        /// Signal Generator Settings
        pub fn Settings(
            frequency: u32,
            amplitude: f32,
            phase: f32,
            sig_type: crate::r#ref::SignalType,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE + f32::SIZE
                + f32::SIZE + crate::r#ref::SignalType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10012000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            frequency.serialize_to(&mut __encoded, &mut __offset);
            amplitude.serialize_to(&mut __encoded, &mut __offset);
            phase.serialize_to(&mut __encoded, &mut __offset);
            sig_type.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Toggle Signal Generator On/Off.
        pub fn Toggle() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10012001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Skip next sample
        pub fn Skip() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10012002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Signal Generator Settings
        pub fn Dp(
            req_type: crate::r#ref::signal_gen::DpReqType,
            records: u32,
            priority: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::signal_gen::DpReqType::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10012003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            req_type.serialize_to(&mut __encoded, &mut __offset);
            records.serialize_to(&mut __encoded, &mut __offset);
            priority.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod sg_3 {
        use fprime_core::*;
        /// Signal Generator Settings
        pub fn Settings(
            frequency: u32,
            amplitude: f32,
            phase: f32,
            sig_type: crate::r#ref::SignalType,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE + f32::SIZE
                + f32::SIZE + crate::r#ref::SignalType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10013000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            frequency.serialize_to(&mut __encoded, &mut __offset);
            amplitude.serialize_to(&mut __encoded, &mut __offset);
            phase.serialize_to(&mut __encoded, &mut __offset);
            sig_type.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Toggle Signal Generator On/Off.
        pub fn Toggle() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10013001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Skip next sample
        pub fn Skip() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10013002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Signal Generator Settings
        pub fn Dp(
            req_type: crate::r#ref::signal_gen::DpReqType,
            records: u32,
            priority: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::signal_gen::DpReqType::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10013003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            req_type.serialize_to(&mut __encoded, &mut __offset);
            records.serialize_to(&mut __encoded, &mut __offset);
            priority.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod sg_4 {
        use fprime_core::*;
        /// Signal Generator Settings
        pub fn Settings(
            frequency: u32,
            amplitude: f32,
            phase: f32,
            sig_type: crate::r#ref::SignalType,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE + f32::SIZE
                + f32::SIZE + crate::r#ref::SignalType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10014000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            frequency.serialize_to(&mut __encoded, &mut __offset);
            amplitude.serialize_to(&mut __encoded, &mut __offset);
            phase.serialize_to(&mut __encoded, &mut __offset);
            sig_type.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Toggle Signal Generator On/Off.
        pub fn Toggle() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10014001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Skip next sample
        pub fn Skip() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10014002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Signal Generator Settings
        pub fn Dp(
            req_type: crate::r#ref::signal_gen::DpReqType,
            records: u32,
            priority: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::signal_gen::DpReqType::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10014003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            req_type.serialize_to(&mut __encoded, &mut __offset);
            records.serialize_to(&mut __encoded, &mut __offset);
            priority.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod sg_5 {
        use fprime_core::*;
        /// Signal Generator Settings
        pub fn Settings(
            frequency: u32,
            amplitude: f32,
            phase: f32,
            sig_type: crate::r#ref::SignalType,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE + f32::SIZE
                + f32::SIZE + crate::r#ref::SignalType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10015000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            frequency.serialize_to(&mut __encoded, &mut __offset);
            amplitude.serialize_to(&mut __encoded, &mut __offset);
            phase.serialize_to(&mut __encoded, &mut __offset);
            sig_type.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Toggle Signal Generator On/Off.
        pub fn Toggle() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10015001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Skip next sample
        pub fn Skip() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10015002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Signal Generator Settings
        pub fn Dp(
            req_type: crate::r#ref::signal_gen::DpReqType,
            records: u32,
            priority: u32,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::signal_gen::DpReqType::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10015003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            req_type.serialize_to(&mut __encoded, &mut __offset);
            records.serialize_to(&mut __encoded, &mut __offset);
            priority.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod signal_gen {
        use fprime_core::*;
        #[derive(Clone, Copy, Debug, Serializable)]
        #[repr(i32)]
        pub enum DpReqType {
            Immediate = 0,
            Async = 1,
        }
    }
    pub mod system_resources {
        use fprime_core::*;
        /// A command to enable or disable system resource telemetry
        pub fn Enable(
            enable: crate::svc::SystemResourceEnabled,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::svc::SystemResourceEnabled::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10023000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            enable.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
    pub mod type_demo {
        use fprime_core::*;
        /// Single choice command
        pub fn Choice(choice: crate::r#ref::Choice) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::Choice::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            choice.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Single enumeration parameter
        pub fn ChoicePrmPrmSet(val: crate::r#ref::Choice) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::Choice::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Single enumeration parameter
        pub fn ChoicePrmPrmSave() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Multiple choice command via Array
        pub fn Choices(choices: crate::r#ref::ManyChoices) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ManyChoices::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Multiple choice command via Array with a preceding and following argument
        pub fn ChoicesWithFriends(
            repeat: u8,
            choices: crate::r#ref::ManyChoices,
            repeat_max: u8,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u8::SIZE
                + crate::r#ref::ManyChoices::SIZE + u8::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005004;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            repeat.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            repeat_max.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Multiple enumeration parameter via Array
        pub fn ChoicesPrmPrmSet(
            val: crate::r#ref::ManyChoices,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ManyChoices::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005005;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Multiple enumeration parameter via Array
        pub fn ChoicesPrmPrmSave() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005006;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Too many choice command via Array
        pub fn ExtraChoices(
            choices: crate::r#ref::TooManyChoices,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::TooManyChoices::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005007;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Too many choices command via Array with a preceding and following argument
        pub fn ExtraChoicesWithFriends(
            repeat: u8,
            choices: crate::r#ref::TooManyChoices,
            repeat_max: u8,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u8::SIZE
                + crate::r#ref::TooManyChoices::SIZE + u8::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005008;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            repeat.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            repeat_max.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Too many enumeration parameter via Array
        pub fn ExtraChoicesPrmPrmSet(
            val: crate::r#ref::ManyChoices,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ManyChoices::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005009;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Too many enumeration parameter via Array
        pub fn ExtraChoicesPrmPrmSave() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500A;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Multiple choice command via Structure
        pub fn ChoicePair(choices: crate::r#ref::ChoicePair) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ChoicePair::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500B;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Multiple choices command via Structure with a preceding and following argument
        pub fn ChoicePairWithFriends(
            repeat: u8,
            choices: crate::r#ref::ChoicePair,
            repeat_max: u8,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u8::SIZE
                + crate::r#ref::ChoicePair::SIZE + u8::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500C;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            repeat.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            repeat_max.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Multiple enumeration parameter via Structure
        pub fn ChoicePairPrmPrmSet(
            val: crate::r#ref::ChoicePair,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ChoicePair::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500D;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Multiple enumeration parameter via Structure
        pub fn ChoicePairPrmPrmSave() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500E;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Multiple choice command via Complex Structure
        pub fn GluttonOfChoice(
            choices: crate::r#ref::ChoiceSlurry,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ChoiceSlurry::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500F;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Multiple choices command via Complex Structure with a preceding and following argument
        pub fn GluttonOfChoiceWithFriends(
            repeat: u8,
            choices: crate::r#ref::ChoiceSlurry,
            repeat_max: u8,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u8::SIZE
                + crate::r#ref::ChoiceSlurry::SIZE + u8::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005010;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            repeat.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
            repeat_max.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Multiple enumeration parameter via Complex Structure
        pub fn GluttonOfChoicePrmPrmSet(
            val: crate::r#ref::ChoiceSlurry,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ChoiceSlurry::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005011;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Multiple enumeration parameter via Complex Structure
        pub fn GluttonOfChoicePrmPrmSave() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005012;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Dump the typed parameters
        pub fn DumpTypedParameters() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005013;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Dump the float values
        pub fn DumpFloats() -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005014;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
        /// Send scalars
        pub fn SendScalars(
            scalar_input: crate::r#ref::ScalarStruct,
        ) -> crate::fw::CmdResponse {
            use fprime_core::Serializable;
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ScalarStruct::SIZE] = unsafe {
                core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005015;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            scalar_input.serialize_to(&mut __encoded, &mut __offset);
            crate::fw::CmdResponse::Ok
        }
    }
}
pub mod svc {
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
