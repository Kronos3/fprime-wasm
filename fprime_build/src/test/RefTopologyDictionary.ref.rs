pub mod Defs {
    #[allow(unused_imports)]
    use fprime_core::*;
    /// The type of a telemetry channel identifier
    pub type FwChanIdType = crate::Defs::FwIdType;
    /// The type of a data product identifier
    pub type FwDpIdType = crate::Defs::FwIdType;
    /// The type of a data product priority
    pub type FwDpPriorityType = u32;
    /// The type of an event identifier
    pub type FwEventIdType = crate::Defs::FwIdType;
    /// The id type.
    pub type FwIdType = u32;
    /// The type of smaller indices internal to the software, used
    /// for array indices, e.g., port indices. Must be signed.
    pub type FwIndexType = crate::Defs::PlatformIndexType;
    /// The type of a command opcode
    pub type FwOpcodeType = crate::Defs::FwIdType;
    /// The width of packet descriptors when they are serialized by the framework
    pub type FwPacketDescriptorType = u16;
    /// The type of a parameter identifier
    pub type FwPrmIdType = crate::Defs::FwIdType;
    /// The type used to serialize a size value
    pub type FwSizeStoreType = u16;
    /// The unsigned type of larger sizes internal to the software,
    /// e.g., memory buffer sizes, file sizes. Must be unsigned.
    pub type FwSizeType = crate::Defs::PlatformSizeType;
    /// The type used to serialize a time base value
    pub type FwTimeBaseStoreType = u16;
    /// The type used to serialize a time context value
    pub type FwTimeContextStoreType = u8;
    /// The type of a telemetry packet identifier
    pub type FwTlmPacketizeIdType = u16;
    /// The type of smaller indices internal to the software, used
    /// for array indices, e.g., port indices. Must be signed.
    pub type PlatformIndexType = i16;
    /// The unsigned type of larger sizes internal to the software,
    /// e.g., memory buffer sizes, file sizes. Must be unsigned.
    /// Supplied by platform, overridable by project.
    pub type PlatformSizeType = u64;
    /// Define enumeration for Time base types
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
    #[repr(u16)]
    pub enum TimeBase {
        /// No time base has been established (Required)
        TB_NONE = 0,
        /// Indicates time is processor cycle time. Not tied to external time
        TB_PROC_TIME = 1,
        /// Time as reported on workstation where software is running. For testing. (Required)
        TB_WORKSTATION_TIME = 2,
        /// Time as reported by the spacecraft clock.
        TB_SC_TIME = 3,
        /// Don't care value for sequences. If FwTimeBaseStoreType is changed, value should be changed (Required)
        TB_DONT_CARE = 65535,
    }
    /// Used for number of Fw::Buffer type ports supported by Svc::ComQueue
    const ComQueueBufferPorts: u64 = 1;
    /// Used for number of Fw::Com type ports supported by Svc::ComQueue
    const ComQueueComPorts: u64 = 2;
    /// Configuration for Fw::String
    /// Note: FPrimeBasicTypes.hpp needs to be updated to sync enum
    const FW_FIXED_LENGTH_STRING_SIZE: u64 = 256;
    /// Value encoded during serialization for boolean false
    const FW_SERIALIZE_FALSE_VALUE: u64 = 0;
    /// Specifies the size of the buffer that contains a communications packet
    const FW_COM_BUFFER_MAX_SIZE: u64 = 512;
    /// The size of a file name in an AssertFatalAdapter event (leading-truncation)
    /// Note: File names in assertion failures are also truncated by
    /// the constants FwAssertTextSize (in this file) and FW_LOG_STRING_MAX_SIZE (set
    /// in FW_LOG_STRING_MAX_SIZE)
    /// Set much smaller than FwAssertTextSize so there's space for time stamp/assert
    /// arguments in log message
    const AssertFatalAdapterEventFileSize: u64 = 240;
    /// The size of a file name string
    const FileNameStringSize: u64 = 240;
    /// Value encoded during serialization for boolean true
    const FW_SERIALIZE_TRUE_VALUE: u64 = 255;
    pub mod ComCfg {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// APIDs are 11 bits in the Space Packet protocol, so we use U16. Max value 7FF
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u16)]
        pub enum Apid {
            /// Command packet type - incoming
            FW_PACKET_COMMAND = 0,
            /// Telemetry packet type - outgoing
            FW_PACKET_TELEM = 1,
            /// Log type - outgoing
            FW_PACKET_LOG = 2,
            /// File type - incoming and outgoing
            FW_PACKET_FILE = 3,
            /// Packetized telemetry packet type
            FW_PACKET_PACKETIZED_TLM = 4,
            /// Data Product packet type
            FW_PACKET_DP = 5,
            /// F Prime idle
            FW_PACKET_IDLE = 6,
            /// F Prime handshake
            FW_PACKET_HAND = 254,
            /// F Prime unknown packet
            FW_PACKET_UNKNOWN = 255,
            /// Per Space Packet Standard, all 1s (11bits) is reserved for Idle Packets
            SPP_IDLE_PACKET = 2047,
            /// Anything equal or higher value is invalid and should not be used
            INVALID_UNINITIALIZED = 2048,
        }
        /// Packet Version Numbers are 3 bits with only 2 currently valid values
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum Pvn {
            /// Fully Featured CCSDS Space Packet Protocol
            SPACE_PACKET_PROTOCOL = 0,
            /// Bare-bones CCSDS Encapsulation Packet Protocol
            ENCAPSULATION_PACKET_PROTOCOL = 7,
            /// Anything equal or higher value is invalid and should not be used
            INVALID_UNINITIALIZED = 8,
        }
        /// Fixed size of CCSDS TM frames
        const TmFrameFixedSize: u64 = 1024;
        /// Spacecraft ID (10 bits) for CCSDS Data Link layer
        const SpacecraftId: u64 = 68;
    }
    pub mod Fw {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Enum representing a command response
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum CmdResponse {
            /// Command successfully executed
            OK = 0,
            /// Invalid opcode dispatched
            INVALID_OPCODE = 1,
            /// Command failed validation
            VALIDATION_ERROR = 2,
            /// Command failed to deserialize
            FORMAT_ERROR = 3,
            /// Command had execution error
            EXECUTION_ERROR = 4,
            /// Component busy
            BUSY = 5,
        }
        /// Deserialization status
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum DeserialStatus {
            OK = 0,
            /// Deserialization buffer was empty when trying to read data
            BUFFER_EMPTY = 3,
            /// Deserialization data had incorrect values (unexpected data types)
            FORMAT_ERROR = 4,
            /// Data was left in in the buffer, but not enough to deserialize
            SIZE_MISMATCH = 5,
            /// Deserialized type ID didn't match
            TYPE_MISMATCH = 6,
        }
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum DpState {
            /// The untransmitted state
            UNTRANSMITTED = 0,
            /// The partially transmitted state
            /// A data product is in this state from the start of transmission
            /// until transmission is complete.
            PARTIAL = 1,
            /// The transmitted state
            TRANSMITTED = 2,
        }
        /// Enabled and disabled states
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum Enabled {
            /// Disabled state
            DISABLED = 0,
            /// Enabled state
            ENABLED = 1,
        }
        /// Enum representing event severity
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum LogSeverity {
            /// A fatal non-recoverable event
            FATAL = 1,
            /// A serious but recoverable event
            WARNING_HI = 2,
            /// A less serious but recoverable event
            WARNING_LO = 3,
            /// An activity related to commanding
            COMMAND = 4,
            /// Important informational events
            ACTIVITY_HI = 5,
            /// Less important informational events
            ACTIVITY_LO = 6,
            /// Software diagnostic events
            DIAGNOSTIC = 7,
        }
        /// Enum representing parameter validity
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum ParamValid {
            UNINIT = 0,
            VALID = 1,
            INVALID = 2,
            DEFAULT = 3,
        }
        /// FPP shadow-enum representing Fw::FormatStatus
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum StringFormatStatus {
            /// Format worked
            SUCCESS = 0,
            /// Format overflowed
            OVERFLOWED = 1,
            /// Format provided invalid format string
            INVALID_FORMAT_STRING = 2,
            /// FwSizeType overflowed the range of size_t
            SIZE_OVERFLOW = 3,
            /// An error was returned from an underlying call
            OTHER_ERROR = 4,
        }
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(i32)]
        pub enum TimeComparison {
            LT = -1,
            EQ = 0,
            GT = 1,
            INCOMPARABLE = 2,
        }
        /// Data structure for Time Interval
        #[derive(Clone, Debug, Serializable)]
        pub struct TimeIntervalValue {
            /// seconds portion of TimeInterval
            pub seconds: u32,
            /// microseconds portion of TimeInterval
            pub useconds: u32,
        }
        /// Data structure for Time
        #[derive(Clone, Debug, Serializable)]
        pub struct TimeValue {
            /// basis of time (defined by system)
            pub timeBase: crate::Defs::TimeBase,
            /// user settable value. Could be reboot count, node, etc
            pub timeContext: crate::Defs::FwTimeContextStoreType,
            /// seconds portion of Time
            pub seconds: u32,
            /// microseconds portion of Time
            pub useconds: u32,
        }
        /// Wait or don't wait for something
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum Wait {
            /// Wait for something
            WAIT = 0,
            /// Don't wait for something
            NO_WAIT = 1,
        }
        pub mod DpCfg {
            #[allow(unused_imports)]
            use fprime_core::*;
            /// A bit mask for selecting the type of processing to perform on
            /// a container before writing it to disk.
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum ProcType {
                /// No Processing
                PROC_TYPE_NONE = 0,
                /// Processing type 0
                PROC_TYPE_ZLIB_DEFLATE = 1,
                /// Processing type 1
                PROC_TYPE_ONE = 2,
                /// Processing type 2
                PROC_TYPE_TWO = 4,
            }
            /// The size in bytes of the user-configurable data in the container
            /// packet header
            const CONTAINER_USER_DATA_SIZE: u64 = 32;
        }
    }
    pub mod Os {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// FPP shadow-enum representing Os::RawTime::Status
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum RawTimeStatus {
            /// Operation was successful
            OP_OK = 0,
            /// Operation result caused an overflow
            OP_OVERFLOW = 1,
            /// Parameters invalid for current platform
            INVALID_PARAMS = 2,
            /// RawTime feature is not supported
            NOT_SUPPORTED = 3,
            /// All other errors
            OTHER_ERROR = 4,
        }
    }
    pub mod Ref {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Enumeration type for use later
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(i32)]
        pub enum Choice {
            ONE = 0,
            TWO = 1,
            RED = 2,
            BLUE = 3,
        }
        /// Structure of enums
        #[derive(Clone, Debug, Serializable)]
        pub struct ChoicePair {
            /// The first choice to make
            pub firstChoice: crate::Defs::Ref::Choice,
            /// The second choice to make
            pub secondChoice: crate::Defs::Ref::Choice,
        }
        /// Structure of enums (with an multi-dimensional array and structure)
        #[derive(Clone, Debug, Serializable)]
        pub struct ChoiceSlurry {
            /// A large set of disorganized choices
            pub tooManyChoices: crate::Defs::Ref::TooManyChoices,
            /// A singular choice
            pub separateChoice: crate::Defs::Ref::Choice,
            /// A pair of choices
            pub choicePair: crate::Defs::Ref::ChoicePair,
            /// An array of choices defined as member array
            pub choiceAsMemberArray: [u8; 2],
        }
        /// Set of floating points to emit
        pub type FloatSet = [f32; 3];
        /// Enumeration array
        pub type ManyChoices = [crate::Defs::Ref::Choice; 2];
        /// Packet receive status
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(i32)]
        pub enum PacketRecvStatus {
            PACKET_STATE_NO_PACKETS = 0,
            PACKET_STATE_OK = 1,
            /// Receiver has seen errors
            PACKET_STATE_ERRORS = 3,
        }
        /// Some Packet Statistics
        #[derive(Clone, Debug, Serializable)]
        pub struct PacketStat {
            /// Number of buffers received
            pub BuffRecv: u32,
            /// Number of buffers received with errors
            pub BuffErr: u32,
            /// Packet Status
            pub PacketStatus: crate::Defs::Ref::PacketRecvStatus,
        }
        /// All scalar inputs
        #[derive(Clone, Debug, Serializable)]
        pub struct ScalarStruct {
            pub i8: i8,
            pub i16: i16,
            pub i32: i32,
            pub i64: i64,
            pub u8: u8,
            pub u16: u16,
            pub u32: u32,
            pub u64: u64,
            pub f32: f32,
            pub f64: f64,
        }
        #[derive(Clone, Debug, Serializable)]
        pub struct SignalInfo {
            pub r#type: crate::Defs::Ref::SignalType,
            pub history: crate::Defs::Ref::SignalSet,
            pub pairHistory: crate::Defs::Ref::SignalPairSet,
        }
        #[derive(Clone, Debug, Serializable)]
        pub struct SignalPair {
            pub time: f32,
            pub value: f32,
        }
        pub type SignalPairSet = [crate::Defs::Ref::SignalPair; 4];
        pub type SignalSet = [f32; 4];
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(i32)]
        pub enum SignalType {
            TRIANGLE = 0,
            SQUARE = 1,
            SINE = 2,
            NOISE = 3,
        }
        /// Array of array
        pub type TooManyChoices = [crate::Defs::Ref::ManyChoices; 2];
        const dimension: u64 = 2;
        pub mod DpDemo {
            #[allow(unused_imports)]
            use fprime_core::*;
            /// Array of array of strings
            pub type ArrayOfStringArray = [crate::Defs::Ref::DpDemo::StringArray; 3];
            pub type ArrayOfStructs = [crate::Defs::Ref::DpDemo::StructWithStringMembers; 3];
            pub type BoolAlias = bool;
            /// Array of booleans
            pub type BooleanArray = [bool; 2];
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(i32)]
            pub enum ColorEnum {
                RED = 0,
                GREEN = 1,
                BLUE = 2,
            }
            #[derive(Clone, Debug, Serializable)]
            pub struct ColorInfoStruct {
                pub Color: crate::Defs::Ref::DpDemo::ColorEnum,
            }
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(i32)]
            pub enum DpReqType {
                IMMEDIATE = 0,
                ASYNC = 1,
            }
            /// Array of enumerations
            pub type EnumArray = [crate::Defs::Ref::DpDemo::ColorEnum; 3];
            /// Array of floats
            pub type F32Array = [f32; 3];
            pub type F64Alias = f64;
            pub type I32Alias = i32;
            pub type StringAlias = String<80>;
            /// Array of strings
            pub type StringArray = [String<80>; 2];
            #[derive(Clone, Debug, Serializable)]
            pub struct StructWithEverything {
                pub integerMember: crate::Defs::Ref::DpDemo::I32Alias,
                pub floatMember: f32,
                pub stringMember: String<80>,
                pub booleanMember: bool,
                pub enumMember: crate::Defs::Ref::DpDemo::ColorEnum,
                pub arrayMemberU32: [crate::Defs::Ref::DpDemo::U32Array; 2],
                pub F32Array: crate::Defs::Ref::DpDemo::F32Array,
                pub U32Array: crate::Defs::Ref::DpDemo::U32Array,
                pub enumArray: crate::Defs::Ref::DpDemo::EnumArray,
                pub stringArray: crate::Defs::Ref::DpDemo::StringArray,
                pub booleanArray: crate::Defs::Ref::DpDemo::BooleanArray,
                pub structWithStrings: crate::Defs::Ref::DpDemo::StructWithStringMembers,
                pub nestedArrays: crate::Defs::Ref::DpDemo::ArrayOfStringArray,
            }
            #[derive(Clone, Debug, Serializable)]
            pub struct StructWithStringMembers {
                pub stringMember: String<80>,
                pub stringArrayMember: crate::Defs::Ref::DpDemo::StringArray,
            }
            /// Array of integers
            pub type U32Array = [u32; 5];
            const stringSize: u64 = 80;
        }
        pub mod SendBuff {
            #[allow(unused_imports)]
            use fprime_core::*;
            /// Active state
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(i32)]
            pub enum ActiveState {
                SEND_IDLE = 0,
                SEND_ACTIVE = 1,
            }
        }
        pub mod SignalGen {
            #[allow(unused_imports)]
            use fprime_core::*;
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(i32)]
            pub enum DpReqType {
                IMMEDIATE = 0,
                ASYNC = 1,
            }
        }
    }
    pub mod Svc {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Sequencer blocking state
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum BlockState {
            BLOCK = 0,
            NO_BLOCK = 1,
        }
        /// Array of queue depths for Fw::Buffer types
        pub type BuffQueueDepth = [u32; 1];
        /// Array of queue depths for Fw::Com types
        pub type ComQueueDepth = [u32; 2];
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum CompressionAlgorithm {
            UNCOMPRESSED = 0,
            ZLIB_DEFLATE = 1,
        }
        #[derive(Clone, Debug, Serializable)]
        pub struct CompressionMetadata {
            pub algorithm: crate::Defs::Svc::CompressionAlgorithm,
        }
        /// Data Structure for custom version Tlm
        #[derive(Clone, Debug, Serializable)]
        pub struct CustomVersionDb {
            /// enumeration/name of the custom version
            pub version_enum: crate::Defs::Svc::VersionCfg::VersionEnum,
            /// string containing custom version
            pub version_value: String<80>,
            /// status of the custom version
            pub version_status: crate::Defs::Svc::VersionStatus,
        }
        /// Header validation error
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum DpHdrField {
            DESCRIPTOR = 0,
            ID = 1,
            PRIORITY = 2,
            CRC = 3,
        }
        /// Data structure representing a data product.
        #[derive(Clone, Debug, Serializable)]
        pub struct DpRecord {
            pub id: crate::Defs::FwDpIdType,
            pub tSec: u32,
            pub tSub: u32,
            pub priority: u32,
            pub size: u64,
            pub blocks: u32,
            pub state: crate::Defs::Fw::DpState,
        }
        /// An enumeration of queue data types
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum QueueType {
            COM_QUEUE = 0,
            BUFFER_QUEUE = 1,
        }
        /// Send file status enum
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum SendFileStatus {
            STATUS_OK = 0,
            STATUS_ERROR = 1,
            STATUS_INVALID = 2,
            STATUS_BUSY = 3,
        }
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum SystemResourceEnabled {
            DISABLED = 0,
            ENABLED = 1,
        }
        /// Tracks versions for project, framework and user defined versions etc
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum VersionEnabled {
            /// verbosity disabled
            DISABLED = 0,
            /// verbosity enabled
            ENABLED = 1,
        }
        /// An enumeration for version status
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum VersionStatus {
            /// Version was good
            OK = 0,
            /// Failure to get version
            FAILURE = 1,
        }
        /// An enumeration for Version Type
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum VersionType {
            /// project version
            PROJECT = 0,
            /// framework version
            FRAMEWORK = 1,
            /// library version
            LIBRARY = 2,
            /// custom version
            CUSTOM = 3,
            /// all above versions
            ALL = 4,
        }
        pub mod BufferAccumulator {
            #[allow(unused_imports)]
            use fprime_core::*;
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum BlockMode {
                NOBLOCK = 0,
                BLOCK = 1,
            }
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum OpState {
                ACCUMULATE = 0,
                DRAIN = 1,
            }
        }
        pub mod Ccsds {
            #[allow(unused_imports)]
            use fprime_core::*;
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
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
            /// Protocol IDs for EPP encapsulation packets per CCSDS 133.1-B-3 Section 4.1.2.3.3
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum EppProtocolId {
                /// 0b000 - Encapsulation Idle Packet
                Idle = 0,
                /// 0b110 - Extended Protocol ID Field Driven
                Extended = 6,
                /// Mission-specific
                MissionSpecific = 7,
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
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum Tfvn {
                /// Telemetry and Telecommand SDLs
                TM_TC = 0,
                /// Advanced Orbiting Systems SDL
                AOS = 1,
                /// Proximity-1 SDL
                PROX_ONE = 2,
                /// Unified Space Data Link Protocol
                USLP = 3,
                /// Anything equal or higher value is invalid and should not be used
                INVALID_UNINITIALIZED = 4,
            }
        }
        pub mod CmdSequencer {
            #[allow(unused_imports)]
            use fprime_core::*;
            /// The stage of the file read operation
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum FileReadStage {
                READ_HEADER = 0,
                READ_HEADER_SIZE = 1,
                DESER_SIZE = 2,
                DESER_NUM_RECORDS = 3,
                DESER_TIME_BASE = 4,
                DESER_TIME_CONTEXT = 5,
                READ_SEQ_CRC = 6,
                READ_SEQ_DATA = 7,
                READ_SEQ_DATA_SIZE = 8,
            }
            /// The sequencer mode
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum SeqMode {
                STEP = 0,
                AUTO = 1,
            }
        }
        pub mod EventManager {
            #[allow(unused_imports)]
            use fprime_core::*;
            /// Enabled and disabled state
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum Enabled {
                /// Enabled state
                ENABLED = 0,
                /// Disabled state
                DISABLED = 1,
            }
            /// Severity level for event filtering
            /// Similar to Fw::LogSeverity, but no FATAL event
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum FilterSeverity {
                /// Filter WARNING_HI events
                WARNING_HI = 0,
                /// Filter WARNING_LO events
                WARNING_LO = 1,
                /// Filter COMMAND events
                COMMAND = 2,
                /// Filter ACTIVITY_HI events
                ACTIVITY_HI = 3,
                /// Filter ACTIVITY_LO events
                ACTIVITY_LO = 4,
                /// Filter DIAGNOSTIC events
                DIAGNOSTIC = 5,
            }
        }
        pub mod Fpy {
            #[allow(unused_imports)]
            use fprime_core::*;
            /// Serial port indices for FpySequencer serialOut port array.
            /// MAX_SERIAL_PORTS must be defined with this exact name for Fpy compiler bounds checking.
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum SerialPortIndex {
                /// Example serial port 0 - rename to application-specific name (e.g., TIME_SYNC_PORT)
                EXAMPLE_PORT_0 = 0,
                /// Example serial port 1 - rename to application-specific name (e.g., SENSOR_DATA_PORT)
                EXAMPLE_PORT_1 = 1,
                /// Example serial port 2 - rename to application-specific name
                EXAMPLE_PORT_2 = 2,
                /// Example serial port 3 - rename to application-specific name
                EXAMPLE_PORT_3 = 3,
                /// Example serial port 4 - rename to application-specific name
                EXAMPLE_PORT_4 = 4,
                /// REQUIRED: Maximum number of serial ports. This sentinel value MUST be named
                MAX_SERIAL_PORTS = 5,
            }
            /// the maximum number of bytes in a stack
            const MAX_STACK_SIZE: u64 = 65535;
            /// the default value of the SEQ_BASE_DIR parameter. suffixed to
            /// the input sequence file path before resolution occurs following
            /// the rules of Os::File::open. trailing slash optional
            const DEFAULT_SEQ_BASE_DIR: &'static str = "";
            /// The maximum number of statements a sequence can have
            const MAX_SEQUENCE_STATEMENT_COUNT: u64 = 2048;
            /// the maximum number of bytes in a directive
            const MAX_DIRECTIVE_SIZE: u64 = 2048;
            /// The maximum number of arguments a sequence can have
            const MAX_SEQUENCE_ARG_COUNT: u64 = 16;
        }
        pub mod PrmDb {
            #[allow(unused_imports)]
            use fprime_core::*;
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum Merge {
                MERGE = 0,
                RESET = 1,
            }
            /// State of parameter DB file load operations
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum PrmDbFileLoadState {
                IDLE = 0,
                LOADING_FILE_UPDATES = 1,
                FILE_UPDATES_STAGED = 2,
            }
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum PrmLoadAction {
                SET_PARAMETER = 0,
                SAVE_FILE_COMMAND = 1,
                LOAD_FILE_COMMAND = 2,
                COMMIT_STAGED_COMMAND = 3,
            }
            /// Parameter read error
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum PrmReadError {
                OPEN = 0,
                DELIMITER = 1,
                DELIMITER_SIZE = 2,
                DELIMITER_VALUE = 3,
                RECORD_SIZE = 4,
                RECORD_SIZE_SIZE = 5,
                RECORD_SIZE_VALUE = 6,
                PARAMETER_ID = 7,
                PARAMETER_ID_SIZE = 8,
                PARAMETER_VALUE = 9,
                PARAMETER_VALUE_SIZE = 10,
                CRC = 11,
                CRC_SIZE = 12,
                CRC_BUFFER = 13,
                SEEK_ZERO = 14,
            }
            /// Parameter write error
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum PrmWriteError {
                OPEN = 0,
                DELIMITER = 1,
                DELIMITER_SIZE = 2,
                RECORD_SIZE = 3,
                RECORD_SIZE_SIZE = 4,
                PARAMETER_ID = 5,
                PARAMETER_ID_SIZE = 6,
                PARAMETER_VALUE = 7,
                PARAMETER_VALUE_SIZE = 8,
                CRC_PLACE = 9,
                CRC_REAL = 10,
                CURR_POSITION = 11,
                SEEK_ZERO = 12,
                SEEK_POSITION = 13,
            }
        }
        pub mod VersionCfg {
            #[allow(unused_imports)]
            use fprime_core::*;
            /// Define a set of Version entries on a project-specific
            /// basis.
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u32)]
            pub enum VersionEnum {
                /// Entry 0
                PROJECT_VERSION_00 = 0,
                /// Entry 1
                PROJECT_VERSION_01 = 1,
                /// Entry 2
                PROJECT_VERSION_02 = 2,
                /// Entry 3
                PROJECT_VERSION_03 = 3,
                /// Entry 4
                PROJECT_VERSION_04 = 4,
                /// Entry 5
                PROJECT_VERSION_05 = 5,
                /// Entry 6
                PROJECT_VERSION_06 = 6,
                /// Entry 7
                PROJECT_VERSION_07 = 7,
                /// Entry 8
                PROJECT_VERSION_08 = 8,
                /// Entry 9
                PROJECT_VERSION_09 = 9,
            }
        }
        pub mod WasmSequencer {
            #[allow(unused_imports)]
            use fprime_core::*;
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(u8)]
            pub enum AllocError {
                /// Not enough pages could be allocated to accommodate this allocation
                OutOfMemory = 0,
                /// Page was too small to fit this allocation
                PageTooSmall = 1,
                /// A generic allocation failure
                AllocationFailed = 2,
            }
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(i32)]
            pub enum HostFunction {
                COMMAND = 0,
                EVENT = 1,
            }
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
            #[repr(i32)]
            pub enum Status {
                OK = 0,
                ERR_NULL_ARG = 1,
                ERR_BAD_ARG = 2,
                ERR_BAD_UTF8 = 3,
                ERR_NAME_TOO_LONG = 4,
                ERR_BAD_SIGNATURE = 5,
                ERR_CAPACITY = 6,
                ERR_NOT_FOUND = 7,
                ERR_WRONG_STATE = 8,
                ERR_GUEST_MEMORY_ALLOC_FAILED = 15,
                ERR_ALLOC_FAILED = 16,
                ERR_OUT_OF_MEMORY = 17,
                ERR_PAGE_TOO_SMALL = 18,
                ERR_MEM_OUT_OF_BOUNDS = 32,
                ERR_PARAM_LEN_MISMATCH = 48,
                ERR_PARAM_TYPE_MISMATCH = 49,
                ERR_STACK_OVERFLOW = 50,
                ERR_EOF = 64,
                ERR_MALFORMED_INTEGER = 65,
                ERR_I33_IS_NEGATIVE = 66,
                ERR_MALFORMED_MAGIC = 67,
                ERR_MALFORMED_VERSION = 68,
                ERR_MALFORMED_UTF8 = 69,
                ERR_DUPLICATE_MODULE_NAME = 70,
                ERR_DUPLICATE_EXPORT_NAME = 71,
                ERR_MALFORMED_SECTION_ID = 72,
                ERR_MALFORMED_VALUE_TYPE = 73,
                ERR_MALFORMED_FUNCTION = 74,
                ERR_MALFORMED_LIMIT = 75,
                ERR_MALFORMED_ELEM_TYPE = 76,
                ERR_MALFORMED_SECTION_SIZE = 77,
                ERR_EXPECTED_CONST_OR_VAR = 78,
                ERR_MALFORMED_IMPORT_EXPORT_DESC = 79,
                ERR_MALFORMED_MEM_TYPE = 80,
                ERR_INVALID_PAGE_SIZE = 81,
                ERR_INVALID_SECTION_ORDERING = 82,
                ERR_DUPLICATE_SECTION = 83,
                ERR_INVALID_MAX_LIMIT = 84,
                ERR_EXPECTED_TERMINAL = 85,
                ERR_INVALID_OPCODE = 86,
                ERR_MALFORMED_CODE_SIZE = 87,
                ERR_INVALID_CODE_SECTION_FUNCTION_COUNT = 88,
                ERR_VEC_TOO_LONG = 89,
                ERR_IDX_TOO_LARGE = 90,
                ERR_MODULE_IDX_TOO_LARGE = 91,
                ERR_MEMORY_TOO_LARGE = 92,
                ERR_MEMORY_IMPORT_TOO_LARGE = 93,
                ERR_MEM_ALIGN_TOO_LARGE = 94,
                ERR_CONTROL_FLOW_TOO_DEEP = 96,
                ERR_STACK_UNDERFLOW = 97,
                ERR_STACK_TOO_LARGE = 98,
                ERR_LABEL_STACK_JUMP_TOO_DEEP = 99,
                ERR_LABEL_JUMP_TOO_LARGE = 100,
                ERR_TYPE_MISMATCH = 101,
                ERR_BLOCK_RESULT_TYPE_MISMATCH = 102,
                ERR_FUNCTION_RESULT_TYPE_MISMATCH = 103,
                ERR_ILLEGAL_MEMORY_GROW = 112,
                ERR_INVALID_ELEMENT_OFFSET = 113,
                ERR_INVALID_ELEMENT_OUT_OF_BOUNDS = 114,
                ERR_INVALID_TABLE_INDEX = 115,
                ERR_TABLE_NOT_DEFINED = 116,
                ERR_INVALID_ELEMENT_COUNT = 117,
                ERR_INVALID_MEM_INDEX = 118,
                ERR_MEMORY_NOT_DEFINED = 119,
                ERR_INVALID_MEM_OFFSET_TYPE = 120,
                ERR_INVALID_NEGATIVE_MEM_OFFSET = 121,
                ERR_INVALID_MEM_OFFSET = 122,
                ERR_MULTIPLE_MEMORIES = 123,
                ERR_MULTIPLE_TABLES = 124,
                ERR_INVALID_LABEL_INDEX = 128,
                ERR_INVALID_ELSE_BLOCK = 129,
                ERR_INVALID_END_BLOCK = 130,
                ERR_INSTRUCTION_OUTSIDE_OF_FUNCTION = 131,
                ERR_LOCAL_IDX_OUT_OF_RANGE = 132,
                ERR_FUNCTION_IDX_OUT_OF_RANGE = 133,
                ERR_TYPE_IDX_OUT_OF_RANGE = 134,
                ERR_FUNCTION_TEXT_OUT_OF_RANGE = 135,
                ERR_GLOBAL_IDX_OUT_OF_RANGE = 136,
                ERR_FUNCTION_IMPORT_NOT_FOUND = 144,
                ERR_GLOBAL_IMPORT_NOT_FOUND = 145,
                ERR_MEMORY_IMPORT_NOT_FOUND = 146,
                ERR_TABLE_IMPORT_NOT_FOUND = 147,
                ERR_FUNCTION_IMPORT_OUT_OF_RANGE = 148,
                ERR_FUNCTION_IMPORT_TYPE_MISMATCH = 149,
                ERR_GLOBAL_IS_NOT_MUTABLE = 150,
                ERR_GLOBAL_IMPORT_TYPE_MISMATCH = 151,
                ERR_MEMORY_IMPORT_TYPE_MISMATCH = 152,
                ERR_TABLE_IMPORT_TYPE_MISMATCH = 153,
                ERR_TABLE_IMPORT_INCOMPATIBLE_SIZE = 154,
                ERR_FUNCTION_PARAMETERS_TOO_LARGE = 160,
                ERR_FUNCTION_RETURNS_TOO_LARGE = 161,
                ERR_TOO_MANY_LOCALS = 162,
                ERR_INVALID_CONST_INSTRUCTION = 163,
                ERR_GLOBAL_TYPE_MISMATCH = 164,
                ERR_ALIGNMENT_LARGER_THAN_TYPE = 165,
                ERR_INVALID_START_FUNCTION_SIGNATURE = 166,
                ERR_CONST_ALREADY_HAS_VALUE = 176,
                ERR_CONST_NO_VALUE = 177,
                ERR_CONST_INVALID_GLOBAL = 178,
                ERR_POSSIBLE_BACKPATCH_CYCLE = 192,
                ERR_PAGE_FAULT = 193,
                ERR_READER_ERROR = 194,
            }
            #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
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
            pub mod SequencerStateMachine {
                #[allow(unused_imports)]
                use fprime_core::*;
                #[derive(Clone, Copy, Debug, Eq, PartialEq, Serializable)]
                #[repr(u8)]
                pub enum State {
                    /// The uninitialized state
                    __FPRIME_UNINITIALIZED = 0,
                    /// Interpreter does not have any modules loaded or a store initialized
                    IDLE = 1,
                    /// Interpreter is loading a Wasm module into the store
                    LOADING = 2,
                    /// a host function paused the interpreter to dispatch an F´ command;
                    /// waiting for the command response to come back in
                    RUNNING_AWAITING_RESPONSE = 3,
                    RUNNING_PAUSED = 4,
                    /// sequencer is not taking any action, waiting for a time in the future to continue
                    RUNNING_SLEEPING = 5,
                    /// sequencer is spinning the interpreter loop
                    RUNNING_SPINNING = 6,
                    /// A module finished loading and the start function needs to be invoked/run
                    STARTING = 7,
                }
            }
        }
    }
}
mod Impl {
    #[allow(unused_imports)]
    use fprime_core::*;
    #[allow(unused_imports)]
    use super::Defs::FwOpcodeType;
    const __SCRATCH_SIZE: usize = 500usize;
    static mut __SCRATCH: [u8; __SCRATCH_SIZE] = [0x0; __SCRATCH_SIZE];
    static mut __TIME: [u8; crate::Defs::Fw::TimeValue::SIZE] = [0; crate::Defs::Fw::TimeValue::SIZE];
    pub struct CdhCoreCmdDisp {}
    impl CdhCoreCmdDisp {
        pub const DEFAULT: Self = Self {};
        /// No-op command
        #[fprime_command(opcode = 0x1000000)]
        pub fn CMD_NO_OP(&self) -> fprime_core::CmdResponse {}
        /// No-op string command
        ///
        ///  * `arg1` - The String command argument
        #[fprime_command(opcode = 0x1000001)]
        pub fn CMD_NO_OP_STRING(&self, arg1: String<40>) -> fprime_core::CmdResponse {}
        /// No-op command
        ///
        ///  * `arg1` - The I32 command argument
        ///  * `arg2` - The F32 command argument
        ///  * `arg3` - The U8 command argument
        #[fprime_command(opcode = 0x1000002)]
        pub fn CMD_TEST_CMD_1(
            &self,
            arg1: i32,
            arg2: f32,
            arg3: u8,
        ) -> fprime_core::CmdResponse {}
        /// Clear command tracking info to recover from components not returning status
        #[fprime_command(opcode = 0x1000003)]
        pub fn CMD_CLEAR_TRACKING(&self) -> fprime_core::CmdResponse {}
        /// Number of commands dispatched
        #[fprime_telemetry(id = 0x1000000)]
        pub fn CommandsDispatched(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// Number of command errors
        #[fprime_telemetry(id = 0x1000001)]
        pub fn CommandErrors(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// Number of commands drooped due to buffer overflow
        #[fprime_telemetry(id = 0x1000002)]
        pub fn CommandsDropped(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct CdhCoreEvents {}
    impl CdhCoreEvents {
        pub const DEFAULT: Self = Self {};
        /// Set filter for reporting events. Events are not stored in component.
        ///
        ///  * `filterLevel` - Filter level
        ///  * `filterEnabled` - Filter state
        #[fprime_command(opcode = 0x1001000)]
        pub fn SET_EVENT_FILTER(
            &self,
            filterLevel: crate::Defs::Svc::EventManager::FilterSeverity,
            filterEnabled: crate::Defs::Svc::EventManager::Enabled,
        ) -> fprime_core::CmdResponse {}
        /// Filter a particular ID
        ///
        ///  * `ID`
        ///  * `idFilterEnabled` - ID filter state
        #[fprime_command(opcode = 0x1001002)]
        pub fn SET_ID_FILTER(
            &self,
            ID: crate::Defs::FwEventIdType,
            idFilterEnabled: crate::Defs::Svc::EventManager::Enabled,
        ) -> fprime_core::CmdResponse {}
        /// Dump the filter states via events
        #[fprime_command(opcode = 0x1001003)]
        pub fn DUMP_FILTER_STATE(&self) -> fprime_core::CmdResponse {}
        /// Number of events dropped due to queue full
        #[fprime_telemetry(id = 0x1001000)]
        pub fn EventsDropped(
            &self,
        ) -> (crate::Defs::FwSizeType, super::Defs::Fw::TimeValue) {}
    }
    pub struct CdhCoreHealth {}
    impl CdhCoreHealth {
        pub const DEFAULT: Self = Self {};
        /// A command to enable or disable health checks
        ///
        ///  * `enable` - whether or not health checks are enabled
        #[fprime_command(opcode = 0x1002000)]
        pub fn HLTH_ENABLE(
            &self,
            enable: crate::Defs::Fw::Enabled,
        ) -> fprime_core::CmdResponse {}
        /// Ignore a particular ping entry
        ///
        ///  * `entry` - The entry to enable/disable
        ///  * `enable` - whether or not a port is pinged
        #[fprime_command(opcode = 0x1002001)]
        pub fn HLTH_PING_ENABLE(
            &self,
            entry: String<40>,
            enable: crate::Defs::Fw::Enabled,
        ) -> fprime_core::CmdResponse {}
        /// Change ping value
        ///
        ///  * `entry` - The entry to modify
        ///  * `warningValue` - Ping warning threshold
        ///  * `fatalValue` - Ping fatal threshold
        #[fprime_command(opcode = 0x1002002)]
        pub fn HLTH_CHNG_PING(
            &self,
            entry: String<40>,
            warningValue: u32,
            fatalValue: u32,
        ) -> fprime_core::CmdResponse {}
        /// Number of overrun warnings
        #[fprime_telemetry(id = 0x1002000)]
        pub fn PingLateWarnings(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct CdhCoreVersion {}
    impl CdhCoreVersion {
        pub const DEFAULT: Self = Self {};
        /// A command to enable or disable Event verbosity and Telemetry
        ///
        ///  * `enable` - whether or not Version telemetry is enabled
        #[fprime_command(opcode = 0x1003000)]
        pub fn ENABLE(
            &self,
            enable: crate::Defs::Svc::VersionEnabled,
        ) -> fprime_core::CmdResponse {}
        /// Report version as Event
        ///
        ///  * `version_type` - which version type Event is requested
        #[fprime_command(opcode = 0x1003001)]
        pub fn VERSION(
            &self,
            version_type: crate::Defs::Svc::VersionType,
        ) -> fprime_core::CmdResponse {}
        /// Software framework version
        #[fprime_telemetry(id = 0x1003000)]
        pub fn FrameworkVersion(&self) -> (String<40>, super::Defs::Fw::TimeValue) {}
        /// Software project version
        #[fprime_telemetry(id = 0x1003001)]
        pub fn ProjectVersion(&self) -> (String<40>, super::Defs::Fw::TimeValue) {}
        /// Custom Versions
        #[fprime_telemetry(id = 0x1003002)]
        pub fn CustomVersion01(
            &self,
        ) -> (crate::Defs::Svc::CustomVersionDb, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x1003003)]
        pub fn CustomVersion02(
            &self,
        ) -> (crate::Defs::Svc::CustomVersionDb, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x1003004)]
        pub fn CustomVersion03(
            &self,
        ) -> (crate::Defs::Svc::CustomVersionDb, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x1003005)]
        pub fn CustomVersion04(
            &self,
        ) -> (crate::Defs::Svc::CustomVersionDb, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x1003006)]
        pub fn CustomVersion05(
            &self,
        ) -> (crate::Defs::Svc::CustomVersionDb, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x1003007)]
        pub fn CustomVersion06(
            &self,
        ) -> (crate::Defs::Svc::CustomVersionDb, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x1003008)]
        pub fn CustomVersion07(
            &self,
        ) -> (crate::Defs::Svc::CustomVersionDb, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x1003009)]
        pub fn CustomVersion08(
            &self,
        ) -> (crate::Defs::Svc::CustomVersionDb, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x100300A)]
        pub fn CustomVersion09(
            &self,
        ) -> (crate::Defs::Svc::CustomVersionDb, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x100300B)]
        pub fn CustomVersion10(
            &self,
        ) -> (crate::Defs::Svc::CustomVersionDb, super::Defs::Fw::TimeValue) {}
        /// Library Versions
        #[fprime_telemetry(id = 0x100300C)]
        pub fn LibraryVersion01(&self) -> (String<40>, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x100300D)]
        pub fn LibraryVersion02(&self) -> (String<40>, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x100300E)]
        pub fn LibraryVersion03(&self) -> (String<40>, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x100300F)]
        pub fn LibraryVersion04(&self) -> (String<40>, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x1003010)]
        pub fn LibraryVersion05(&self) -> (String<40>, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x1003011)]
        pub fn LibraryVersion06(&self) -> (String<40>, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x1003012)]
        pub fn LibraryVersion07(&self) -> (String<40>, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x1003013)]
        pub fn LibraryVersion08(&self) -> (String<40>, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x1003014)]
        pub fn LibraryVersion09(&self) -> (String<40>, super::Defs::Fw::TimeValue) {}
        #[fprime_telemetry(id = 0x1003015)]
        pub fn LibraryVersion10(&self) -> (String<40>, super::Defs::Fw::TimeValue) {}
    }
    pub struct CdhCore {
        pub cmdDisp: CdhCoreCmdDisp,
        pub events: CdhCoreEvents,
        pub health: CdhCoreHealth,
        pub version: CdhCoreVersion,
    }
    impl CdhCore {
        pub const DEFAULT: Self = Self {
            cmdDisp: CdhCoreCmdDisp::DEFAULT,
            events: CdhCoreEvents::DEFAULT,
            health: CdhCoreHealth::DEFAULT,
            version: CdhCoreVersion::DEFAULT,
        };
    }
    pub struct ComCcsdsComQueue {}
    impl ComCcsdsComQueue {
        pub const DEFAULT: Self = Self {};
        /// Flush a specific queue. This will discard all queued data in the specified queue removing it from eventual
        /// downlink. Buffers requiring ownership return will be returned via the bufferReturnOut port.
        ///
        ///  * `queueType` - The Queue data type
        ///  * `indexType` - The index of the queue (within the supplied type) to flush
        #[fprime_command(opcode = 0x2000000)]
        pub fn FLUSH_QUEUE(
            &self,
            queueType: crate::Defs::Svc::QueueType,
            indexType: crate::Defs::FwIndexType,
        ) -> fprime_core::CmdResponse {}
        /// Flush all queues. This will discard all queued data removing it from eventual downlink. Buffers requiring
        /// ownership return will be returned via the bufferReturnOut port.
        #[fprime_command(opcode = 0x2000001)]
        pub fn FLUSH_ALL_QUEUES(&self) -> fprime_core::CmdResponse {}
        /// Set the priority of a specific queue at runtime
        ///
        ///  * `queueType` - The Queue data type
        ///  * `indexType` - The index of the queue (within the supplied type) to modify
        ///  * `newPriority` - New priority value for the queue
        #[fprime_command(opcode = 0x2000002)]
        pub fn SET_QUEUE_PRIORITY(
            &self,
            queueType: crate::Defs::Svc::QueueType,
            indexType: crate::Defs::FwIndexType,
            newPriority: crate::Defs::FwIndexType,
        ) -> fprime_core::CmdResponse {}
        /// Depth of queues of Fw::ComBuffer type
        #[fprime_telemetry(id = 0x2000000)]
        pub fn comQueueDepth(
            &self,
        ) -> (crate::Defs::Svc::ComQueueDepth, super::Defs::Fw::TimeValue) {}
        /// Depth of queues of Fw::Buffer type
        #[fprime_telemetry(id = 0x2000001)]
        pub fn buffQueueDepth(
            &self,
        ) -> (crate::Defs::Svc::BuffQueueDepth, super::Defs::Fw::TimeValue) {}
    }
    pub struct ComCcsdsCommsBufferManager {}
    impl ComCcsdsCommsBufferManager {
        pub const DEFAULT: Self = Self {};
        /// The total buffers allocated
        #[fprime_telemetry(id = 0x2002000)]
        pub fn TotalBuffs(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The current number of allocated buffers
        #[fprime_telemetry(id = 0x2002001)]
        pub fn CurrBuffs(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The high water mark of allocated buffers
        #[fprime_telemetry(id = 0x2002002)]
        pub fn HiBuffs(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The number of requests that couldn't return a buffer
        #[fprime_telemetry(id = 0x2002003)]
        pub fn NoBuffs(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The number of empty buffers returned
        #[fprime_telemetry(id = 0x2002004)]
        pub fn EmptyBuffs(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct ComCcsds {
        pub comQueue: ComCcsdsComQueue,
        pub commsBufferManager: ComCcsdsCommsBufferManager,
    }
    impl ComCcsds {
        pub const DEFAULT: Self = Self {
            comQueue: ComCcsdsComQueue::DEFAULT,
            commsBufferManager: ComCcsdsCommsBufferManager::DEFAULT,
        };
    }
    pub struct DataProductsDpBufferAccumulator {}
    impl DataProductsDpBufferAccumulator {
        pub const DEFAULT: Self = Self {};
        /// Set the mode
        ///
        ///  * `mode`
        #[fprime_command(opcode = 0x4004000)]
        pub fn BA_SetMode(
            &self,
            mode: crate::Defs::Svc::BufferAccumulator::OpState,
        ) -> fprime_core::CmdResponse {}
        /// Drain the commanded number of buffers
        ///
        ///  * `numToDrain`
        ///  * `blockMode`
        #[fprime_command(opcode = 0x4004001)]
        pub fn BA_DrainBuffers(
            &self,
            numToDrain: u32,
            blockMode: crate::Defs::Svc::BufferAccumulator::BlockMode,
        ) -> fprime_core::CmdResponse {}
        /// The number of buffers queued
        #[fprime_telemetry(id = 0x4004000)]
        pub fn BA_NumQueuedBuffers(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct DataProductsDpBufferManager {}
    impl DataProductsDpBufferManager {
        pub const DEFAULT: Self = Self {};
        /// The total buffers allocated
        #[fprime_telemetry(id = 0x4003000)]
        pub fn TotalBuffs(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The current number of allocated buffers
        #[fprime_telemetry(id = 0x4003001)]
        pub fn CurrBuffs(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The high water mark of allocated buffers
        #[fprime_telemetry(id = 0x4003002)]
        pub fn HiBuffs(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The number of requests that couldn't return a buffer
        #[fprime_telemetry(id = 0x4003003)]
        pub fn NoBuffs(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The number of empty buffers returned
        #[fprime_telemetry(id = 0x4003004)]
        pub fn EmptyBuffs(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct DataProductsDpCat {}
    impl DataProductsDpCat {
        pub const DEFAULT: Self = Self {};
        /// Build catalog from data product directory. Will block until complete
        #[fprime_command(opcode = 0x4000000)]
        pub fn BUILD_CATALOG(&self) -> fprime_core::CmdResponse {}
        /// Start transmitting catalog
        ///
        ///  * `wait` - have START_XMIT command complete wait for catalog to complete transmitting
        ///  * `remainActive` - should the catalog resume transmission when Dps are added at runtime
        #[fprime_command(opcode = 0x4000001)]
        pub fn START_XMIT_CATALOG(
            &self,
            wait: crate::Defs::Fw::Wait,
            remainActive: bool,
        ) -> fprime_core::CmdResponse {}
        /// Stop transmitting catalog
        #[fprime_command(opcode = 0x4000002)]
        pub fn STOP_XMIT_CATALOG(&self) -> fprime_core::CmdResponse {}
        /// clear existing catalog
        #[fprime_command(opcode = 0x4000003)]
        pub fn CLEAR_CATALOG(&self) -> fprime_core::CmdResponse {}
        /// Number of data products in catalog
        #[fprime_telemetry(id = 0x4000000)]
        pub fn CatalogDps(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// Number of data products sent
        #[fprime_telemetry(id = 0x4000001)]
        pub fn DpsSent(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct DataProductsDpMgr {}
    impl DataProductsDpMgr {
        pub const DEFAULT: Self = Self {};
        /// Clear event throttling
        #[fprime_command(opcode = 0x4001000)]
        pub fn CLEAR_EVENT_THROTTLE(&self) -> fprime_core::CmdResponse {}
        /// The number of successful buffer allocations
        #[fprime_telemetry(id = 0x4001000)]
        pub fn NumSuccessfulAllocations(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The number of failed buffer allocations
        #[fprime_telemetry(id = 0x4001001)]
        pub fn NumFailedAllocations(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// Number of data products handled
        #[fprime_telemetry(id = 0x4001002)]
        pub fn NumDataProducts(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// Number of bytes handled
        #[fprime_telemetry(id = 0x4001003)]
        pub fn NumBytes(&self) -> (u64, super::Defs::Fw::TimeValue) {}
    }
    pub struct DataProductsDpWriter {}
    impl DataProductsDpWriter {
        pub const DEFAULT: Self = Self {};
        /// Clear event throttling
        #[fprime_command(opcode = 0x4002000)]
        pub fn CLEAR_EVENT_THROTTLE(&self) -> fprime_core::CmdResponse {}
        /// The number of buffers received
        #[fprime_telemetry(id = 0x4002000)]
        pub fn NumBuffersReceived(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The number of bytes written
        #[fprime_telemetry(id = 0x4002001)]
        pub fn NumBytesWritten(&self) -> (u64, super::Defs::Fw::TimeValue) {}
        /// The number of successful writes
        #[fprime_telemetry(id = 0x4002002)]
        pub fn NumSuccessfulWrites(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The number of failed writes
        #[fprime_telemetry(id = 0x4002003)]
        pub fn NumFailedWrites(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The number of errors
        #[fprime_telemetry(id = 0x4002004)]
        pub fn NumErrors(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct DataProducts {
        pub dpBufferAccumulator: DataProductsDpBufferAccumulator,
        pub dpBufferManager: DataProductsDpBufferManager,
        pub dpCat: DataProductsDpCat,
        pub dpMgr: DataProductsDpMgr,
        pub dpWriter: DataProductsDpWriter,
    }
    impl DataProducts {
        pub const DEFAULT: Self = Self {
            dpBufferAccumulator: DataProductsDpBufferAccumulator::DEFAULT,
            dpBufferManager: DataProductsDpBufferManager::DEFAULT,
            dpCat: DataProductsDpCat::DEFAULT,
            dpMgr: DataProductsDpMgr::DEFAULT,
            dpWriter: DataProductsDpWriter::DEFAULT,
        };
    }
    pub struct FileHandlingFileDownlink {}
    impl FileHandlingFileDownlink {
        pub const DEFAULT: Self = Self {};
        /// Read a named file off the disk. Divide it into packets and send the packets for transmission to the ground.
        ///
        ///  * `sourceFileName` - The name of the on-board file to send
        ///  * `destFileName` - The name of the destination file on the ground
        #[fprime_command(opcode = 0x5001000)]
        pub fn SendFile(
            &self,
            sourceFileName: String<100>,
            destFileName: String<100>,
        ) -> fprime_core::CmdResponse {}
        /// Cancel the downlink in progress, if any
        #[fprime_command(opcode = 0x5001001)]
        pub fn Cancel(&self) -> fprime_core::CmdResponse {}
        /// Read a named file off the disk from a starting position. Divide it into packets and send the packets for transmission to the ground.
        ///
        ///  * `sourceFileName` - The name of the on-board file to send
        ///  * `destFileName` - The name of the destination file on the ground
        ///  * `startOffset` - Starting offset of the source file
        ///  * `length` - Number of bytes to send from starting offset. Length of 0 implies until the end of the file
        #[fprime_command(opcode = 0x5001002)]
        pub fn SendPartial(
            &self,
            sourceFileName: String<100>,
            destFileName: String<100>,
            startOffset: u32,
            length: u32,
        ) -> fprime_core::CmdResponse {}
        /// The total number of files sent
        #[fprime_telemetry(id = 0x5001000)]
        pub fn FilesSent(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The total number of packets sent
        #[fprime_telemetry(id = 0x5001001)]
        pub fn PacketsSent(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The total number of warnings
        #[fprime_telemetry(id = 0x5001002)]
        pub fn Warnings(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct FileHandlingFileManager {}
    impl FileHandlingFileManager {
        pub const DEFAULT: Self = Self {};
        /// Create a directory
        ///
        ///  * `dirName` - The directory to create
        #[fprime_command(opcode = 0x5002000)]
        pub fn CreateDirectory(&self, dirName: String<240>) -> fprime_core::CmdResponse {}
        /// Move a file
        ///
        ///  * `sourceFileName` - The source file name
        ///  * `destFileName` - The destination file name
        #[fprime_command(opcode = 0x5002001)]
        pub fn MoveFile(
            &self,
            sourceFileName: String<240>,
            destFileName: String<240>,
        ) -> fprime_core::CmdResponse {}
        /// Remove a directory, which must be empty
        ///
        ///  * `dirName` - The directory to remove
        #[fprime_command(opcode = 0x5002002)]
        pub fn RemoveDirectory(&self, dirName: String<240>) -> fprime_core::CmdResponse {}
        /// Remove a file
        ///
        ///  * `fileName` - The file to remove
        ///  * `ignoreErrors` - Ignore nonexistent files
        #[fprime_command(opcode = 0x5002003)]
        pub fn RemoveFile(
            &self,
            fileName: String<240>,
            ignoreErrors: bool,
        ) -> fprime_core::CmdResponse {}
        /// Append 1 file's contents to the end of another.
        ///
        ///  * `source` - The name of the file to take content from
        ///  * `target` - The name of the file to append to
        #[fprime_command(opcode = 0x5002005)]
        pub fn AppendFile(
            &self,
            source: String<240>,
            target: String<240>,
        ) -> fprime_core::CmdResponse {}
        /// Get the size of a file
        ///
        ///  * `fileName` - The file to get the size of
        #[fprime_command(opcode = 0x5002006)]
        pub fn FileSize(&self, fileName: String<240>) -> fprime_core::CmdResponse {}
        /// List the contents of a directory
        ///
        ///  * `dirName` - The directory to list
        #[fprime_command(opcode = 0x5002007)]
        pub fn ListDirectory(&self, dirName: String<240>) -> fprime_core::CmdResponse {}
        /// Calculate the CRC of a file
        ///
        ///  * `filename` - The file to CRC
        #[fprime_command(opcode = 0x5002008)]
        pub fn CalculateCrc(&self, filename: String<240>) -> fprime_core::CmdResponse {}
        /// The total number of commands successfully executed
        #[fprime_telemetry(id = 0x5002000)]
        pub fn CommandsExecuted(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The total number of errors
        #[fprime_telemetry(id = 0x5002001)]
        pub fn Errors(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct FileHandlingFileUplink {}
    impl FileHandlingFileUplink {
        pub const DEFAULT: Self = Self {};
        /// The total number of complete files received
        #[fprime_telemetry(id = 0x5000000)]
        pub fn FilesReceived(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The total number of packets received
        #[fprime_telemetry(id = 0x5000001)]
        pub fn PacketsReceived(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The total number of warnings issued
        #[fprime_telemetry(id = 0x5000002)]
        pub fn Warnings(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct FileHandlingPrmDb {}
    impl FileHandlingPrmDb {
        pub const DEFAULT: Self = Self {};
        /// Command to save parameter image to file. Uses file name passed to constructor
        #[fprime_command(opcode = 0x5003000)]
        pub fn PRM_SAVE_FILE(&self) -> fprime_core::CmdResponse {}
        /// Loads a file from storage into the staging database. The file could have selective IDs and not the whole set.
        ///
        ///  * `fileName` - The name of the on-board file to set parameters from
        ///  * `merge` - Whether to merge or fully reset the parameter database from the file contents
        #[fprime_command(opcode = 0x5003001)]
        pub fn PRM_LOAD_FILE(
            &self,
            fileName: String<240>,
            merge: crate::Defs::Svc::PrmDb::Merge,
        ) -> fprime_core::CmdResponse {}
        /// Commits the backup database to become the prime (active) database
        #[fprime_command(opcode = 0x5003002)]
        pub fn PRM_COMMIT_STAGED(&self) -> fprime_core::CmdResponse {}
    }
    pub struct FileHandling {
        pub fileDownlink: FileHandlingFileDownlink,
        pub fileManager: FileHandlingFileManager,
        pub fileUplink: FileHandlingFileUplink,
        pub prmDb: FileHandlingPrmDb,
    }
    impl FileHandling {
        pub const DEFAULT: Self = Self {
            fileDownlink: FileHandlingFileDownlink::DEFAULT,
            fileManager: FileHandlingFileManager::DEFAULT,
            fileUplink: FileHandlingFileUplink::DEFAULT,
            prmDb: FileHandlingPrmDb::DEFAULT,
        };
    }
    pub struct RefSg1 {}
    impl RefSg1 {
        pub const DEFAULT: Self = Self {};
        /// Signal Generator Settings
        ///
        ///  * `Frequency`
        ///  * `Amplitude`
        ///  * `Phase`
        ///  * `SigType`
        #[fprime_command(opcode = 0x10011000)]
        pub fn Settings(
            &self,
            Frequency: u32,
            Amplitude: f32,
            Phase: f32,
            SigType: crate::Defs::Ref::SignalType,
        ) -> fprime_core::CmdResponse {}
        /// Toggle Signal Generator On/Off.
        #[fprime_command(opcode = 0x10011001)]
        pub fn Toggle(&self) -> fprime_core::CmdResponse {}
        /// Skip next sample
        #[fprime_command(opcode = 0x10011002)]
        pub fn Skip(&self) -> fprime_core::CmdResponse {}
        /// Signal Generator Settings
        ///
        ///  * `reqType`
        ///  * `records`
        ///  * `priority`
        #[fprime_command(opcode = 0x10011003)]
        pub fn Dp(
            &self,
            reqType: crate::Defs::Ref::SignalGen::DpReqType,
            records: u32,
            priority: u32,
        ) -> fprime_core::CmdResponse {}
        /// Type of the output signal: SINE, TRIANGLE, etc.
        #[fprime_telemetry(id = 0x10011000)]
        pub fn Type(
            &self,
        ) -> (crate::Defs::Ref::SignalType, super::Defs::Fw::TimeValue) {}
        /// Single Y value of the output
        #[fprime_telemetry(id = 0x10011001)]
        pub fn Output(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// Single (time, value) pair of the signal
        #[fprime_telemetry(id = 0x10011002)]
        pub fn PairOutput(
            &self,
        ) -> (crate::Defs::Ref::SignalPair, super::Defs::Fw::TimeValue) {}
        /// Last 10 Y values of the signal
        #[fprime_telemetry(id = 0x10011003)]
        pub fn History(
            &self,
        ) -> (crate::Defs::Ref::SignalSet, super::Defs::Fw::TimeValue) {}
        /// Last 10 (time, value) pairs of the signal
        #[fprime_telemetry(id = 0x10011004)]
        pub fn PairHistory(
            &self,
        ) -> (crate::Defs::Ref::SignalPairSet, super::Defs::Fw::TimeValue) {}
        /// Composite field of signal information, containing histories, pairs etc
        #[fprime_telemetry(id = 0x10011005)]
        pub fn Info(
            &self,
        ) -> (crate::Defs::Ref::SignalInfo, super::Defs::Fw::TimeValue) {}
        /// DP bytes written
        #[fprime_telemetry(id = 0x10011006)]
        pub fn DpBytes(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// DP records written
        #[fprime_telemetry(id = 0x10011007)]
        pub fn DpRecords(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct RefSg2 {}
    impl RefSg2 {
        pub const DEFAULT: Self = Self {};
        /// Signal Generator Settings
        ///
        ///  * `Frequency`
        ///  * `Amplitude`
        ///  * `Phase`
        ///  * `SigType`
        #[fprime_command(opcode = 0x10012000)]
        pub fn Settings(
            &self,
            Frequency: u32,
            Amplitude: f32,
            Phase: f32,
            SigType: crate::Defs::Ref::SignalType,
        ) -> fprime_core::CmdResponse {}
        /// Toggle Signal Generator On/Off.
        #[fprime_command(opcode = 0x10012001)]
        pub fn Toggle(&self) -> fprime_core::CmdResponse {}
        /// Skip next sample
        #[fprime_command(opcode = 0x10012002)]
        pub fn Skip(&self) -> fprime_core::CmdResponse {}
        /// Signal Generator Settings
        ///
        ///  * `reqType`
        ///  * `records`
        ///  * `priority`
        #[fprime_command(opcode = 0x10012003)]
        pub fn Dp(
            &self,
            reqType: crate::Defs::Ref::SignalGen::DpReqType,
            records: u32,
            priority: u32,
        ) -> fprime_core::CmdResponse {}
        /// Type of the output signal: SINE, TRIANGLE, etc.
        #[fprime_telemetry(id = 0x10012000)]
        pub fn Type(
            &self,
        ) -> (crate::Defs::Ref::SignalType, super::Defs::Fw::TimeValue) {}
        /// Single Y value of the output
        #[fprime_telemetry(id = 0x10012001)]
        pub fn Output(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// Single (time, value) pair of the signal
        #[fprime_telemetry(id = 0x10012002)]
        pub fn PairOutput(
            &self,
        ) -> (crate::Defs::Ref::SignalPair, super::Defs::Fw::TimeValue) {}
        /// Last 10 Y values of the signal
        #[fprime_telemetry(id = 0x10012003)]
        pub fn History(
            &self,
        ) -> (crate::Defs::Ref::SignalSet, super::Defs::Fw::TimeValue) {}
        /// Last 10 (time, value) pairs of the signal
        #[fprime_telemetry(id = 0x10012004)]
        pub fn PairHistory(
            &self,
        ) -> (crate::Defs::Ref::SignalPairSet, super::Defs::Fw::TimeValue) {}
        /// Composite field of signal information, containing histories, pairs etc
        #[fprime_telemetry(id = 0x10012005)]
        pub fn Info(
            &self,
        ) -> (crate::Defs::Ref::SignalInfo, super::Defs::Fw::TimeValue) {}
        /// DP bytes written
        #[fprime_telemetry(id = 0x10012006)]
        pub fn DpBytes(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// DP records written
        #[fprime_telemetry(id = 0x10012007)]
        pub fn DpRecords(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct RefSg3 {}
    impl RefSg3 {
        pub const DEFAULT: Self = Self {};
        /// Signal Generator Settings
        ///
        ///  * `Frequency`
        ///  * `Amplitude`
        ///  * `Phase`
        ///  * `SigType`
        #[fprime_command(opcode = 0x10013000)]
        pub fn Settings(
            &self,
            Frequency: u32,
            Amplitude: f32,
            Phase: f32,
            SigType: crate::Defs::Ref::SignalType,
        ) -> fprime_core::CmdResponse {}
        /// Toggle Signal Generator On/Off.
        #[fprime_command(opcode = 0x10013001)]
        pub fn Toggle(&self) -> fprime_core::CmdResponse {}
        /// Skip next sample
        #[fprime_command(opcode = 0x10013002)]
        pub fn Skip(&self) -> fprime_core::CmdResponse {}
        /// Signal Generator Settings
        ///
        ///  * `reqType`
        ///  * `records`
        ///  * `priority`
        #[fprime_command(opcode = 0x10013003)]
        pub fn Dp(
            &self,
            reqType: crate::Defs::Ref::SignalGen::DpReqType,
            records: u32,
            priority: u32,
        ) -> fprime_core::CmdResponse {}
        /// Type of the output signal: SINE, TRIANGLE, etc.
        #[fprime_telemetry(id = 0x10013000)]
        pub fn Type(
            &self,
        ) -> (crate::Defs::Ref::SignalType, super::Defs::Fw::TimeValue) {}
        /// Single Y value of the output
        #[fprime_telemetry(id = 0x10013001)]
        pub fn Output(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// Single (time, value) pair of the signal
        #[fprime_telemetry(id = 0x10013002)]
        pub fn PairOutput(
            &self,
        ) -> (crate::Defs::Ref::SignalPair, super::Defs::Fw::TimeValue) {}
        /// Last 10 Y values of the signal
        #[fprime_telemetry(id = 0x10013003)]
        pub fn History(
            &self,
        ) -> (crate::Defs::Ref::SignalSet, super::Defs::Fw::TimeValue) {}
        /// Last 10 (time, value) pairs of the signal
        #[fprime_telemetry(id = 0x10013004)]
        pub fn PairHistory(
            &self,
        ) -> (crate::Defs::Ref::SignalPairSet, super::Defs::Fw::TimeValue) {}
        /// Composite field of signal information, containing histories, pairs etc
        #[fprime_telemetry(id = 0x10013005)]
        pub fn Info(
            &self,
        ) -> (crate::Defs::Ref::SignalInfo, super::Defs::Fw::TimeValue) {}
        /// DP bytes written
        #[fprime_telemetry(id = 0x10013006)]
        pub fn DpBytes(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// DP records written
        #[fprime_telemetry(id = 0x10013007)]
        pub fn DpRecords(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct RefSg4 {}
    impl RefSg4 {
        pub const DEFAULT: Self = Self {};
        /// Signal Generator Settings
        ///
        ///  * `Frequency`
        ///  * `Amplitude`
        ///  * `Phase`
        ///  * `SigType`
        #[fprime_command(opcode = 0x10014000)]
        pub fn Settings(
            &self,
            Frequency: u32,
            Amplitude: f32,
            Phase: f32,
            SigType: crate::Defs::Ref::SignalType,
        ) -> fprime_core::CmdResponse {}
        /// Toggle Signal Generator On/Off.
        #[fprime_command(opcode = 0x10014001)]
        pub fn Toggle(&self) -> fprime_core::CmdResponse {}
        /// Skip next sample
        #[fprime_command(opcode = 0x10014002)]
        pub fn Skip(&self) -> fprime_core::CmdResponse {}
        /// Signal Generator Settings
        ///
        ///  * `reqType`
        ///  * `records`
        ///  * `priority`
        #[fprime_command(opcode = 0x10014003)]
        pub fn Dp(
            &self,
            reqType: crate::Defs::Ref::SignalGen::DpReqType,
            records: u32,
            priority: u32,
        ) -> fprime_core::CmdResponse {}
        /// Type of the output signal: SINE, TRIANGLE, etc.
        #[fprime_telemetry(id = 0x10014000)]
        pub fn Type(
            &self,
        ) -> (crate::Defs::Ref::SignalType, super::Defs::Fw::TimeValue) {}
        /// Single Y value of the output
        #[fprime_telemetry(id = 0x10014001)]
        pub fn Output(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// Single (time, value) pair of the signal
        #[fprime_telemetry(id = 0x10014002)]
        pub fn PairOutput(
            &self,
        ) -> (crate::Defs::Ref::SignalPair, super::Defs::Fw::TimeValue) {}
        /// Last 10 Y values of the signal
        #[fprime_telemetry(id = 0x10014003)]
        pub fn History(
            &self,
        ) -> (crate::Defs::Ref::SignalSet, super::Defs::Fw::TimeValue) {}
        /// Last 10 (time, value) pairs of the signal
        #[fprime_telemetry(id = 0x10014004)]
        pub fn PairHistory(
            &self,
        ) -> (crate::Defs::Ref::SignalPairSet, super::Defs::Fw::TimeValue) {}
        /// Composite field of signal information, containing histories, pairs etc
        #[fprime_telemetry(id = 0x10014005)]
        pub fn Info(
            &self,
        ) -> (crate::Defs::Ref::SignalInfo, super::Defs::Fw::TimeValue) {}
        /// DP bytes written
        #[fprime_telemetry(id = 0x10014006)]
        pub fn DpBytes(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// DP records written
        #[fprime_telemetry(id = 0x10014007)]
        pub fn DpRecords(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct RefSg5 {}
    impl RefSg5 {
        pub const DEFAULT: Self = Self {};
        /// Signal Generator Settings
        ///
        ///  * `Frequency`
        ///  * `Amplitude`
        ///  * `Phase`
        ///  * `SigType`
        #[fprime_command(opcode = 0x10015000)]
        pub fn Settings(
            &self,
            Frequency: u32,
            Amplitude: f32,
            Phase: f32,
            SigType: crate::Defs::Ref::SignalType,
        ) -> fprime_core::CmdResponse {}
        /// Toggle Signal Generator On/Off.
        #[fprime_command(opcode = 0x10015001)]
        pub fn Toggle(&self) -> fprime_core::CmdResponse {}
        /// Skip next sample
        #[fprime_command(opcode = 0x10015002)]
        pub fn Skip(&self) -> fprime_core::CmdResponse {}
        /// Signal Generator Settings
        ///
        ///  * `reqType`
        ///  * `records`
        ///  * `priority`
        #[fprime_command(opcode = 0x10015003)]
        pub fn Dp(
            &self,
            reqType: crate::Defs::Ref::SignalGen::DpReqType,
            records: u32,
            priority: u32,
        ) -> fprime_core::CmdResponse {}
        /// Type of the output signal: SINE, TRIANGLE, etc.
        #[fprime_telemetry(id = 0x10015000)]
        pub fn Type(
            &self,
        ) -> (crate::Defs::Ref::SignalType, super::Defs::Fw::TimeValue) {}
        /// Single Y value of the output
        #[fprime_telemetry(id = 0x10015001)]
        pub fn Output(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// Single (time, value) pair of the signal
        #[fprime_telemetry(id = 0x10015002)]
        pub fn PairOutput(
            &self,
        ) -> (crate::Defs::Ref::SignalPair, super::Defs::Fw::TimeValue) {}
        /// Last 10 Y values of the signal
        #[fprime_telemetry(id = 0x10015003)]
        pub fn History(
            &self,
        ) -> (crate::Defs::Ref::SignalSet, super::Defs::Fw::TimeValue) {}
        /// Last 10 (time, value) pairs of the signal
        #[fprime_telemetry(id = 0x10015004)]
        pub fn PairHistory(
            &self,
        ) -> (crate::Defs::Ref::SignalPairSet, super::Defs::Fw::TimeValue) {}
        /// Composite field of signal information, containing histories, pairs etc
        #[fprime_telemetry(id = 0x10015005)]
        pub fn Info(
            &self,
        ) -> (crate::Defs::Ref::SignalInfo, super::Defs::Fw::TimeValue) {}
        /// DP bytes written
        #[fprime_telemetry(id = 0x10015006)]
        pub fn DpBytes(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// DP records written
        #[fprime_telemetry(id = 0x10015007)]
        pub fn DpRecords(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct RefBlockDrv {}
    impl RefBlockDrv {
        pub const DEFAULT: Self = Self {};
        /// Driver cycle count
        #[fprime_telemetry(id = 0x10000000)]
        pub fn BD_Cycles(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct RefCmdSeq {}
    impl RefCmdSeq {
        pub const DEFAULT: Self = Self {};
        /// Run a command sequence file
        ///
        ///  * `fileName` - The name of the sequence file
        ///  * `block` - Return command status when complete or not
        #[fprime_command(opcode = 0x10006000)]
        pub fn CS_RUN(
            &self,
            fileName: String<240>,
            block: crate::Defs::Svc::BlockState,
        ) -> fprime_core::CmdResponse {}
        /// Validate a command sequence file
        ///
        ///  * `fileName` - The name of the sequence file
        #[fprime_command(opcode = 0x10006001)]
        pub fn CS_VALIDATE(&self, fileName: String<240>) -> fprime_core::CmdResponse {}
        /// Cancel a command sequence
        #[fprime_command(opcode = 0x10006002)]
        pub fn CS_CANCEL(&self) -> fprime_core::CmdResponse {}
        /// Start running a command sequence
        #[fprime_command(opcode = 0x10006003)]
        pub fn CS_START(&self) -> fprime_core::CmdResponse {}
        /// Perform one step in a command sequence. Valid only if CmdSequencer is in MANUAL run mode.
        #[fprime_command(opcode = 0x10006004)]
        pub fn CS_STEP(&self) -> fprime_core::CmdResponse {}
        /// Set the run mode to AUTO.
        #[fprime_command(opcode = 0x10006005)]
        pub fn CS_AUTO(&self) -> fprime_core::CmdResponse {}
        /// Set the run mode to MANUAL.
        #[fprime_command(opcode = 0x10006006)]
        pub fn CS_MANUAL(&self) -> fprime_core::CmdResponse {}
        /// Wait for sequences that are running to finish. Allow user to run multiple seq files in SEQ_NO_BLOCK mode then wait for them to finish before allowing more seq run request.
        #[fprime_command(opcode = 0x10006007)]
        pub fn CS_JOIN_WAIT(&self) -> fprime_core::CmdResponse {}
        /// The number of Load commands executed
        #[fprime_telemetry(id = 0x10006000)]
        pub fn CS_LoadCommands(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The number of Cancel commands executed
        #[fprime_telemetry(id = 0x10006001)]
        pub fn CS_CancelCommands(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The number of errors that have occurred
        #[fprime_telemetry(id = 0x10006002)]
        pub fn CS_Errors(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The number of commands executed across all sequences.
        #[fprime_telemetry(id = 0x10006003)]
        pub fn CS_CommandsExecuted(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// The number of sequences completed.
        #[fprime_telemetry(id = 0x10006004)]
        pub fn CS_SequencesCompleted(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct RefDpDemo {}
    impl RefDpDemo {
        pub const DEFAULT: Self = Self {};
        /// Select color
        ///
        ///  * `color`
        #[fprime_command(opcode = 0xA10)]
        pub fn SelectColor(
            &self,
            color: crate::Defs::Ref::DpDemo::ColorEnum,
        ) -> fprime_core::CmdResponse {}
        /// Command for generating a DP
        ///
        ///  * `reqType`
        ///  * `priority`
        ///  * `proc`
        #[fprime_command(opcode = 0xA11)]
        pub fn Dp(
            &self,
            reqType: crate::Defs::Ref::DpDemo::DpReqType,
            priority: u32,
            proc: crate::Defs::Fw::DpCfg::ProcType,
        ) -> fprime_core::CmdResponse {}
    }
    pub struct RefPingRcvr {}
    impl RefPingRcvr {
        pub const DEFAULT: Self = Self {};
        /// Command to disable ping response
        #[fprime_command(opcode = 0x10004000)]
        pub fn PR_StopPings(&self) -> fprime_core::CmdResponse {}
        /// Number of pings received
        #[fprime_telemetry(id = 0x10004000)]
        pub fn PR_NumPings(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct RefRateGroup1Comp {}
    impl RefRateGroup1Comp {
        pub const DEFAULT: Self = Self {};
        /// Max execution time rate group
        #[fprime_telemetry(id = 0x10001000)]
        pub fn RgMaxTime(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// Cycle slips for rate group
        #[fprime_telemetry(id = 0x10001001)]
        pub fn RgCycleSlips(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct RefRateGroup2Comp {}
    impl RefRateGroup2Comp {
        pub const DEFAULT: Self = Self {};
        /// Max execution time rate group
        #[fprime_telemetry(id = 0x10002000)]
        pub fn RgMaxTime(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// Cycle slips for rate group
        #[fprime_telemetry(id = 0x10002001)]
        pub fn RgCycleSlips(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct RefRateGroup3Comp {}
    impl RefRateGroup3Comp {
        pub const DEFAULT: Self = Self {};
        /// Max execution time rate group
        #[fprime_telemetry(id = 0x10003000)]
        pub fn RgMaxTime(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// Cycle slips for rate group
        #[fprime_telemetry(id = 0x10003001)]
        pub fn RgCycleSlips(&self) -> (u32, super::Defs::Fw::TimeValue) {}
    }
    pub struct RefRecvBuffComp {}
    impl RefRecvBuffComp {
        pub const DEFAULT: Self = Self {};
        /// A test parameter
        ///
        ///  * `val`
        #[fprime_command(opcode = 0x10022000)]
        pub fn PARAMETER1_PRM_SET(&self, val: u32) -> fprime_core::CmdResponse {}
        /// A test parameter
        #[fprime_command(opcode = 0x10022001)]
        pub fn PARAMETER1_PRM_SAVE(&self) -> fprime_core::CmdResponse {}
        /// A test parameter
        ///
        ///  * `val`
        #[fprime_command(opcode = 0x10022002)]
        pub fn PARAMETER2_PRM_SET(&self, val: i16) -> fprime_core::CmdResponse {}
        /// A test parameter
        #[fprime_command(opcode = 0x10022003)]
        pub fn PARAMETER2_PRM_SAVE(&self) -> fprime_core::CmdResponse {}
        /// Packet Statistics
        #[fprime_telemetry(id = 0x10022000)]
        pub fn PktState(
            &self,
        ) -> (crate::Defs::Ref::PacketStat, super::Defs::Fw::TimeValue) {}
        /// Value of Sensor1
        #[fprime_telemetry(id = 0x10022001)]
        pub fn Sensor1(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// Value of Sensor3
        #[fprime_telemetry(id = 0x10022002)]
        pub fn Sensor2(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// Readback of Parameter1
        #[fprime_telemetry(id = 0x10022003)]
        pub fn Parameter1(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// Readback of Parameter2
        #[fprime_telemetry(id = 0x10022004)]
        pub fn Parameter2(&self) -> (i16, super::Defs::Fw::TimeValue) {}
        /// A test parameter
        #[fprime_parameter(id = 0x10022000)]
        pub fn parameter1(&self) -> u32 {}
        /// A test parameter
        #[fprime_parameter(id = 0x10022001)]
        pub fn parameter2(&self) -> i16 {}
    }
    pub struct RefSendBuffComp {}
    impl RefSendBuffComp {
        pub const DEFAULT: Self = Self {};
        /// Command to start sending packets
        #[fprime_command(opcode = 0x10010000)]
        pub fn SB_START_PKTS(&self) -> fprime_core::CmdResponse {}
        /// Send a bad packet
        #[fprime_command(opcode = 0x10010001)]
        pub fn SB_INJECT_PKT_ERROR(&self) -> fprime_core::CmdResponse {}
        /// Generate a FATAL EVR
        ///
        ///  * `arg1` - First FATAL Argument
        ///  * `arg2` - Second FATAL Argument
        ///  * `arg3` - Third FATAL Argument
        #[fprime_command(opcode = 0x10010002)]
        pub fn SB_GEN_FATAL(
            &self,
            arg1: u32,
            arg2: u32,
            arg3: u32,
        ) -> fprime_core::CmdResponse {}
        /// Generate an ASSERT
        ///
        ///  * `arg1` - First ASSERT Argument
        ///  * `arg2` - Second ASSERT Argument
        ///  * `arg3` - Third ASSERT Argument
        ///  * `arg4` - Fourth ASSERT Argument
        ///  * `arg5` - Fifth ASSERT Argument
        ///  * `arg6` - Sixth ASSERT Argument
        #[fprime_command(opcode = 0x10010003)]
        pub fn SB_GEN_ASSERT(
            &self,
            arg1: u32,
            arg2: u32,
            arg3: u32,
            arg4: u32,
            arg5: u32,
            arg6: u32,
        ) -> fprime_core::CmdResponse {}
        /// A test parameter
        ///
        ///  * `val`
        #[fprime_command(opcode = 0x1001000A)]
        pub fn PARAMETER3_PRM_SET(&self, val: u8) -> fprime_core::CmdResponse {}
        /// A test parameter
        #[fprime_command(opcode = 0x1001000B)]
        pub fn PARAMETER3_PRM_SAVE(&self) -> fprime_core::CmdResponse {}
        /// A test parameter
        ///
        ///  * `val`
        #[fprime_command(opcode = 0x1001000C)]
        pub fn PARAMETER4_PRM_SET(&self, val: f32) -> fprime_core::CmdResponse {}
        /// A test parameter
        #[fprime_command(opcode = 0x1001000D)]
        pub fn PARAMETER4_PRM_SAVE(&self) -> fprime_core::CmdResponse {}
        /// Number of packets sent
        #[fprime_telemetry(id = 0x10010000)]
        pub fn PacketsSent(&self) -> (u64, super::Defs::Fw::TimeValue) {}
        /// Number of errors injected
        #[fprime_telemetry(id = 0x10010001)]
        pub fn NumErrorsInjected(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// Readback of Parameter3
        #[fprime_telemetry(id = 0x10010002)]
        pub fn Parameter3(&self) -> (u8, super::Defs::Fw::TimeValue) {}
        /// Readback of Parameter4
        #[fprime_telemetry(id = 0x10010003)]
        pub fn Parameter4(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// Readback of Parameter4
        #[fprime_telemetry(id = 0x10010004)]
        pub fn SendState(
            &self,
        ) -> (crate::Defs::Ref::SendBuff::ActiveState, super::Defs::Fw::TimeValue) {}
        /// A test parameter
        #[fprime_parameter(id = 0x10010000)]
        pub fn parameter3(&self) -> u8 {}
        /// A test parameter
        #[fprime_parameter(id = 0x10010001)]
        pub fn parameter4(&self) -> f32 {}
    }
    pub struct RefSystemResources {}
    impl RefSystemResources {
        pub const DEFAULT: Self = Self {};
        /// A command to enable or disable system resource telemetry
        ///
        ///  * `enable` - whether or not system resource telemetry is enabled
        #[fprime_command(opcode = 0x10023000)]
        pub fn ENABLE(
            &self,
            enable: crate::Defs::Svc::SystemResourceEnabled,
        ) -> fprime_core::CmdResponse {}
        /// Total system memory in KB
        #[fprime_telemetry(id = 0x10023000)]
        pub fn MEMORY_TOTAL(&self) -> (u64, super::Defs::Fw::TimeValue) {}
        /// System memory used in KB
        #[fprime_telemetry(id = 0x10023001)]
        pub fn MEMORY_USED(&self) -> (u64, super::Defs::Fw::TimeValue) {}
        /// System non-volatile available in KB
        #[fprime_telemetry(id = 0x10023002)]
        pub fn NON_VOLATILE_TOTAL(&self) -> (u64, super::Defs::Fw::TimeValue) {}
        /// System non-volatile available in KB
        #[fprime_telemetry(id = 0x10023003)]
        pub fn NON_VOLATILE_FREE(&self) -> (u64, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x10023004)]
        pub fn CPU(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x10023005)]
        pub fn CPU_00(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x10023006)]
        pub fn CPU_01(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x10023007)]
        pub fn CPU_02(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x10023008)]
        pub fn CPU_03(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x10023009)]
        pub fn CPU_04(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x1002300A)]
        pub fn CPU_05(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x1002300B)]
        pub fn CPU_06(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x1002300C)]
        pub fn CPU_07(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x1002300D)]
        pub fn CPU_08(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x1002300E)]
        pub fn CPU_09(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x1002300F)]
        pub fn CPU_10(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x10023010)]
        pub fn CPU_11(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x10023011)]
        pub fn CPU_12(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x10023012)]
        pub fn CPU_13(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x10023013)]
        pub fn CPU_14(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// System's CPU Percentage
        #[fprime_telemetry(id = 0x10023014)]
        pub fn CPU_15(&self) -> (f32, super::Defs::Fw::TimeValue) {}
    }
    pub struct RefTypeDemo {}
    impl RefTypeDemo {
        pub const DEFAULT: Self = Self {};
        /// Single choice command
        ///
        ///  * `choice` - A single choice
        #[fprime_command(opcode = 0x10005000)]
        pub fn CHOICE(
            &self,
            choice: crate::Defs::Ref::Choice,
        ) -> fprime_core::CmdResponse {}
        /// Single enumeration parameter
        ///
        ///  * `val`
        #[fprime_command(opcode = 0x10005001)]
        pub fn CHOICE_PRM_PRM_SET(
            &self,
            val: crate::Defs::Ref::Choice,
        ) -> fprime_core::CmdResponse {}
        /// Single enumeration parameter
        #[fprime_command(opcode = 0x10005002)]
        pub fn CHOICE_PRM_PRM_SAVE(&self) -> fprime_core::CmdResponse {}
        /// Multiple choice command via Array
        ///
        ///  * `choices` - A set of choices
        #[fprime_command(opcode = 0x10005003)]
        pub fn CHOICES(
            &self,
            choices: crate::Defs::Ref::ManyChoices,
        ) -> fprime_core::CmdResponse {}
        /// Multiple choice command via Array with a preceding and following argument
        ///
        ///  * `repeat` - Number of times to repeat the choices
        ///  * `choices` - A set of choices
        ///  * `repeat_max` - Limit to the number of repetitions
        #[fprime_command(opcode = 0x10005004)]
        pub fn CHOICES_WITH_FRIENDS(
            &self,
            repeat: u8,
            choices: crate::Defs::Ref::ManyChoices,
            repeat_max: u8,
        ) -> fprime_core::CmdResponse {}
        /// Multiple enumeration parameter via Array
        ///
        ///  * `val`
        #[fprime_command(opcode = 0x10005005)]
        pub fn CHOICES_PRM_PRM_SET(
            &self,
            val: crate::Defs::Ref::ManyChoices,
        ) -> fprime_core::CmdResponse {}
        /// Multiple enumeration parameter via Array
        #[fprime_command(opcode = 0x10005006)]
        pub fn CHOICES_PRM_PRM_SAVE(&self) -> fprime_core::CmdResponse {}
        /// Too many choice command via Array
        ///
        ///  * `choices` - Way to many choices to make
        #[fprime_command(opcode = 0x10005007)]
        pub fn EXTRA_CHOICES(
            &self,
            choices: crate::Defs::Ref::TooManyChoices,
        ) -> fprime_core::CmdResponse {}
        /// Too many choices command via Array with a preceding and following argument
        ///
        ///  * `repeat` - Number of times to repeat the choices
        ///  * `choices` - Way to many choices to make
        ///  * `repeat_max` - Limit to the number of repetitions
        #[fprime_command(opcode = 0x10005008)]
        pub fn EXTRA_CHOICES_WITH_FRIENDS(
            &self,
            repeat: u8,
            choices: crate::Defs::Ref::TooManyChoices,
            repeat_max: u8,
        ) -> fprime_core::CmdResponse {}
        /// Too many enumeration parameter via Array
        ///
        ///  * `val`
        #[fprime_command(opcode = 0x10005009)]
        pub fn EXTRA_CHOICES_PRM_PRM_SET(
            &self,
            val: crate::Defs::Ref::ManyChoices,
        ) -> fprime_core::CmdResponse {}
        /// Too many enumeration parameter via Array
        #[fprime_command(opcode = 0x1000500A)]
        pub fn EXTRA_CHOICES_PRM_PRM_SAVE(&self) -> fprime_core::CmdResponse {}
        /// Multiple choice command via Structure
        ///
        ///  * `choices` - A pair of choices
        #[fprime_command(opcode = 0x1000500B)]
        pub fn CHOICE_PAIR(
            &self,
            choices: crate::Defs::Ref::ChoicePair,
        ) -> fprime_core::CmdResponse {}
        /// Multiple choices command via Structure with a preceding and following argument
        ///
        ///  * `repeat` - Number of times to repeat the choices
        ///  * `choices` - A pair of choices
        ///  * `repeat_max` - Limit to the number of repetitions
        #[fprime_command(opcode = 0x1000500C)]
        pub fn CHOICE_PAIR_WITH_FRIENDS(
            &self,
            repeat: u8,
            choices: crate::Defs::Ref::ChoicePair,
            repeat_max: u8,
        ) -> fprime_core::CmdResponse {}
        /// Multiple enumeration parameter via Structure
        ///
        ///  * `val`
        #[fprime_command(opcode = 0x1000500D)]
        pub fn CHOICE_PAIR_PRM_PRM_SET(
            &self,
            val: crate::Defs::Ref::ChoicePair,
        ) -> fprime_core::CmdResponse {}
        /// Multiple enumeration parameter via Structure
        #[fprime_command(opcode = 0x1000500E)]
        pub fn CHOICE_PAIR_PRM_PRM_SAVE(&self) -> fprime_core::CmdResponse {}
        /// Multiple choice command via Complex Structure
        ///
        ///  * `choices` - A phenomenal amount of choice
        #[fprime_command(opcode = 0x1000500F)]
        pub fn GLUTTON_OF_CHOICE(
            &self,
            choices: crate::Defs::Ref::ChoiceSlurry,
        ) -> fprime_core::CmdResponse {}
        /// Multiple choices command via Complex Structure with a preceding and following argument
        ///
        ///  * `repeat` - Number of times to repeat the choices
        ///  * `choices` - A phenomenal amount of choice
        ///  * `repeat_max` - Limit to the number of repetitions
        #[fprime_command(opcode = 0x10005010)]
        pub fn GLUTTON_OF_CHOICE_WITH_FRIENDS(
            &self,
            repeat: u8,
            choices: crate::Defs::Ref::ChoiceSlurry,
            repeat_max: u8,
        ) -> fprime_core::CmdResponse {}
        /// Multiple enumeration parameter via Complex Structure
        ///
        ///  * `val`
        #[fprime_command(opcode = 0x10005011)]
        pub fn GLUTTON_OF_CHOICE_PRM_PRM_SET(
            &self,
            val: crate::Defs::Ref::ChoiceSlurry,
        ) -> fprime_core::CmdResponse {}
        /// Multiple enumeration parameter via Complex Structure
        #[fprime_command(opcode = 0x10005012)]
        pub fn GLUTTON_OF_CHOICE_PRM_PRM_SAVE(&self) -> fprime_core::CmdResponse {}
        /// Dump the typed parameters
        #[fprime_command(opcode = 0x10005013)]
        pub fn DUMP_TYPED_PARAMETERS(&self) -> fprime_core::CmdResponse {}
        /// Dump the float values
        #[fprime_command(opcode = 0x10005014)]
        pub fn DUMP_FLOATS(&self) -> fprime_core::CmdResponse {}
        /// Send scalars
        ///
        ///  * `scalar_input`
        #[fprime_command(opcode = 0x10005015)]
        pub fn SEND_SCALARS(
            &self,
            scalar_input: crate::Defs::Ref::ScalarStruct,
        ) -> fprime_core::CmdResponse {}
        /// Single choice channel
        #[fprime_telemetry(id = 0x10005000)]
        pub fn ChoiceCh(
            &self,
        ) -> (crate::Defs::Ref::Choice, super::Defs::Fw::TimeValue) {}
        /// Multiple choice channel via Array
        #[fprime_telemetry(id = 0x10005001)]
        pub fn ChoicesCh(
            &self,
        ) -> (crate::Defs::Ref::ManyChoices, super::Defs::Fw::TimeValue) {}
        /// Too many choice channel via Array
        #[fprime_telemetry(id = 0x10005002)]
        pub fn ExtraChoicesCh(
            &self,
        ) -> (crate::Defs::Ref::TooManyChoices, super::Defs::Fw::TimeValue) {}
        /// Multiple choice channel via Structure
        #[fprime_telemetry(id = 0x10005003)]
        pub fn ChoicePairCh(
            &self,
        ) -> (crate::Defs::Ref::ChoicePair, super::Defs::Fw::TimeValue) {}
        /// Multiple choice channel via Complex Structure
        #[fprime_telemetry(id = 0x10005004)]
        pub fn ChoiceSlurryCh(
            &self,
        ) -> (crate::Defs::Ref::ChoiceSlurry, super::Defs::Fw::TimeValue) {}
        /// Float output channel 1
        #[fprime_telemetry(id = 0x10005005)]
        pub fn Float1Ch(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// Float output channel 2
        #[fprime_telemetry(id = 0x10005006)]
        pub fn Float2Ch(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// Float output channel 3
        #[fprime_telemetry(id = 0x10005007)]
        pub fn Float3Ch(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// Float set output channel
        #[fprime_telemetry(id = 0x10005008)]
        pub fn FloatSet(
            &self,
        ) -> (crate::Defs::Ref::FloatSet, super::Defs::Fw::TimeValue) {}
        /// Scalar struct channel
        #[fprime_telemetry(id = 0x10005009)]
        pub fn ScalarStructCh(
            &self,
        ) -> (crate::Defs::Ref::ScalarStruct, super::Defs::Fw::TimeValue) {}
        /// Scalar U8 channel
        #[fprime_telemetry(id = 0x1000500A)]
        pub fn ScalarU8Ch(&self) -> (u8, super::Defs::Fw::TimeValue) {}
        /// Scalar U16 channel
        #[fprime_telemetry(id = 0x1000500B)]
        pub fn ScalarU16Ch(&self) -> (u16, super::Defs::Fw::TimeValue) {}
        /// Scalar U32 channel
        #[fprime_telemetry(id = 0x1000500C)]
        pub fn ScalarU32Ch(&self) -> (u32, super::Defs::Fw::TimeValue) {}
        /// Scalar U64 channel
        #[fprime_telemetry(id = 0x1000500D)]
        pub fn ScalarU64Ch(&self) -> (u64, super::Defs::Fw::TimeValue) {}
        /// Scalar I8 channel
        #[fprime_telemetry(id = 0x1000500E)]
        pub fn ScalarI8Ch(&self) -> (i8, super::Defs::Fw::TimeValue) {}
        /// Scalar I16 channel
        #[fprime_telemetry(id = 0x1000500F)]
        pub fn ScalarI16Ch(&self) -> (i16, super::Defs::Fw::TimeValue) {}
        /// Scalar I32 channel
        #[fprime_telemetry(id = 0x10005010)]
        pub fn ScalarI32Ch(&self) -> (i32, super::Defs::Fw::TimeValue) {}
        /// Scalar I64 channel
        #[fprime_telemetry(id = 0x10005011)]
        pub fn ScalarI64Ch(&self) -> (i64, super::Defs::Fw::TimeValue) {}
        /// Scalar F32 channel
        #[fprime_telemetry(id = 0x10005012)]
        pub fn ScalarF32Ch(&self) -> (f32, super::Defs::Fw::TimeValue) {}
        /// Scalar F64 channel
        #[fprime_telemetry(id = 0x10005013)]
        pub fn ScalarF64Ch(&self) -> (f64, super::Defs::Fw::TimeValue) {}
        /// Single enumeration parameter
        #[fprime_parameter(id = 0x10005000)]
        pub fn CHOICE_PRM(&self) -> crate::Defs::Ref::Choice {}
        /// Multiple enumeration parameter via Array
        #[fprime_parameter(id = 0x10005001)]
        pub fn CHOICES_PRM(&self) -> crate::Defs::Ref::ManyChoices {}
        /// Too many enumeration parameter via Array
        #[fprime_parameter(id = 0x10005002)]
        pub fn EXTRA_CHOICES_PRM(&self) -> crate::Defs::Ref::ManyChoices {}
        /// Multiple enumeration parameter via Structure
        #[fprime_parameter(id = 0x10005003)]
        pub fn CHOICE_PAIR_PRM(&self) -> crate::Defs::Ref::ChoicePair {}
        /// Multiple enumeration parameter via Complex Structure
        #[fprime_parameter(id = 0x10005004)]
        pub fn GLUTTON_OF_CHOICE_PRM(&self) -> crate::Defs::Ref::ChoiceSlurry {}
    }
    pub struct RefWasmSeq {}
    impl RefWasmSeq {
        pub const DEFAULT: Self = Self {};
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
        #[fprime_command(opcode = 0x10007000)]
        pub fn RUN(
            &self,
            fileName: String<240>,
            block: crate::Defs::Svc::BlockState,
        ) -> fprime_core::CmdResponse {}
        /// Wait for the interpreter to finish and return it's result as a CmdResponse
        #[fprime_command(opcode = 0x10007001)]
        pub fn WAIT(&self) -> fprime_core::CmdResponse {}
        /// Loads and validates a WebAssembly module into the store.
        /// This command loads the module with a empty name meaning only a single module may be loaded.
        /// To allow multiple modules, use the `LOAD_NAME` command instead.
        ///
        ///  * `fileName` - The name of the sequence file
        #[fprime_command(opcode = 0x10007002)]
        pub fn LOAD(&self, fileName: String<240>) -> fprime_core::CmdResponse {}
        /// Load and validate a WebAssembly module into the store. This module is given a name so that
        /// it's exports can be referenced by other modules.
        ///
        ///  * `fileName` - The name of the sequence file
        ///  * `name` - WebAssembly module name, must not conflict with previously loaded modules
        #[fprime_command(opcode = 0x10007003)]
        pub fn LOAD_NAME(
            &self,
            fileName: String<240>,
            name: String<16>,
        ) -> fprime_core::CmdResponse {}
        /// Invoke a main function from a loaded module
        ///
        ///  * `module` - Name of the module to invoke a function from
        ///  * `block`
        #[fprime_command(opcode = 0x10007004)]
        pub fn INVOKE(
            &self,
            module: String<16>,
            block: crate::Defs::Svc::BlockState,
        ) -> fprime_core::CmdResponse {}
        /// Cancels a running or validated sequence. After running CANCEL, the sequencer should return to IDLE
        /// This completely clears the store.
        /// Cancelling during LOADING will trigger a fail response in the reader and
        /// return to IDLE mode.
        #[fprime_command(opcode = 0x10007005)]
        pub fn CANCEL(&self) -> fprime_core::CmdResponse {}
        /// Pauses the execution of the sequencer, just before it is about to dispatch the next directive,
        /// until unpaused by the CONTINUE command, or stepped by the STEP command. This command is only valid
        /// substates of the RUNNING state that are not RUNNING.PAUSED.
        #[fprime_command(opcode = 0x10007006)]
        pub fn PAUSE(&self) -> fprime_core::CmdResponse {}
        /// Dump a stack trace in events
        #[fprime_command(opcode = 0x10007007)]
        pub fn TRACE(&self) -> fprime_core::CmdResponse {}
        #[fprime_command(opcode = 0x10007008)]
        pub fn CONTINUE(&self) -> fprime_core::CmdResponse {}
        /// the number of seconds to wait before giving up
        /// on a directive or command. if <= 0 or greater than U32 max, never time out.
        /// accuracy of this timeout is determined by the rate group driving this
        /// component. it will be rounded up
        ///
        ///  * `val`
        #[fprime_command(opcode = 0x10007009)]
        pub fn STATEMENT_TIMEOUT_SECS_PRM_SET(
            &self,
            val: f32,
        ) -> fprime_core::CmdResponse {}
        /// the number of seconds to wait before giving up
        /// on a directive or command. if <= 0 or greater than U32 max, never time out.
        /// accuracy of this timeout is determined by the rate group driving this
        /// component. it will be rounded up
        #[fprime_command(opcode = 0x1000700A)]
        pub fn STATEMENT_TIMEOUT_SECS_PRM_SAVE(&self) -> fprime_core::CmdResponse {}
        /// Number of Wasm instructions to execute per interpreter cycle.
        /// Larger numbers will effect the responsiveness of the interpreter to
        /// `PAUSE` commands. The interpreter will always pause before commands.
        ///
        /// Larger numbers make the interpreter faster because the engine doesn't
        /// need to feed itself through a queue to fuel more cycles.
        ///
        ///  * `val`
        #[fprime_command(opcode = 0x1000700B)]
        pub fn INSTRUCTION_FUEL_PRM_SET(
            &self,
            val: crate::Defs::FwSizeType,
        ) -> fprime_core::CmdResponse {}
        /// Number of Wasm instructions to execute per interpreter cycle.
        /// Larger numbers will effect the responsiveness of the interpreter to
        /// `PAUSE` commands. The interpreter will always pause before commands.
        ///
        /// Larger numbers make the interpreter faster because the engine doesn't
        /// need to feed itself through a queue to fuel more cycles.
        #[fprime_command(opcode = 0x1000700C)]
        pub fn INSTRUCTION_FUEL_PRM_SAVE(&self) -> fprime_core::CmdResponse {}
        /// the number of seconds to wait before giving up
        /// on a directive or command. if <= 0 or greater than U32 max, never time out.
        /// accuracy of this timeout is determined by the rate group driving this
        /// component. it will be rounded up
        #[fprime_parameter(id = 0x10007000)]
        pub fn STATEMENT_TIMEOUT_SECS(&self) -> f32 {}
        /// Number of Wasm instructions to execute per interpreter cycle.
        /// Larger numbers will effect the responsiveness of the interpreter to
        /// `PAUSE` commands. The interpreter will always pause before commands.
        ///
        /// Larger numbers make the interpreter faster because the engine doesn't
        /// need to feed itself through a queue to fuel more cycles.
        #[fprime_parameter(id = 0x10007001)]
        pub fn INSTRUCTION_FUEL(&self) -> crate::Defs::FwSizeType {}
    }
    pub struct Ref {
        pub SG1: RefSg1,
        pub SG2: RefSg2,
        pub SG3: RefSg3,
        pub SG4: RefSg4,
        pub SG5: RefSg5,
        pub blockDrv: RefBlockDrv,
        pub cmdSeq: RefCmdSeq,
        pub dpDemo: RefDpDemo,
        pub pingRcvr: RefPingRcvr,
        pub rateGroup1Comp: RefRateGroup1Comp,
        pub rateGroup2Comp: RefRateGroup2Comp,
        pub rateGroup3Comp: RefRateGroup3Comp,
        pub recvBuffComp: RefRecvBuffComp,
        pub sendBuffComp: RefSendBuffComp,
        pub systemResources: RefSystemResources,
        pub typeDemo: RefTypeDemo,
        pub wasmSeq: RefWasmSeq,
    }
    impl Ref {
        pub const DEFAULT: Self = Self {
            SG1: RefSg1::DEFAULT,
            SG2: RefSg2::DEFAULT,
            SG3: RefSg3::DEFAULT,
            SG4: RefSg4::DEFAULT,
            SG5: RefSg5::DEFAULT,
            blockDrv: RefBlockDrv::DEFAULT,
            cmdSeq: RefCmdSeq::DEFAULT,
            dpDemo: RefDpDemo::DEFAULT,
            pingRcvr: RefPingRcvr::DEFAULT,
            rateGroup1Comp: RefRateGroup1Comp::DEFAULT,
            rateGroup2Comp: RefRateGroup2Comp::DEFAULT,
            rateGroup3Comp: RefRateGroup3Comp::DEFAULT,
            recvBuffComp: RefRecvBuffComp::DEFAULT,
            sendBuffComp: RefSendBuffComp::DEFAULT,
            systemResources: RefSystemResources::DEFAULT,
            typeDemo: RefTypeDemo::DEFAULT,
            wasmSeq: RefWasmSeq::DEFAULT,
        };
    }
}
pub const CdhCore: Impl::CdhCore = Impl::CdhCore::DEFAULT;
pub const ComCcsds: Impl::ComCcsds = Impl::ComCcsds::DEFAULT;
pub const DataProducts: Impl::DataProducts = Impl::DataProducts::DEFAULT;
pub const FileHandling: Impl::FileHandling = Impl::FileHandling::DEFAULT;
pub const Ref: Impl::Ref = Impl::Ref::DEFAULT;
