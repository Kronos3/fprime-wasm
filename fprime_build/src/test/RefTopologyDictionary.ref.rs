#[allow(unused_imports)]
use fprime_core::*;
const __SCRATCH_SIZE: usize = 500usize;
static mut __SCRATCH: [u8; __SCRATCH_SIZE] = [0x0; __SCRATCH_SIZE];
static mut __TIME: [u8; fw::TimeValue::SIZE] = [0; fw::TimeValue::SIZE];
/// The type of a telemetry channel identifier
pub type FwChanIdType = crate::FwIdType;
/// The unsigned type of larger sizes internal to the software,
/// e.g., memory buffer sizes, file sizes. Must be unsigned.
pub type FwSizeType = crate::PlatformSizeType;
/// The id type.
pub type FwIdType = u32;
/// The type of smaller indices internal to the software, used
/// for array indices, e.g., port indices. Must be signed.
pub type PlatformIndexType = i16;
/// Define enumeration for Time base types
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
#[repr(u16)]
pub enum TimeBase {
    /// No time base has been established (Required)
    TbNone = 0,
    /// Indicates time is processor cycle time. Not tied to external time
    TbProcTime = 1,
    /// Time as reported on workstation where software is running. For testing. (Required)
    TbWorkstationTime = 2,
    /// Time as reported by the spacecraft clock.
    TbScTime = 3,
    /// Don't care value for sequences. If FwTimeBaseStoreType is changed, value should be changed (Required)
    TbDontCare = 65535,
}
/// The type of an event identifier
pub type FwEventIdType = crate::FwIdType;
/// The type used to serialize a time base value
pub type FwTimeBaseStoreType = u16;
/// The type of smaller indices internal to the software, used
/// for array indices, e.g., port indices. Must be signed.
pub type FwIndexType = crate::PlatformIndexType;
/// The type of a parameter identifier
pub type FwPrmIdType = crate::FwIdType;
/// The width of packet descriptors when they are serialized by the framework
pub type FwPacketDescriptorType = u16;
/// The unsigned type of larger sizes internal to the software,
/// e.g., memory buffer sizes, file sizes. Must be unsigned.
/// Supplied by platform, overridable by project.
pub type PlatformSizeType = u64;
/// The type of a data product identifier
pub type FwDpIdType = crate::FwIdType;
/// The type used to serialize a size value
pub type FwSizeStoreType = u16;
/// The type used to serialize a time context value
pub type FwTimeContextStoreType = u8;
/// The type of a telemetry packet identifier
pub type FwTlmPacketizeIdType = u16;
/// The type of a command opcode
pub type FwOpcodeType = crate::FwIdType;
/// The type of a data product priority
pub type FwDpPriorityType = u32;
pub mod com_cfg {
    #[allow(unused_imports)]
    use fprime_core::*;
    #[allow(unused_imports)]
    use fprime_core::*;
    /// APIDs are 11 bits in the Space Packet protocol, so we use U16. Max value 7FF
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u16)]
    pub enum Apid {
        /// Command packet type - incoming
        FwPacketCommand = 0,
        /// Telemetry packet type - outgoing
        FwPacketTelem = 1,
        /// Log type - outgoing
        FwPacketLog = 2,
        /// File type - incoming and outgoing
        FwPacketFile = 3,
        /// Packetized telemetry packet type
        FwPacketPacketizedTlm = 4,
        /// Data Product packet type
        FwPacketDp = 5,
        /// F Prime idle
        FwPacketIdle = 6,
        /// F Prime handshake
        FwPacketHand = 254,
        /// F Prime unknown packet
        FwPacketUnknown = 255,
        /// Per Space Packet Standard, all 1s (11bits) is reserved for Idle Packets
        SppIdlePacket = 2047,
        /// Anything equal or higher value is invalid and should not be used
        InvalidUninitialized = 2048,
    }
    /// Packet Version Numbers are 3 bits with only 2 currently valid values
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum Pvn {
        /// Fully Featured CCSDS Space Packet Protocol
        SpacePacketProtocol = 0,
        /// Bare-bones CCSDS Encapsulation Packet Protocol
        EncapsulationPacketProtocol = 7,
        /// Anything equal or higher value is invalid and should not be used
        InvalidUninitialized = 8,
    }
}
pub mod fw {
    #[allow(unused_imports)]
    use fprime_core::*;
    #[allow(unused_imports)]
    use fprime_core::*;
    /// Data structure for Time Interval
    #[derive(Clone, Debug, Serializable)]
    pub struct TimeIntervalValue {
        /// seconds portion of TimeInterval
        pub seconds: u32,
        /// microseconds portion of TimeInterval
        pub useconds: u32,
    }
    /// Deserialization status
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
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
    /// Enabled and disabled states
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum Enabled {
        /// Disabled state
        Disabled = 0,
        /// Enabled state
        Enabled = 1,
    }
    /// Enum representing parameter validity
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum ParamValid {
        Uninit = 0,
        Valid = 1,
        Invalid = 2,
        Default = 3,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(i32)]
    pub enum TimeComparison {
        Lt = -1,
        Eq = 0,
        Gt = 1,
        Incomparable = 2,
    }
    /// Wait or don't wait for something
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum Wait {
        /// Wait for something
        Wait = 0,
        /// Don't wait for something
        NoWait = 1,
    }
    /// Data structure for Time
    #[derive(Clone, Debug, Serializable)]
    pub struct TimeValue {
        /// basis of time (defined by system)
        pub time_base: crate::TimeBase,
        /// user settable value. Could be reboot count, node, etc
        pub time_context: crate::FwTimeContextStoreType,
        /// seconds portion of Time
        pub seconds: u32,
        /// microseconds portion of Time
        pub useconds: u32,
    }
    /// Enum representing event severity
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum LogSeverity {
        /// A fatal non-recoverable event
        Fatal = 1,
        /// A serious but recoverable event
        WarningHi = 2,
        /// A less serious but recoverable event
        WarningLo = 3,
        /// An activity related to commanding
        Command = 4,
        /// Important informational events
        ActivityHi = 5,
        /// Less important informational events
        ActivityLo = 6,
        /// Software diagnostic events
        Diagnostic = 7,
    }
    /// FPP shadow-enum representing Fw::FormatStatus
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum StringFormatStatus {
        /// Format worked
        Success = 0,
        /// Format overflowed
        Overflowed = 1,
        /// Format provided invalid format string
        InvalidFormatString = 2,
        /// FwSizeType overflowed the range of size_t
        SizeOverflow = 3,
        /// An error was returned from an underlying call
        OtherError = 4,
    }
    /// Enum representing a command response
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
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
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
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
    pub mod dp_cfg {
        #[allow(unused_imports)]
        use fprime_core::*;
        #[allow(unused_imports)]
        use fprime_core::*;
        /// A bit mask for selecting the type of processing to perform on
        /// a container before writing it to disk.
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum ProcType {
            /// No Processing
            ProcTypeNone = 0,
            /// Processing type 0
            ProcTypeZlibDeflate = 1,
            /// Processing type 1
            ProcTypeOne = 2,
            /// Processing type 2
            ProcTypeTwo = 4,
        }
    }
}
pub mod os {
    #[allow(unused_imports)]
    use fprime_core::*;
    #[allow(unused_imports)]
    use fprime_core::*;
    /// FPP shadow-enum representing Os::RawTime::Status
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum RawTimeStatus {
        /// Operation was successful
        OpOk = 0,
        /// Operation result caused an overflow
        OpOverflow = 1,
        /// Parameters invalid for current platform
        InvalidParams = 2,
        /// RawTime feature is not supported
        NotSupported = 3,
        /// All other errors
        OtherError = 4,
    }
}
pub mod r#ref {
    #[allow(unused_imports)]
    use fprime_core::*;
    #[allow(unused_imports)]
    use fprime_core::*;
    /// Packet receive status
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(i32)]
    pub enum PacketRecvStatus {
        PacketStateNoPackets = 0,
        PacketStateOk = 1,
        /// Receiver has seen errors
        PacketStateErrors = 3,
    }
    /// Some Packet Statistics
    #[derive(Clone, Debug, Serializable)]
    pub struct PacketStat {
        /// Number of buffers received
        pub buff_recv: u32,
        /// Number of buffers received with errors
        pub buff_err: u32,
        /// Packet Status
        pub packet_status: crate::Ref::PacketRecvStatus,
    }
    /// Array of array
    pub type TooManyChoices = [crate::Ref::ManyChoices; 2];
    /// Enumeration array
    pub type ManyChoices = [crate::Ref::Choice; 2];
    /// Structure of enums
    #[derive(Clone, Debug, Serializable)]
    pub struct ChoicePair {
        /// The first choice to make
        pub first_choice: crate::Ref::Choice,
        /// The second choice to make
        pub second_choice: crate::Ref::Choice,
    }
    #[derive(Clone, Debug, Serializable)]
    pub struct SignalInfo {
        pub r#type: crate::Ref::SignalType,
        pub history: crate::Ref::SignalSet,
        pub pair_history: crate::Ref::SignalPairSet,
    }
    /// Set of floating points to emit
    pub type FloatSet = [f32; 3];
    pub type SignalPairSet = [crate::Ref::SignalPair; 4];
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(i32)]
    pub enum SignalType {
        Triangle = 0,
        Square = 1,
        Sine = 2,
        Noise = 3,
    }
    pub type SignalSet = [f32; 4];
    /// Enumeration type for use later
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(i32)]
    pub enum Choice {
        One = 0,
        Two = 1,
        Red = 2,
        Blue = 3,
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
    #[derive(Clone, Debug, Serializable)]
    pub struct SignalPair {
        pub time: f32,
        pub value: f32,
    }
    /// Structure of enums (with an multi-dimensional array and structure)
    #[derive(Clone, Debug, Serializable)]
    pub struct ChoiceSlurry {
        /// A large set of disorganized choices
        pub too_many_choices: crate::Ref::TooManyChoices,
        /// A singular choice
        pub separate_choice: crate::Ref::Choice,
        /// A pair of choices
        pub choice_pair: crate::Ref::ChoicePair,
        /// An array of choices defined as member array
        pub choice_as_member_array: [u8; 2],
    }
    pub mod dp_demo {
        #[allow(unused_imports)]
        use fprime_core::*;
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Array of strings
        pub type StringArray = [String<80>; 2];
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(i32)]
        pub enum ColorEnum {
            Red = 0,
            Green = 1,
            Blue = 2,
        }
        pub type I32Alias = i32;
        pub type BoolAlias = bool;
        #[derive(Clone, Debug, Serializable)]
        pub struct ColorInfoStruct {
            pub color: crate::Ref::DpDemo::ColorEnum,
        }
        /// Array of booleans
        pub type BooleanArray = [bool; 2];
        pub type ArrayOfStructs = [crate::Ref::DpDemo::StructWithStringMembers; 3];
        /// Array of floats
        pub type F32Array = [f32; 3];
        #[derive(Clone, Debug, Serializable)]
        pub struct StructWithStringMembers {
            pub string_member: String<80>,
            pub string_array_member: crate::Ref::DpDemo::StringArray,
        }
        /// Array of array of strings
        pub type ArrayOfStringArray = [crate::Ref::DpDemo::StringArray; 3];
        pub type StringAlias = String<80>;
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(i32)]
        pub enum DpReqType {
            Immediate = 0,
            Async = 1,
        }
        /// Array of enumerations
        pub type EnumArray = [crate::Ref::DpDemo::ColorEnum; 3];
        /// Array of integers
        pub type U32Array = [u32; 5];
        #[derive(Clone, Debug, Serializable)]
        pub struct StructWithEverything {
            pub integer_member: crate::Ref::DpDemo::I32Alias,
            pub float_member: f32,
            pub string_member: String<80>,
            pub boolean_member: bool,
            pub enum_member: crate::Ref::DpDemo::ColorEnum,
            pub array_member_u_32: [crate::Ref::DpDemo::U32Array; 2],
            pub f_32_array: crate::Ref::DpDemo::F32Array,
            pub u_32_array: crate::Ref::DpDemo::U32Array,
            pub enum_array: crate::Ref::DpDemo::EnumArray,
            pub string_array: crate::Ref::DpDemo::StringArray,
            pub boolean_array: crate::Ref::DpDemo::BooleanArray,
            pub struct_with_strings: crate::Ref::DpDemo::StructWithStringMembers,
            pub nested_arrays: crate::Ref::DpDemo::ArrayOfStringArray,
        }
        pub type F64Alias = f64;
    }
    pub mod send_buff {
        #[allow(unused_imports)]
        use fprime_core::*;
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Active state
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(i32)]
        pub enum ActiveState {
            SendIdle = 0,
            SendActive = 1,
        }
    }
    pub mod signal_gen {
        #[allow(unused_imports)]
        use fprime_core::*;
        #[allow(unused_imports)]
        use fprime_core::*;
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(i32)]
        pub enum DpReqType {
            Immediate = 0,
            Async = 1,
        }
    }
}
pub mod svc {
    #[allow(unused_imports)]
    use fprime_core::*;
    #[allow(unused_imports)]
    use fprime_core::*;
    /// Header validation error
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum DpHdrField {
        Descriptor = 0,
        Id = 1,
        Priority = 2,
        Crc = 3,
    }
    /// Array of queue depths for Fw::Buffer types
    pub type BuffQueueDepth = [u32; 1];
    /// An enumeration for Version Type
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
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
    #[derive(Clone, Debug, Serializable)]
    pub struct CompressionMetadata {
        pub algorithm: crate::Svc::CompressionAlgorithm,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum SystemResourceEnabled {
        Disabled = 0,
        Enabled = 1,
    }
    /// Data Structure for custom version Tlm
    #[derive(Clone, Debug, Serializable)]
    pub struct CustomVersionDb {
        /// enumeration/name of the custom version
        pub version_enum: crate::Svc::VersionCfg::VersionEnum,
        /// string containing custom version
        pub version_value: String<80>,
        /// status of the custom version
        pub version_status: crate::Svc::VersionStatus,
    }
    /// An enumeration for version status
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum VersionStatus {
        /// Version was good
        Ok = 0,
        /// Failure to get version
        Failure = 1,
    }
    /// Array of queue depths for Fw::Com types
    pub type ComQueueDepth = [u32; 2];
    /// Sequencer blocking state
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum BlockState {
        Block = 0,
        NoBlock = 1,
    }
    /// An enumeration of queue data types
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum QueueType {
        ComQueue = 0,
        BufferQueue = 1,
    }
    /// Send file status enum
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum SendFileStatus {
        StatusOk = 0,
        StatusError = 1,
        StatusInvalid = 2,
        StatusBusy = 3,
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
        pub state: crate::Fw::DpState,
    }
    /// Tracks versions for project, framework and user defined versions etc
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum VersionEnabled {
        /// verbosity disabled
        Disabled = 0,
        /// verbosity enabled
        Enabled = 1,
    }
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum CompressionAlgorithm {
        Uncompressed = 0,
        ZlibDeflate = 1,
    }
    pub mod buffer_accumulator {
        #[allow(unused_imports)]
        use fprime_core::*;
        #[allow(unused_imports)]
        use fprime_core::*;
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum OpState {
            Accumulate = 0,
            Drain = 1,
        }
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum BlockMode {
            Noblock = 0,
            Block = 1,
        }
    }
    pub mod ccsds {
        #[allow(unused_imports)]
        use fprime_core::*;
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Protocol IDs for EPP encapsulation packets per CCSDS 133.1-B-3 Section 4.1.2.3.3
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum EppProtocolId {
            /// 0b000 - Encapsulation Idle Packet
            Idle = 0,
            /// 0b110 - Extended Protocol ID Field Driven
            Extended = 6,
            /// Mission-specific
            MissionSpecific = 7,
        }
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum EppLengthOfLength {
            /// 0b00 - Single Byte Idle Packet
            Zero = 0,
            /// 0b01 - Two Byte Header
            One = 1,
            /// 0b10 - Four Byte Header
            Two = 2,
            /// 0b11 - Eight Byte Header
            Four = 3,
        }
        /// Transfer Frame Version Numbers are 4 bits long
        /// CCSDS References add 1 to the binary value when counting versions in english (e.g. TM & TC are "version 1" but 0b00)
        /// Until the release of USLP, TFVN were 2 bits long
        /// With the addition of USLP two bits were added to the trailing end that serve as the 2 most significant bits
        /// (i.e. on the wire USLP sends 0b1100)
        /// See section 3.2.2.2 of USLP Overview (CCSDS 700.1-G-1),
        /// section 4.1.2.2.2 of AOS (CCSDS 732.0-B-5),
        /// section 4.3.2 of Space Data Link Protocol Summary (CCSDS 130.2-G-3),
        /// & table 3-1 in Overview of Space Comms Protocols (CCSDS 130.0-G-4)
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum Tfvn {
            /// Telemetry and Telecommand SDLs
            TmTc = 0,
            /// Advanced Orbiting Systems SDL
            Aos = 1,
            /// Proximity-1 SDL
            ProxOne = 2,
            /// Unified Space Data Link Protocol
            Uslp = 3,
            /// Anything equal or higher value is invalid and should not be used
            InvalidUninitialized = 4,
        }
    }
    pub mod cmd_sequencer {
        #[allow(unused_imports)]
        use fprime_core::*;
        #[allow(unused_imports)]
        use fprime_core::*;
        /// The stage of the file read operation
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
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
        /// The sequencer mode
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum SeqMode {
            Step = 0,
            Auto = 1,
        }
    }
    pub mod event_manager {
        #[allow(unused_imports)]
        use fprime_core::*;
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Enabled and disabled state
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum Enabled {
            /// Enabled state
            Enabled = 0,
            /// Disabled state
            Disabled = 1,
        }
        /// Severity level for event filtering
        /// Similar to Fw::LogSeverity, but no FATAL event
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
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
    }
    pub mod fpy {
        #[allow(unused_imports)]
        use fprime_core::*;
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Serial port indices for FpySequencer serialOut port array.
        /// MAX_SERIAL_PORTS must be defined with this exact name for Fpy compiler bounds checking.
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum SerialPortIndex {
            /// Example serial port 0 - rename to application-specific name (e.g., TIME_SYNC_PORT)
            ExamplePort0 = 0,
            /// Example serial port 1 - rename to application-specific name (e.g., SENSOR_DATA_PORT)
            ExamplePort1 = 1,
            /// Example serial port 2 - rename to application-specific name
            ExamplePort2 = 2,
            /// Example serial port 3 - rename to application-specific name
            ExamplePort3 = 3,
            /// Example serial port 4 - rename to application-specific name
            ExamplePort4 = 4,
            /// REQUIRED: Maximum number of serial ports. This sentinel value MUST be named
            MaxSerialPorts = 5,
        }
    }
    pub mod prm_db {
        #[allow(unused_imports)]
        use fprime_core::*;
        #[allow(unused_imports)]
        use fprime_core::*;
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum PrmLoadAction {
            SetParameter = 0,
            SaveFileCommand = 1,
            LoadFileCommand = 2,
            CommitStagedCommand = 3,
        }
        /// State of parameter DB file load operations
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum PrmDbFileLoadState {
            Idle = 0,
            LoadingFileUpdates = 1,
            FileUpdatesStaged = 2,
        }
        /// Parameter write error
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
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
            CrcPlace = 9,
            CrcReal = 10,
            CurrPosition = 11,
            SeekZero = 12,
            SeekPosition = 13,
        }
        /// Parameter read error
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
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
            Crc = 11,
            CrcSize = 12,
            CrcBuffer = 13,
            SeekZero = 14,
        }
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum Merge {
            Merge = 0,
            Reset = 1,
        }
    }
    pub mod version_cfg {
        #[allow(unused_imports)]
        use fprime_core::*;
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Define a set of Version entries on a project-specific
        /// basis.
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
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
    pub mod wasm_sequencer {
        #[allow(unused_imports)]
        use fprime_core::*;
        #[allow(unused_imports)]
        use fprime_core::*;
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum AllocError {
            /// Not enough pages could be allocated to accommodate this allocation
            OutOfMemory = 0,
            /// Page was too small to fit this allocation
            PageTooSmall = 1,
            /// A generic allocation failure
            AllocationFailed = 2,
        }
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(i32)]
        pub enum Status {
            Ok = 0,
            ErrNullArg = 1,
            ErrBadArg = 2,
            ErrBadUtf8 = 3,
            ErrNameTooLong = 4,
            ErrBadSignature = 5,
            ErrCapacity = 6,
            ErrNotFound = 7,
            ErrWrongState = 8,
            ErrGuestMemoryAllocFailed = 15,
            ErrAllocFailed = 16,
            ErrOutOfMemory = 17,
            ErrPageTooSmall = 18,
            ErrMemOutOfBounds = 32,
            ErrParamLenMismatch = 48,
            ErrParamTypeMismatch = 49,
            ErrStackOverflow = 50,
            ErrEof = 64,
            ErrMalformedInteger = 65,
            ErrI33IsNegative = 66,
            ErrMalformedMagic = 67,
            ErrMalformedVersion = 68,
            ErrMalformedUtf8 = 69,
            ErrDuplicateModuleName = 70,
            ErrDuplicateExportName = 71,
            ErrMalformedSectionId = 72,
            ErrMalformedValueType = 73,
            ErrMalformedFunction = 74,
            ErrMalformedLimit = 75,
            ErrMalformedElemType = 76,
            ErrMalformedSectionSize = 77,
            ErrExpectedConstOrVar = 78,
            ErrMalformedImportExportDesc = 79,
            ErrMalformedMemType = 80,
            ErrInvalidPageSize = 81,
            ErrInvalidSectionOrdering = 82,
            ErrDuplicateSection = 83,
            ErrInvalidMaxLimit = 84,
            ErrExpectedTerminal = 85,
            ErrInvalidOpcode = 86,
            ErrMalformedCodeSize = 87,
            ErrInvalidCodeSectionFunctionCount = 88,
            ErrVecTooLong = 89,
            ErrIdxTooLarge = 90,
            ErrModuleIdxTooLarge = 91,
            ErrMemoryTooLarge = 92,
            ErrMemoryImportTooLarge = 93,
            ErrMemAlignTooLarge = 94,
            ErrControlFlowTooDeep = 96,
            ErrStackUnderflow = 97,
            ErrStackTooLarge = 98,
            ErrLabelStackJumpTooDeep = 99,
            ErrLabelJumpTooLarge = 100,
            ErrTypeMismatch = 101,
            ErrBlockResultTypeMismatch = 102,
            ErrFunctionResultTypeMismatch = 103,
            ErrIllegalMemoryGrow = 112,
            ErrInvalidElementOffset = 113,
            ErrInvalidElementOutOfBounds = 114,
            ErrInvalidTableIndex = 115,
            ErrTableNotDefined = 116,
            ErrInvalidElementCount = 117,
            ErrInvalidMemIndex = 118,
            ErrMemoryNotDefined = 119,
            ErrInvalidMemOffsetType = 120,
            ErrInvalidNegativeMemOffset = 121,
            ErrInvalidMemOffset = 122,
            ErrMultipleMemories = 123,
            ErrMultipleTables = 124,
            ErrInvalidLabelIndex = 128,
            ErrInvalidElseBlock = 129,
            ErrInvalidEndBlock = 130,
            ErrInstructionOutsideOfFunction = 131,
            ErrLocalIdxOutOfRange = 132,
            ErrFunctionIdxOutOfRange = 133,
            ErrTypeIdxOutOfRange = 134,
            ErrFunctionTextOutOfRange = 135,
            ErrGlobalIdxOutOfRange = 136,
            ErrFunctionImportNotFound = 144,
            ErrGlobalImportNotFound = 145,
            ErrMemoryImportNotFound = 146,
            ErrTableImportNotFound = 147,
            ErrFunctionImportOutOfRange = 148,
            ErrFunctionImportTypeMismatch = 149,
            ErrGlobalIsNotMutable = 150,
            ErrGlobalImportTypeMismatch = 151,
            ErrMemoryImportTypeMismatch = 152,
            ErrTableImportTypeMismatch = 153,
            ErrTableImportIncompatibleSize = 154,
            ErrFunctionParametersTooLarge = 160,
            ErrFunctionReturnsTooLarge = 161,
            ErrTooManyLocals = 162,
            ErrInvalidConstInstruction = 163,
            ErrGlobalTypeMismatch = 164,
            ErrAlignmentLargerThanType = 165,
            ErrInvalidStartFunctionSignature = 166,
            ErrConstAlreadyHasValue = 176,
            ErrConstNoValue = 177,
            ErrConstInvalidGlobal = 178,
            ErrPossibleBackpatchCycle = 192,
            ErrPageFault = 193,
            ErrReaderError = 194,
        }
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(i32)]
        pub enum TrapReason {
            /// Triggered by unreachable instruction
            Unreachable = 0,
            /// A host function has noted an unrecoverable failure
            Host = 1,
            /// Integer or floating point division by zero
            DivideByZero = 2,
            /// An indirect call tried to map to a table function out of range
            InvalidTableIndex = 3,
            /// The function type in an indirect call does not match the function pointer's type
            InvalidTableFunctionType = 4,
            /// An indirect call tried to map to a table function out of range
            UninitializedTableElement = 5,
            /// An imported global could not be read
            GlobalGetFailed = 6,
            /// An imported global could not be set
            GlobalSetFailed = 7,
            /// A memory operation is out of bounds
            OutOfMemory = 8,
            /// memory.grow failed because a host function has taken ownership of a memory
            MemoryRefNotUnique = 9,
            /// A memory operation is out of bounds
            MemoryOutOfBounds = 10,
            /// Ran out of stack space
            StackOverflow = 11,
            /// Attempting to convert Inf to integer
            UnrepresentableResult = 12,
            /// Signed division causes integer overflow
            IntegerOverflow = 13,
            /// Attempting to convert NaN to integer
            BadConversionToInteger = 14,
        }
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(i32)]
        pub enum HostFunction {
            Command = 0,
            Event = 1,
        }
        pub mod sequencer_state_machine {
            #[allow(unused_imports)]
            use fprime_core::*;
            #[allow(unused_imports)]
            use fprime_core::*;
            #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum State {
                /// The uninitialized state
                FprimeUninitialized = 0,
                /// Interpreter does not have any modules loaded or a store initialized
                Idle = 1,
                /// Interpreter is loading a Wasm module into the store
                Loading = 2,
                /// a host function paused the interpreter to dispatch an F´ command;
                /// waiting for the command response to come back in
                RunningAwaitingResponse = 3,
                RunningPaused = 4,
                /// sequencer is not taking any action, waiting for a time in the future to continue
                RunningSleeping = 5,
                /// sequencer is spinning the interpreter loop
                RunningSpinning = 6,
                /// A module finished loading and the start function needs to be invoked/run
                Starting = 7,
            }
        }
    }
}
struct CdhCoreCmdDisp {}
impl CdhCoreCmdDisp {
    /// No-op command
    pub fn cmd_no_op() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1000000;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// No-op string command
    ///
    ///  * `arg1` - The String command argument
    pub fn cmd_no_op_string(arg_1: &str) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1000001;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<40> as StrTruncate<40>>::truncate(arg_1)
            .serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// No-op command
    ///
    ///  * `arg1` - The I32 command argument
    ///  * `arg2` - The F32 command argument
    ///  * `arg3` - The U8 command argument
    pub fn cmd_test_cmd_1(arg_1: i32, arg_2: f32, arg_3: u8) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1000002;
        __opcode.serialize_to(__encoded, &mut __offset);
        arg_1.serialize_to(__encoded, &mut __offset);
        arg_2.serialize_to(__encoded, &mut __offset);
        arg_3.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Clear command tracking info to recover from components not returning status
    pub fn cmd_clear_tracking() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1000003;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Number of commands dispatched
    pub fn commands_dispatched() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1000000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Number of command errors
    pub fn command_errors() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1000001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Number of commands drooped due to buffer overflow
    pub fn commands_dropped() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1000002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct CdhCoreEvents {}
impl CdhCoreEvents {
    /// Set filter for reporting events. Events are not stored in component.
    ///
    ///  * `filterLevel` - Filter level
    ///  * `filterEnabled` - Filter state
    pub fn set_event_filter(
        filter_level: crate::Svc::EventManager::FilterSeverity,
        filter_enabled: crate::Svc::EventManager::Enabled,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1001000;
        __opcode.serialize_to(__encoded, &mut __offset);
        filter_level.serialize_to(__encoded, &mut __offset);
        filter_enabled.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Filter a particular ID
    ///
    ///  * `ID`
    ///  * `idFilterEnabled` - ID filter state
    pub fn set_id_filter(
        id: crate::FwEventIdType,
        id_filter_enabled: crate::Svc::EventManager::Enabled,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1001002;
        __opcode.serialize_to(__encoded, &mut __offset);
        id.serialize_to(__encoded, &mut __offset);
        id_filter_enabled.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Dump the filter states via events
    pub fn dump_filter_state() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1001003;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Number of events dropped due to queue full
    pub fn events_dropped() -> FprimeResult<(crate::FwSizeType, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::FwSizeType as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1001000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::FwSizeType as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct CdhCoreHealth {}
impl CdhCoreHealth {
    /// A command to enable or disable health checks
    ///
    ///  * `enable` - whether or not health checks are enabled
    pub fn hlth_enable(enable: crate::Fw::Enabled) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1002000;
        __opcode.serialize_to(__encoded, &mut __offset);
        enable.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Ignore a particular ping entry
    ///
    ///  * `entry` - The entry to enable/disable
    ///  * `enable` - whether or not a port is pinged
    pub fn hlth_ping_enable(
        entry: &str,
        enable: crate::Fw::Enabled,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1002001;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<40> as StrTruncate<40>>::truncate(entry)
            .serialize_to(__encoded, &mut __offset);
        enable.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Change ping value
    ///
    ///  * `entry` - The entry to modify
    ///  * `warningValue` - Ping warning threshold
    ///  * `fatalValue` - Ping fatal threshold
    pub fn hlth_chng_ping(
        entry: &str,
        warning_value: u32,
        fatal_value: u32,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1002002;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<40> as StrTruncate<40>>::truncate(entry)
            .serialize_to(__encoded, &mut __offset);
        warning_value.serialize_to(__encoded, &mut __offset);
        fatal_value.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Number of overrun warnings
    pub fn ping_late_warnings() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1002000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct CdhCoreVersion {}
impl CdhCoreVersion {
    /// A command to enable or disable Event verbosity and Telemetry
    ///
    ///  * `enable` - whether or not Version telemetry is enabled
    pub fn enable(enable: crate::Svc::VersionEnabled) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1003000;
        __opcode.serialize_to(__encoded, &mut __offset);
        enable.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Report version as Event
    ///
    ///  * `version_type` - which version type Event is requested
    pub fn version(version_type: crate::Svc::VersionType) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1003001;
        __opcode.serialize_to(__encoded, &mut __offset);
        version_type.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Software framework version
    pub fn framework_version() -> FprimeResult<(String<40>, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <String<40> as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <String<40> as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Software project version
    pub fn project_version() -> FprimeResult<(String<40>, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <String<40> as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <String<40> as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Custom Versions
    pub fn custom_version_01() -> FprimeResult<
        (crate::Svc::CustomVersionDb, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Svc::CustomVersionDb as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn custom_version_02() -> FprimeResult<
        (crate::Svc::CustomVersionDb, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003003, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Svc::CustomVersionDb as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn custom_version_03() -> FprimeResult<
        (crate::Svc::CustomVersionDb, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003004, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Svc::CustomVersionDb as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn custom_version_04() -> FprimeResult<
        (crate::Svc::CustomVersionDb, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003005, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Svc::CustomVersionDb as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn custom_version_05() -> FprimeResult<
        (crate::Svc::CustomVersionDb, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003006, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Svc::CustomVersionDb as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn custom_version_06() -> FprimeResult<
        (crate::Svc::CustomVersionDb, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003007, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Svc::CustomVersionDb as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn custom_version_07() -> FprimeResult<
        (crate::Svc::CustomVersionDb, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003008, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Svc::CustomVersionDb as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn custom_version_08() -> FprimeResult<
        (crate::Svc::CustomVersionDb, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003009, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Svc::CustomVersionDb as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn custom_version_09() -> FprimeResult<
        (crate::Svc::CustomVersionDb, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x100300A, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Svc::CustomVersionDb as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn custom_version_10() -> FprimeResult<
        (crate::Svc::CustomVersionDb, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x100300B, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Svc::CustomVersionDb as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Library Versions
    pub fn library_version_01() -> FprimeResult<(String<40>, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <String<40> as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x100300C, &mut time_buf, &mut value_buf) }?;
        Ok((
            <String<40> as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn library_version_02() -> FprimeResult<(String<40>, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <String<40> as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x100300D, &mut time_buf, &mut value_buf) }?;
        Ok((
            <String<40> as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn library_version_03() -> FprimeResult<(String<40>, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <String<40> as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x100300E, &mut time_buf, &mut value_buf) }?;
        Ok((
            <String<40> as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn library_version_04() -> FprimeResult<(String<40>, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <String<40> as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x100300F, &mut time_buf, &mut value_buf) }?;
        Ok((
            <String<40> as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn library_version_05() -> FprimeResult<(String<40>, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <String<40> as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003010, &mut time_buf, &mut value_buf) }?;
        Ok((
            <String<40> as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn library_version_06() -> FprimeResult<(String<40>, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <String<40> as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003011, &mut time_buf, &mut value_buf) }?;
        Ok((
            <String<40> as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn library_version_07() -> FprimeResult<(String<40>, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <String<40> as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003012, &mut time_buf, &mut value_buf) }?;
        Ok((
            <String<40> as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn library_version_08() -> FprimeResult<(String<40>, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <String<40> as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003013, &mut time_buf, &mut value_buf) }?;
        Ok((
            <String<40> as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn library_version_09() -> FprimeResult<(String<40>, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <String<40> as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003014, &mut time_buf, &mut value_buf) }?;
        Ok((
            <String<40> as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    pub fn library_version_10() -> FprimeResult<(String<40>, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <String<40> as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1003015, &mut time_buf, &mut value_buf) }?;
        Ok((
            <String<40> as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct CdhCore {
    cmd_disp: CdhCoreCmdDisp,
    events: CdhCoreEvents,
    health: CdhCoreHealth,
    version: CdhCoreVersion,
}
impl CdhCore {}
struct ComCcsdsComQueue {}
impl ComCcsdsComQueue {
    /// Flush a specific queue. This will discard all queued data in the specified queue removing it from eventual
    /// downlink. Buffers requiring ownership return will be returned via the bufferReturnOut port.
    ///
    ///  * `queueType` - The Queue data type
    ///  * `indexType` - The index of the queue (within the supplied type) to flush
    pub fn flush_queue(
        queue_type: crate::Svc::QueueType,
        index_type: crate::FwIndexType,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x2000000;
        __opcode.serialize_to(__encoded, &mut __offset);
        queue_type.serialize_to(__encoded, &mut __offset);
        index_type.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Flush all queues. This will discard all queued data removing it from eventual downlink. Buffers requiring
    /// ownership return will be returned via the bufferReturnOut port.
    pub fn flush_all_queues() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x2000001;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Set the priority of a specific queue at runtime
    ///
    ///  * `queueType` - The Queue data type
    ///  * `indexType` - The index of the queue (within the supplied type) to modify
    ///  * `newPriority` - New priority value for the queue
    pub fn set_queue_priority(
        queue_type: crate::Svc::QueueType,
        index_type: crate::FwIndexType,
        new_priority: crate::FwIndexType,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x2000002;
        __opcode.serialize_to(__encoded, &mut __offset);
        queue_type.serialize_to(__encoded, &mut __offset);
        index_type.serialize_to(__encoded, &mut __offset);
        new_priority.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Depth of queues of Fw::ComBuffer type
    pub fn com_queue_depth() -> FprimeResult<
        (crate::Svc::ComQueueDepth, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Svc::ComQueueDepth as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x2000000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Svc::ComQueueDepth as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Depth of queues of Fw::Buffer type
    pub fn buff_queue_depth() -> FprimeResult<
        (crate::Svc::BuffQueueDepth, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Svc::BuffQueueDepth as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x2000001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Svc::BuffQueueDepth as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct ComCcsdsCommsBufferManager {}
impl ComCcsdsCommsBufferManager {
    /// The total buffers allocated
    pub fn total_buffs() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x2002000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The current number of allocated buffers
    pub fn curr_buffs() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x2002001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The high water mark of allocated buffers
    pub fn hi_buffs() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x2002002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The number of requests that couldn't return a buffer
    pub fn no_buffs() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x2002003, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The number of empty buffers returned
    pub fn empty_buffs() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x2002004, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct ComCcsds {
    com_queue: ComCcsdsComQueue,
    comms_buffer_manager: ComCcsdsCommsBufferManager,
}
impl ComCcsds {}
struct DataProductsDpBufferAccumulator {}
impl DataProductsDpBufferAccumulator {
    /// Set the mode
    ///
    ///  * `mode`
    pub fn ba_set_mode(
        mode: crate::Svc::BufferAccumulator::OpState,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x4004000;
        __opcode.serialize_to(__encoded, &mut __offset);
        mode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Drain the commanded number of buffers
    ///
    ///  * `numToDrain`
    ///  * `blockMode`
    pub fn ba_drain_buffers(
        num_to_drain: u32,
        block_mode: crate::Svc::BufferAccumulator::BlockMode,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x4004001;
        __opcode.serialize_to(__encoded, &mut __offset);
        num_to_drain.serialize_to(__encoded, &mut __offset);
        block_mode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// The number of buffers queued
    pub fn ba_num_queued_buffers() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4004000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct DataProductsDpBufferManager {}
impl DataProductsDpBufferManager {
    /// The total buffers allocated
    pub fn total_buffs() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4003000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The current number of allocated buffers
    pub fn curr_buffs() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4003001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The high water mark of allocated buffers
    pub fn hi_buffs() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4003002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The number of requests that couldn't return a buffer
    pub fn no_buffs() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4003003, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The number of empty buffers returned
    pub fn empty_buffs() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4003004, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct DataProductsDpCat {}
impl DataProductsDpCat {
    /// Build catalog from data product directory. Will block until complete
    pub fn build_catalog() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x4000000;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Start transmitting catalog
    ///
    ///  * `wait` - have START_XMIT command complete wait for catalog to complete transmitting
    ///  * `remainActive` - should the catalog resume transmission when Dps are added at runtime
    pub fn start_xmit_catalog(
        wait: crate::Fw::Wait,
        remain_active: bool,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x4000001;
        __opcode.serialize_to(__encoded, &mut __offset);
        wait.serialize_to(__encoded, &mut __offset);
        remain_active.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Stop transmitting catalog
    pub fn stop_xmit_catalog() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x4000002;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// clear existing catalog
    pub fn clear_catalog() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x4000003;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Number of data products in catalog
    pub fn catalog_dps() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4000000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Number of data products sent
    pub fn dps_sent() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4000001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct DataProductsDpMgr {}
impl DataProductsDpMgr {
    /// Clear event throttling
    pub fn clear_event_throttle() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x4001000;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// The number of successful buffer allocations
    pub fn num_successful_allocations() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4001000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The number of failed buffer allocations
    pub fn num_failed_allocations() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4001001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Number of data products handled
    pub fn num_data_products() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4001002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Number of bytes handled
    pub fn num_bytes() -> FprimeResult<(u64, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u64 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4001003, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u64 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct DataProductsDpWriter {}
impl DataProductsDpWriter {
    /// Clear event throttling
    pub fn clear_event_throttle() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x4002000;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// The number of buffers received
    pub fn num_buffers_received() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4002000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The number of bytes written
    pub fn num_bytes_written() -> FprimeResult<(u64, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u64 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4002001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u64 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The number of successful writes
    pub fn num_successful_writes() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4002002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The number of failed writes
    pub fn num_failed_writes() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4002003, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The number of errors
    pub fn num_errors() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x4002004, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct DataProducts {
    dp_buffer_accumulator: DataProductsDpBufferAccumulator,
    dp_buffer_manager: DataProductsDpBufferManager,
    dp_cat: DataProductsDpCat,
    dp_mgr: DataProductsDpMgr,
    dp_writer: DataProductsDpWriter,
}
impl DataProducts {}
struct FileHandlingFileDownlink {}
impl FileHandlingFileDownlink {
    /// Read a named file off the disk. Divide it into packets and send the packets for transmission to the ground.
    ///
    ///  * `sourceFileName` - The name of the on-board file to send
    ///  * `destFileName` - The name of the destination file on the ground
    pub fn send_file(
        source_file_name: &str,
        dest_file_name: &str,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x5001000;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<100> as StrTruncate<100>>::truncate(source_file_name)
            .serialize_to(__encoded, &mut __offset);
        <String<100> as StrTruncate<100>>::truncate(dest_file_name)
            .serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Cancel the downlink in progress, if any
    pub fn cancel() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x5001001;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Read a named file off the disk from a starting position. Divide it into packets and send the packets for transmission to the ground.
    ///
    ///  * `sourceFileName` - The name of the on-board file to send
    ///  * `destFileName` - The name of the destination file on the ground
    ///  * `startOffset` - Starting offset of the source file
    ///  * `length` - Number of bytes to send from starting offset. Length of 0 implies until the end of the file
    pub fn send_partial(
        source_file_name: &str,
        dest_file_name: &str,
        start_offset: u32,
        length: u32,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x5001002;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<100> as StrTruncate<100>>::truncate(source_file_name)
            .serialize_to(__encoded, &mut __offset);
        <String<100> as StrTruncate<100>>::truncate(dest_file_name)
            .serialize_to(__encoded, &mut __offset);
        start_offset.serialize_to(__encoded, &mut __offset);
        length.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// The total number of files sent
    pub fn files_sent() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x5001000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The total number of packets sent
    pub fn packets_sent() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x5001001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The total number of warnings
    pub fn warnings() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x5001002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct FileHandlingFileManager {}
impl FileHandlingFileManager {
    /// Create a directory
    ///
    ///  * `dirName` - The directory to create
    pub fn create_directory(dir_name: &str) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x5002000;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(dir_name)
            .serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Move a file
    ///
    ///  * `sourceFileName` - The source file name
    ///  * `destFileName` - The destination file name
    pub fn move_file(
        source_file_name: &str,
        dest_file_name: &str,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x5002001;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(source_file_name)
            .serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(dest_file_name)
            .serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Remove a directory, which must be empty
    ///
    ///  * `dirName` - The directory to remove
    pub fn remove_directory(dir_name: &str) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x5002002;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(dir_name)
            .serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Remove a file
    ///
    ///  * `fileName` - The file to remove
    ///  * `ignoreErrors` - Ignore nonexistent files
    pub fn remove_file(file_name: &str, ignore_errors: bool) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x5002003;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(file_name)
            .serialize_to(__encoded, &mut __offset);
        ignore_errors.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Append 1 file's contents to the end of another.
    ///
    ///  * `source` - The name of the file to take content from
    ///  * `target` - The name of the file to append to
    pub fn append_file(source: &str, target: &str) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x5002005;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(source)
            .serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(target)
            .serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Get the size of a file
    ///
    ///  * `fileName` - The file to get the size of
    pub fn file_size(file_name: &str) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x5002006;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(file_name)
            .serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// List the contents of a directory
    ///
    ///  * `dirName` - The directory to list
    pub fn list_directory(dir_name: &str) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x5002007;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(dir_name)
            .serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Calculate the CRC of a file
    ///
    ///  * `filename` - The file to CRC
    pub fn calculate_crc(filename: &str) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x5002008;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(filename)
            .serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// The total number of commands successfully executed
    pub fn commands_executed() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x5002000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The total number of errors
    pub fn errors() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x5002001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct FileHandlingFileUplink {}
impl FileHandlingFileUplink {
    /// The total number of complete files received
    pub fn files_received() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x5000000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The total number of packets received
    pub fn packets_received() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x5000001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The total number of warnings issued
    pub fn warnings() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x5000002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct FileHandlingPrmDb {}
impl FileHandlingPrmDb {
    /// Command to save parameter image to file. Uses file name passed to constructor
    pub fn prm_save_file() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x5003000;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Loads a file from storage into the staging database. The file could have selective IDs and not the whole set.
    ///
    ///  * `fileName` - The name of the on-board file to set parameters from
    ///  * `merge` - Whether to merge or fully reset the parameter database from the file contents
    pub fn prm_load_file(
        file_name: &str,
        merge: crate::Svc::PrmDb::Merge,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x5003001;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(file_name)
            .serialize_to(__encoded, &mut __offset);
        merge.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Commits the backup database to become the prime (active) database
    pub fn prm_commit_staged() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x5003002;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
}
struct FileHandling {
    file_downlink: FileHandlingFileDownlink,
    file_manager: FileHandlingFileManager,
    file_uplink: FileHandlingFileUplink,
    prm_db: FileHandlingPrmDb,
}
impl FileHandling {}
struct RefSg1 {}
impl RefSg1 {
    /// Signal Generator Settings
    ///
    ///  * `Frequency`
    ///  * `Amplitude`
    ///  * `Phase`
    ///  * `SigType`
    pub fn settings(
        frequency: u32,
        amplitude: f32,
        phase: f32,
        sig_type: crate::Ref::SignalType,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10011000;
        __opcode.serialize_to(__encoded, &mut __offset);
        frequency.serialize_to(__encoded, &mut __offset);
        amplitude.serialize_to(__encoded, &mut __offset);
        phase.serialize_to(__encoded, &mut __offset);
        sig_type.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Toggle Signal Generator On/Off.
    pub fn toggle() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10011001;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Skip next sample
    pub fn skip() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10011002;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Signal Generator Settings
    ///
    ///  * `reqType`
    ///  * `records`
    ///  * `priority`
    pub fn dp(
        req_type: crate::Ref::SignalGen::DpReqType,
        records: u32,
        priority: u32,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10011003;
        __opcode.serialize_to(__encoded, &mut __offset);
        req_type.serialize_to(__encoded, &mut __offset);
        records.serialize_to(__encoded, &mut __offset);
        priority.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Type of the output signal: SINE, TRIANGLE, etc.
    pub fn r#type() -> FprimeResult<(crate::Ref::SignalType, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalType as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10011000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalType as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Single Y value of the output
    pub fn output() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10011001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Single (time, value) pair of the signal
    pub fn pair_output() -> FprimeResult<
        (crate::Ref::SignalPair, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalPair as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10011002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalPair as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Last 10 Y values of the signal
    pub fn history() -> FprimeResult<(crate::Ref::SignalSet, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalSet as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10011003, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalSet as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Last 10 (time, value) pairs of the signal
    pub fn pair_history() -> FprimeResult<
        (crate::Ref::SignalPairSet, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalPairSet as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10011004, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalPairSet as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Composite field of signal information, containing histories, pairs etc
    pub fn info() -> FprimeResult<(crate::Ref::SignalInfo, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalInfo as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10011005, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalInfo as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// DP bytes written
    pub fn dp_bytes() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10011006, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// DP records written
    pub fn dp_records() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10011007, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct RefSg2 {}
impl RefSg2 {
    /// Signal Generator Settings
    ///
    ///  * `Frequency`
    ///  * `Amplitude`
    ///  * `Phase`
    ///  * `SigType`
    pub fn settings(
        frequency: u32,
        amplitude: f32,
        phase: f32,
        sig_type: crate::Ref::SignalType,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10012000;
        __opcode.serialize_to(__encoded, &mut __offset);
        frequency.serialize_to(__encoded, &mut __offset);
        amplitude.serialize_to(__encoded, &mut __offset);
        phase.serialize_to(__encoded, &mut __offset);
        sig_type.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Toggle Signal Generator On/Off.
    pub fn toggle() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10012001;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Skip next sample
    pub fn skip() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10012002;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Signal Generator Settings
    ///
    ///  * `reqType`
    ///  * `records`
    ///  * `priority`
    pub fn dp(
        req_type: crate::Ref::SignalGen::DpReqType,
        records: u32,
        priority: u32,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10012003;
        __opcode.serialize_to(__encoded, &mut __offset);
        req_type.serialize_to(__encoded, &mut __offset);
        records.serialize_to(__encoded, &mut __offset);
        priority.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Type of the output signal: SINE, TRIANGLE, etc.
    pub fn r#type() -> FprimeResult<(crate::Ref::SignalType, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalType as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10012000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalType as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Single Y value of the output
    pub fn output() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10012001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Single (time, value) pair of the signal
    pub fn pair_output() -> FprimeResult<
        (crate::Ref::SignalPair, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalPair as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10012002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalPair as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Last 10 Y values of the signal
    pub fn history() -> FprimeResult<(crate::Ref::SignalSet, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalSet as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10012003, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalSet as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Last 10 (time, value) pairs of the signal
    pub fn pair_history() -> FprimeResult<
        (crate::Ref::SignalPairSet, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalPairSet as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10012004, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalPairSet as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Composite field of signal information, containing histories, pairs etc
    pub fn info() -> FprimeResult<(crate::Ref::SignalInfo, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalInfo as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10012005, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalInfo as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// DP bytes written
    pub fn dp_bytes() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10012006, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// DP records written
    pub fn dp_records() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10012007, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct RefSg3 {}
impl RefSg3 {
    /// Signal Generator Settings
    ///
    ///  * `Frequency`
    ///  * `Amplitude`
    ///  * `Phase`
    ///  * `SigType`
    pub fn settings(
        frequency: u32,
        amplitude: f32,
        phase: f32,
        sig_type: crate::Ref::SignalType,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10013000;
        __opcode.serialize_to(__encoded, &mut __offset);
        frequency.serialize_to(__encoded, &mut __offset);
        amplitude.serialize_to(__encoded, &mut __offset);
        phase.serialize_to(__encoded, &mut __offset);
        sig_type.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Toggle Signal Generator On/Off.
    pub fn toggle() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10013001;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Skip next sample
    pub fn skip() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10013002;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Signal Generator Settings
    ///
    ///  * `reqType`
    ///  * `records`
    ///  * `priority`
    pub fn dp(
        req_type: crate::Ref::SignalGen::DpReqType,
        records: u32,
        priority: u32,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10013003;
        __opcode.serialize_to(__encoded, &mut __offset);
        req_type.serialize_to(__encoded, &mut __offset);
        records.serialize_to(__encoded, &mut __offset);
        priority.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Type of the output signal: SINE, TRIANGLE, etc.
    pub fn r#type() -> FprimeResult<(crate::Ref::SignalType, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalType as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10013000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalType as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Single Y value of the output
    pub fn output() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10013001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Single (time, value) pair of the signal
    pub fn pair_output() -> FprimeResult<
        (crate::Ref::SignalPair, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalPair as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10013002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalPair as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Last 10 Y values of the signal
    pub fn history() -> FprimeResult<(crate::Ref::SignalSet, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalSet as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10013003, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalSet as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Last 10 (time, value) pairs of the signal
    pub fn pair_history() -> FprimeResult<
        (crate::Ref::SignalPairSet, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalPairSet as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10013004, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalPairSet as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Composite field of signal information, containing histories, pairs etc
    pub fn info() -> FprimeResult<(crate::Ref::SignalInfo, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalInfo as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10013005, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalInfo as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// DP bytes written
    pub fn dp_bytes() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10013006, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// DP records written
    pub fn dp_records() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10013007, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct RefSg4 {}
impl RefSg4 {
    /// Signal Generator Settings
    ///
    ///  * `Frequency`
    ///  * `Amplitude`
    ///  * `Phase`
    ///  * `SigType`
    pub fn settings(
        frequency: u32,
        amplitude: f32,
        phase: f32,
        sig_type: crate::Ref::SignalType,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10014000;
        __opcode.serialize_to(__encoded, &mut __offset);
        frequency.serialize_to(__encoded, &mut __offset);
        amplitude.serialize_to(__encoded, &mut __offset);
        phase.serialize_to(__encoded, &mut __offset);
        sig_type.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Toggle Signal Generator On/Off.
    pub fn toggle() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10014001;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Skip next sample
    pub fn skip() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10014002;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Signal Generator Settings
    ///
    ///  * `reqType`
    ///  * `records`
    ///  * `priority`
    pub fn dp(
        req_type: crate::Ref::SignalGen::DpReqType,
        records: u32,
        priority: u32,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10014003;
        __opcode.serialize_to(__encoded, &mut __offset);
        req_type.serialize_to(__encoded, &mut __offset);
        records.serialize_to(__encoded, &mut __offset);
        priority.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Type of the output signal: SINE, TRIANGLE, etc.
    pub fn r#type() -> FprimeResult<(crate::Ref::SignalType, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalType as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10014000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalType as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Single Y value of the output
    pub fn output() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10014001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Single (time, value) pair of the signal
    pub fn pair_output() -> FprimeResult<
        (crate::Ref::SignalPair, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalPair as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10014002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalPair as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Last 10 Y values of the signal
    pub fn history() -> FprimeResult<(crate::Ref::SignalSet, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalSet as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10014003, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalSet as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Last 10 (time, value) pairs of the signal
    pub fn pair_history() -> FprimeResult<
        (crate::Ref::SignalPairSet, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalPairSet as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10014004, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalPairSet as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Composite field of signal information, containing histories, pairs etc
    pub fn info() -> FprimeResult<(crate::Ref::SignalInfo, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalInfo as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10014005, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalInfo as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// DP bytes written
    pub fn dp_bytes() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10014006, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// DP records written
    pub fn dp_records() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10014007, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct RefSg5 {}
impl RefSg5 {
    /// Signal Generator Settings
    ///
    ///  * `Frequency`
    ///  * `Amplitude`
    ///  * `Phase`
    ///  * `SigType`
    pub fn settings(
        frequency: u32,
        amplitude: f32,
        phase: f32,
        sig_type: crate::Ref::SignalType,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10015000;
        __opcode.serialize_to(__encoded, &mut __offset);
        frequency.serialize_to(__encoded, &mut __offset);
        amplitude.serialize_to(__encoded, &mut __offset);
        phase.serialize_to(__encoded, &mut __offset);
        sig_type.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Toggle Signal Generator On/Off.
    pub fn toggle() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10015001;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Skip next sample
    pub fn skip() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10015002;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Signal Generator Settings
    ///
    ///  * `reqType`
    ///  * `records`
    ///  * `priority`
    pub fn dp(
        req_type: crate::Ref::SignalGen::DpReqType,
        records: u32,
        priority: u32,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10015003;
        __opcode.serialize_to(__encoded, &mut __offset);
        req_type.serialize_to(__encoded, &mut __offset);
        records.serialize_to(__encoded, &mut __offset);
        priority.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Type of the output signal: SINE, TRIANGLE, etc.
    pub fn r#type() -> FprimeResult<(crate::Ref::SignalType, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalType as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10015000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalType as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Single Y value of the output
    pub fn output() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10015001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Single (time, value) pair of the signal
    pub fn pair_output() -> FprimeResult<
        (crate::Ref::SignalPair, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalPair as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10015002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalPair as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Last 10 Y values of the signal
    pub fn history() -> FprimeResult<(crate::Ref::SignalSet, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalSet as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10015003, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalSet as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Last 10 (time, value) pairs of the signal
    pub fn pair_history() -> FprimeResult<
        (crate::Ref::SignalPairSet, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalPairSet as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10015004, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalPairSet as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Composite field of signal information, containing histories, pairs etc
    pub fn info() -> FprimeResult<(crate::Ref::SignalInfo, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SignalInfo as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10015005, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SignalInfo as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// DP bytes written
    pub fn dp_bytes() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10015006, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// DP records written
    pub fn dp_records() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10015007, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct RefBlockDrv {}
impl RefBlockDrv {
    /// Driver cycle count
    pub fn bd_cycles() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10000000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct RefCmdSeq {}
impl RefCmdSeq {
    /// Run a command sequence file
    ///
    ///  * `fileName` - The name of the sequence file
    ///  * `block` - Return command status when complete or not
    pub fn cs_run(
        file_name: &str,
        block: crate::Svc::BlockState,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10006000;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(file_name)
            .serialize_to(__encoded, &mut __offset);
        block.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Validate a command sequence file
    ///
    ///  * `fileName` - The name of the sequence file
    pub fn cs_validate(file_name: &str) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10006001;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(file_name)
            .serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Cancel a command sequence
    pub fn cs_cancel() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10006002;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Start running a command sequence
    pub fn cs_start() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10006003;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Perform one step in a command sequence. Valid only if CmdSequencer is in MANUAL run mode.
    pub fn cs_step() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10006004;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Set the run mode to AUTO.
    pub fn cs_auto() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10006005;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Set the run mode to MANUAL.
    pub fn cs_manual() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10006006;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Wait for sequences that are running to finish. Allow user to run multiple seq files in SEQ_NO_BLOCK mode then wait for them to finish before allowing more seq run request.
    pub fn cs_join_wait() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10006007;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// The number of Load commands executed
    pub fn cs_load_commands() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10006000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The number of Cancel commands executed
    pub fn cs_cancel_commands() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10006001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The number of errors that have occurred
    pub fn cs_errors() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10006002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The number of commands executed across all sequences.
    pub fn cs_commands_executed() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10006003, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// The number of sequences completed.
    pub fn cs_sequences_completed() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10006004, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct RefDpDemo {}
impl RefDpDemo {
    /// Select color
    ///
    ///  * `color`
    pub fn select_color(color: crate::Ref::DpDemo::ColorEnum) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0xA10;
        __opcode.serialize_to(__encoded, &mut __offset);
        color.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Command for generating a DP
    ///
    ///  * `reqType`
    ///  * `priority`
    ///  * `proc`
    pub fn dp(
        req_type: crate::Ref::DpDemo::DpReqType,
        priority: u32,
        proc: crate::Fw::DpCfg::ProcType,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0xA11;
        __opcode.serialize_to(__encoded, &mut __offset);
        req_type.serialize_to(__encoded, &mut __offset);
        priority.serialize_to(__encoded, &mut __offset);
        proc.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
}
struct RefPingRcvr {}
impl RefPingRcvr {
    /// Command to disable ping response
    pub fn pr_stop_pings() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10004000;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Number of pings received
    pub fn pr_num_pings() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10004000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct RefRateGroup1Comp {}
impl RefRateGroup1Comp {
    /// Max execution time rate group
    pub fn rg_max_time() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10001000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Cycle slips for rate group
    pub fn rg_cycle_slips() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10001001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct RefRateGroup2Comp {}
impl RefRateGroup2Comp {
    /// Max execution time rate group
    pub fn rg_max_time() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10002000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Cycle slips for rate group
    pub fn rg_cycle_slips() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10002001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct RefRateGroup3Comp {}
impl RefRateGroup3Comp {
    /// Max execution time rate group
    pub fn rg_max_time() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10003000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Cycle slips for rate group
    pub fn rg_cycle_slips() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10003001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct RefRecvBuffComp {}
impl RefRecvBuffComp {
    /// A test parameter
    ///
    ///  * `val`
    pub fn parameter_1_prm_set(val: u32) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10022000;
        __opcode.serialize_to(__encoded, &mut __offset);
        val.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// A test parameter
    pub fn parameter_1_prm_save() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10022001;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// A test parameter
    ///
    ///  * `val`
    pub fn parameter_2_prm_set(val: i16) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10022002;
        __opcode.serialize_to(__encoded, &mut __offset);
        val.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// A test parameter
    pub fn parameter_2_prm_save() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10022003;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Packet Statistics
    pub fn pkt_state() -> FprimeResult<(crate::Ref::PacketStat, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::PacketStat as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10022000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::PacketStat as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Value of Sensor1
    pub fn sensor_1() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10022001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Value of Sensor3
    pub fn sensor_2() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10022002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Readback of Parameter1
    pub fn parameter_1() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10022003, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Readback of Parameter2
    pub fn parameter_2() -> FprimeResult<(i16, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <i16 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10022004, &mut time_buf, &mut value_buf) }?;
        Ok((
            <i16 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct RefSendBuffComp {}
impl RefSendBuffComp {
    /// Command to start sending packets
    pub fn sb_start_pkts() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10010000;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Send a bad packet
    pub fn sb_inject_pkt_error() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10010001;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Generate a FATAL EVR
    ///
    ///  * `arg1` - First FATAL Argument
    ///  * `arg2` - Second FATAL Argument
    ///  * `arg3` - Third FATAL Argument
    pub fn sb_gen_fatal(arg_1: u32, arg_2: u32, arg_3: u32) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10010002;
        __opcode.serialize_to(__encoded, &mut __offset);
        arg_1.serialize_to(__encoded, &mut __offset);
        arg_2.serialize_to(__encoded, &mut __offset);
        arg_3.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Generate an ASSERT
    ///
    ///  * `arg1` - First ASSERT Argument
    ///  * `arg2` - Second ASSERT Argument
    ///  * `arg3` - Third ASSERT Argument
    ///  * `arg4` - Fourth ASSERT Argument
    ///  * `arg5` - Fifth ASSERT Argument
    ///  * `arg6` - Sixth ASSERT Argument
    pub fn sb_gen_assert(
        arg_1: u32,
        arg_2: u32,
        arg_3: u32,
        arg_4: u32,
        arg_5: u32,
        arg_6: u32,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10010003;
        __opcode.serialize_to(__encoded, &mut __offset);
        arg_1.serialize_to(__encoded, &mut __offset);
        arg_2.serialize_to(__encoded, &mut __offset);
        arg_3.serialize_to(__encoded, &mut __offset);
        arg_4.serialize_to(__encoded, &mut __offset);
        arg_5.serialize_to(__encoded, &mut __offset);
        arg_6.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// A test parameter
    ///
    ///  * `val`
    pub fn parameter_3_prm_set(val: u8) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1001000A;
        __opcode.serialize_to(__encoded, &mut __offset);
        val.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// A test parameter
    pub fn parameter_3_prm_save() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1001000B;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// A test parameter
    ///
    ///  * `val`
    pub fn parameter_4_prm_set(val: f32) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1001000C;
        __opcode.serialize_to(__encoded, &mut __offset);
        val.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// A test parameter
    pub fn parameter_4_prm_save() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1001000D;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Number of packets sent
    pub fn packets_sent() -> FprimeResult<(u64, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u64 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10010000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u64 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Number of errors injected
    pub fn num_errors_injected() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10010001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Readback of Parameter3
    pub fn parameter_3() -> FprimeResult<(u8, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u8 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10010002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u8 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Readback of Parameter4
    pub fn parameter_4() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10010003, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Readback of Parameter4
    pub fn send_state() -> FprimeResult<
        (crate::Ref::SendBuff::ActiveState, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::SendBuff::ActiveState as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10010004, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::SendBuff::ActiveState as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct RefSystemResources {}
impl RefSystemResources {
    /// A command to enable or disable system resource telemetry
    ///
    ///  * `enable` - whether or not system resource telemetry is enabled
    pub fn enable(enable: crate::Svc::SystemResourceEnabled) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10023000;
        __opcode.serialize_to(__encoded, &mut __offset);
        enable.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Total system memory in KB
    pub fn memory_total() -> FprimeResult<(u64, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u64 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10023000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u64 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System memory used in KB
    pub fn memory_used() -> FprimeResult<(u64, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u64 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10023001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u64 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System non-volatile available in KB
    pub fn non_volatile_total() -> FprimeResult<(u64, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u64 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10023002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u64 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System non-volatile available in KB
    pub fn non_volatile_free() -> FprimeResult<(u64, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u64 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10023003, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u64 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10023004, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_00() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10023005, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_01() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10023006, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_02() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10023007, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_03() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10023008, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_04() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10023009, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_05() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1002300A, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_06() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1002300B, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_07() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1002300C, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_08() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1002300D, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_09() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1002300E, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_10() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1002300F, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_11() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10023010, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_12() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10023011, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_13() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10023012, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_14() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10023013, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// System's CPU Percentage
    pub fn cpu_15() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10023014, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct RefTypeDemo {}
impl RefTypeDemo {
    /// Single choice command
    ///
    ///  * `choice` - A single choice
    pub fn choice(choice: crate::Ref::Choice) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005000;
        __opcode.serialize_to(__encoded, &mut __offset);
        choice.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Single enumeration parameter
    ///
    ///  * `val`
    pub fn choice_prm_prm_set(val: crate::Ref::Choice) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005001;
        __opcode.serialize_to(__encoded, &mut __offset);
        val.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Single enumeration parameter
    pub fn choice_prm_prm_save() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005002;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Multiple choice command via Array
    ///
    ///  * `choices` - A set of choices
    pub fn choices(choices: crate::Ref::ManyChoices) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005003;
        __opcode.serialize_to(__encoded, &mut __offset);
        choices.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Multiple choice command via Array with a preceding and following argument
    ///
    ///  * `repeat` - Number of times to repeat the choices
    ///  * `choices` - A set of choices
    ///  * `repeat_max` - Limit to the number of repetitions
    pub fn choices_with_friends(
        repeat: u8,
        choices: crate::Ref::ManyChoices,
        repeat_max: u8,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005004;
        __opcode.serialize_to(__encoded, &mut __offset);
        repeat.serialize_to(__encoded, &mut __offset);
        choices.serialize_to(__encoded, &mut __offset);
        repeat_max.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Multiple enumeration parameter via Array
    ///
    ///  * `val`
    pub fn choices_prm_prm_set(val: crate::Ref::ManyChoices) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005005;
        __opcode.serialize_to(__encoded, &mut __offset);
        val.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Multiple enumeration parameter via Array
    pub fn choices_prm_prm_save() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005006;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Too many choice command via Array
    ///
    ///  * `choices` - Way to many choices to make
    pub fn extra_choices(choices: crate::Ref::TooManyChoices) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005007;
        __opcode.serialize_to(__encoded, &mut __offset);
        choices.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Too many choices command via Array with a preceding and following argument
    ///
    ///  * `repeat` - Number of times to repeat the choices
    ///  * `choices` - Way to many choices to make
    ///  * `repeat_max` - Limit to the number of repetitions
    pub fn extra_choices_with_friends(
        repeat: u8,
        choices: crate::Ref::TooManyChoices,
        repeat_max: u8,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005008;
        __opcode.serialize_to(__encoded, &mut __offset);
        repeat.serialize_to(__encoded, &mut __offset);
        choices.serialize_to(__encoded, &mut __offset);
        repeat_max.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Too many enumeration parameter via Array
    ///
    ///  * `val`
    pub fn extra_choices_prm_prm_set(
        val: crate::Ref::ManyChoices,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005009;
        __opcode.serialize_to(__encoded, &mut __offset);
        val.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Too many enumeration parameter via Array
    pub fn extra_choices_prm_prm_save() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1000500A;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Multiple choice command via Structure
    ///
    ///  * `choices` - A pair of choices
    pub fn choice_pair(choices: crate::Ref::ChoicePair) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1000500B;
        __opcode.serialize_to(__encoded, &mut __offset);
        choices.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Multiple choices command via Structure with a preceding and following argument
    ///
    ///  * `repeat` - Number of times to repeat the choices
    ///  * `choices` - A pair of choices
    ///  * `repeat_max` - Limit to the number of repetitions
    pub fn choice_pair_with_friends(
        repeat: u8,
        choices: crate::Ref::ChoicePair,
        repeat_max: u8,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1000500C;
        __opcode.serialize_to(__encoded, &mut __offset);
        repeat.serialize_to(__encoded, &mut __offset);
        choices.serialize_to(__encoded, &mut __offset);
        repeat_max.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Multiple enumeration parameter via Structure
    ///
    ///  * `val`
    pub fn choice_pair_prm_prm_set(
        val: crate::Ref::ChoicePair,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1000500D;
        __opcode.serialize_to(__encoded, &mut __offset);
        val.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Multiple enumeration parameter via Structure
    pub fn choice_pair_prm_prm_save() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1000500E;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Multiple choice command via Complex Structure
    ///
    ///  * `choices` - A phenomenal amount of choice
    pub fn glutton_of_choice(
        choices: crate::Ref::ChoiceSlurry,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1000500F;
        __opcode.serialize_to(__encoded, &mut __offset);
        choices.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Multiple choices command via Complex Structure with a preceding and following argument
    ///
    ///  * `repeat` - Number of times to repeat the choices
    ///  * `choices` - A phenomenal amount of choice
    ///  * `repeat_max` - Limit to the number of repetitions
    pub fn glutton_of_choice_with_friends(
        repeat: u8,
        choices: crate::Ref::ChoiceSlurry,
        repeat_max: u8,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005010;
        __opcode.serialize_to(__encoded, &mut __offset);
        repeat.serialize_to(__encoded, &mut __offset);
        choices.serialize_to(__encoded, &mut __offset);
        repeat_max.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Multiple enumeration parameter via Complex Structure
    ///
    ///  * `val`
    pub fn glutton_of_choice_prm_prm_set(
        val: crate::Ref::ChoiceSlurry,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005011;
        __opcode.serialize_to(__encoded, &mut __offset);
        val.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Multiple enumeration parameter via Complex Structure
    pub fn glutton_of_choice_prm_prm_save() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005012;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Dump the typed parameters
    pub fn dump_typed_parameters() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005013;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Dump the float values
    pub fn dump_floats() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005014;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Send scalars
    ///
    ///  * `scalar_input`
    pub fn send_scalars(
        scalar_input: crate::Ref::ScalarStruct,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10005015;
        __opcode.serialize_to(__encoded, &mut __offset);
        scalar_input.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Single choice channel
    pub fn choice_ch() -> FprimeResult<(crate::Ref::Choice, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::Choice as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10005000, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::Choice as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Multiple choice channel via Array
    pub fn choices_ch() -> FprimeResult<
        (crate::Ref::ManyChoices, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::ManyChoices as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10005001, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::ManyChoices as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Too many choice channel via Array
    pub fn extra_choices_ch() -> FprimeResult<
        (crate::Ref::TooManyChoices, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::TooManyChoices as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10005002, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::TooManyChoices as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Multiple choice channel via Structure
    pub fn choice_pair_ch() -> FprimeResult<
        (crate::Ref::ChoicePair, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::ChoicePair as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10005003, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::ChoicePair as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Multiple choice channel via Complex Structure
    pub fn choice_slurry_ch() -> FprimeResult<
        (crate::Ref::ChoiceSlurry, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::ChoiceSlurry as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10005004, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::ChoiceSlurry as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Float output channel 1
    pub fn float_1_ch() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10005005, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Float output channel 2
    pub fn float_2_ch() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10005006, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Float output channel 3
    pub fn float_3_ch() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10005007, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Float set output channel
    pub fn float_set() -> FprimeResult<(crate::Ref::FloatSet, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::FloatSet as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10005008, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::FloatSet as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Scalar struct channel
    pub fn scalar_struct_ch() -> FprimeResult<
        (crate::Ref::ScalarStruct, crate::fw::TimeValue),
    > {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <crate::Ref::ScalarStruct as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10005009, &mut time_buf, &mut value_buf) }?;
        Ok((
            <crate::Ref::ScalarStruct as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Scalar U8 channel
    pub fn scalar_u_8_ch() -> FprimeResult<(u8, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u8 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1000500A, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u8 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Scalar U16 channel
    pub fn scalar_u_16_ch() -> FprimeResult<(u16, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u16 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1000500B, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u16 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Scalar U32 channel
    pub fn scalar_u_32_ch() -> FprimeResult<(u32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1000500C, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Scalar U64 channel
    pub fn scalar_u_64_ch() -> FprimeResult<(u64, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <u64 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1000500D, &mut time_buf, &mut value_buf) }?;
        Ok((
            <u64 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Scalar I8 channel
    pub fn scalar_i_8_ch() -> FprimeResult<(i8, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <i8 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1000500E, &mut time_buf, &mut value_buf) }?;
        Ok((
            <i8 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Scalar I16 channel
    pub fn scalar_i_16_ch() -> FprimeResult<(i16, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <i16 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x1000500F, &mut time_buf, &mut value_buf) }?;
        Ok((
            <i16 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Scalar I32 channel
    pub fn scalar_i_32_ch() -> FprimeResult<(i32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <i32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10005010, &mut time_buf, &mut value_buf) }?;
        Ok((
            <i32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Scalar I64 channel
    pub fn scalar_i_64_ch() -> FprimeResult<(i64, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <i64 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10005011, &mut time_buf, &mut value_buf) }?;
        Ok((
            <i64 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Scalar F32 channel
    pub fn scalar_f_32_ch() -> FprimeResult<(f32, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f32 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10005012, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f32 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
    /// Scalar F64 channel
    pub fn scalar_f_64_ch() -> FprimeResult<(f64, crate::fw::TimeValue)> {
        let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        let mut value_buf: [u8; <f64 as Serializable>::SIZE] = unsafe {
            #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
        };
        unsafe { sys::telemetry(0x10005013, &mut time_buf, &mut value_buf) }?;
        Ok((
            <f64 as Serializable>::deserialize(value_buf),
            crate::fw::TimeValue::deserialize(time_buf),
        ))
    }
}
struct RefWasmSeq {}
impl RefWasmSeq {
    /// Run a Wasm module main function on it's own in the interpreter
    /// This command is short-hand for:
    /// 1. LOAD_NAME [fileName] ""
    /// 2. INVOKE "" "main"
    /// 3. CONTINUE
    ///
    /// If $block == Svc.BlockState.BLOCK this command will wait for complemention.
    ///
    ///  * `fileName` - The name of the sequence file
    ///  * `block` - Return command status when complete or not
    pub fn run(
        file_name: &str,
        block: crate::Svc::BlockState,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10007000;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(file_name)
            .serialize_to(__encoded, &mut __offset);
        block.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Wait for the interpreter to finish and return it's result as a CmdResponse
    pub fn wait() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10007001;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Loads and validates a WebAssembly module into the store.
    /// This command loads the module with a empty name meaning only a single module may be loaded.
    /// To allow multiple modules, use the `LOAD_NAME` command instead.
    ///
    ///  * `fileName` - The name of the sequence file
    pub fn load(file_name: &str) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10007002;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(file_name)
            .serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Load and validate a WebAssembly module into the store. This module is given a name so that
    /// it's exports can be referenced by other modules.
    ///
    ///  * `fileName` - The name of the sequence file
    ///  * `name` - WebAssembly module name, must not conflict with previously loaded modules
    pub fn load_name(file_name: &str, name: &str) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10007003;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<240> as StrTruncate<240>>::truncate(file_name)
            .serialize_to(__encoded, &mut __offset);
        <String<16> as StrTruncate<16>>::truncate(name)
            .serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Invoke a main function from a loaded module
    ///
    ///  * `module` - Name of the module to invoke a function from
    ///  * `block`
    pub fn invoke(
        module: &str,
        block: crate::Svc::BlockState,
    ) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10007004;
        __opcode.serialize_to(__encoded, &mut __offset);
        <String<16> as StrTruncate<16>>::truncate(module)
            .serialize_to(__encoded, &mut __offset);
        block.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Cancels a running or validated sequence. After running CANCEL, the sequencer should return to IDLE
    /// This completely clears the store.
    /// Cancelling during LOADING will trigger a fail response in the reader and
    /// return to IDLE mode.
    pub fn cancel() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10007005;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Pauses the execution of the sequencer, just before it is about to dispatch the next directive,
    /// until unpaused by the CONTINUE command, or stepped by the STEP command. This command is only valid
    /// substates of the RUNNING state that are not RUNNING.PAUSED.
    pub fn pause() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10007006;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Dump a stack trace in events
    pub fn trace() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10007007;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    pub fn r#continue() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10007008;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// the number of seconds to wait before giving up
    /// on a directive or command. if <= 0 or greater than U32 max, never time out.
    /// accuracy of this timeout is determined by the rate group driving this
    /// component. it will be rounded up
    ///
    ///  * `val`
    pub fn statement_timeout_secs_prm_set(val: f32) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x10007009;
        __opcode.serialize_to(__encoded, &mut __offset);
        val.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// the number of seconds to wait before giving up
    /// on a directive or command. if <= 0 or greater than U32 max, never time out.
    /// accuracy of this timeout is determined by the rate group driving this
    /// component. it will be rounded up
    pub fn statement_timeout_secs_prm_save() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1000700A;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Number of Wasm instructions to execute per interpreter cycle.
    /// Larger numbers will effect the responsiveness of the interpreter to
    /// `PAUSE` commands. The interpreter will always pause before commands.
    ///
    /// Larger numbers make the interpreter faster because the engine doesn't
    /// need to feed itself through a queue to fuel more cycles.
    ///
    ///  * `val`
    pub fn instruction_fuel_prm_set(val: crate::FwSizeType) -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1000700B;
        __opcode.serialize_to(__encoded, &mut __offset);
        val.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
    /// Number of Wasm instructions to execute per interpreter cycle.
    /// Larger numbers will effect the responsiveness of the interpreter to
    /// `PAUSE` commands. The interpreter will always pause before commands.
    ///
    /// Larger numbers make the interpreter faster because the engine doesn't
    /// need to feed itself through a queue to fuel more cycles.
    pub fn instruction_fuel_prm_save() -> crate::fw::CmdResponse {
        let __encoded = unsafe {
            let ptr = (&raw mut super::super::__SCRATCH) as *mut u8;
            let len = super::super::__SCRATCH_SIZE;
            core::slice::from_raw_parts_mut(ptr, len)
        };
        let mut __offset: usize = 0;
        let __opcode: crate::FwOpcodeType = 0x1000700C;
        __opcode.serialize_to(__encoded, &mut __offset);
        let res = unsafe { sys::command(&__encoded[0..__offset]) };
        unsafe { core::mem::transmute(res as u8) }
    }
}
struct Ref {
    sg_1: RefSg1,
    sg_2: RefSg2,
    sg_3: RefSg3,
    sg_4: RefSg4,
    sg_5: RefSg5,
    block_drv: RefBlockDrv,
    cmd_seq: RefCmdSeq,
    dp_demo: RefDpDemo,
    ping_rcvr: RefPingRcvr,
    rate_group_1_comp: RefRateGroup1Comp,
    rate_group_2_comp: RefRateGroup2Comp,
    rate_group_3_comp: RefRateGroup3Comp,
    recv_buff_comp: RefRecvBuffComp,
    send_buff_comp: RefSendBuffComp,
    system_resources: RefSystemResources,
    type_demo: RefTypeDemo,
    wasm_seq: RefWasmSeq,
}
impl Ref {}
