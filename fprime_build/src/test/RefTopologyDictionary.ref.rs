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
pub mod fw {
    /// Enum representing a command response
    #[derive(Clone, Debug)]
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
    #[derive(Clone, Debug)]
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
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum ParamValid {
        Uninit = 0,
        Valid = 1,
        Invalid = 2,
        Default = 3,
    }
    /// Deserialization status
    #[derive(Clone, Debug)]
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
    #[derive(Clone, Debug)]
    #[repr(u8)]
    pub enum Wait {
        /// Wait for something
        Wait = 0,
        /// Don't wait for something
        NoWait = 1,
    }
    /// Enabled and disabled states
    #[derive(Clone, Debug)]
    #[repr(u8)]
    pub enum Enabled {
        /// Disabled state
        Disabled = 0,
        /// Enabled state
        Enabled = 1,
    }
    pub mod dp_cfg {
        /// A bit mask for selecting the type of processing to perform on
        /// a container before writing it to disk.
        #[derive(Clone, Debug)]
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
pub mod svc {
    /// An enumeration for Version Type
    #[derive(Clone, Debug)]
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
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum VersionStatus {
        /// Version was good
        Ok = 0,
        /// Failure to get version
        Failure = 1,
    }
    /// Tracks versions for project, framework and user defined versions etc
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum VersionEnabled {
        /// verbosity disabled
        Disabled = 0,
        /// verbosity enabled
        Enabled = 1,
    }
    /// Data structure representing a data product.
    #[derive(Clone, Debug)]
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
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum SendFileStatus {
        StatusOk = 0,
        StatusError = 1,
        StatusInvalid = 2,
        StatusBusy = 3,
    }
    /// An enumeration of queue data types
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum QueueType {
        ComQueue = 0,
        BufferQueue = 1,
    }
    /// Header validation error
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum DpHdrField {
        Descriptor = 0,
        Id = 1,
        Priority = 2,
        Crc = 3,
    }
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum SystemResourceEnabled {
        Disabled = 0,
        Enabled = 1,
    }
    /// Data Structure for custom version Tlm
    #[derive(Clone, Debug)]
    pub struct CustomVersionDb {
        /// enumeration/name of the custom version
        pub version_enum: crate::svc::version_cfg::VersionEnum,
        /// string containing custom version
        pub version_value: heapless::String<80>,
        /// status of the custom version
        pub version_status: crate::svc::VersionStatus,
    }
    pub mod cmd_sequencer {
        /// The stage of the file read operation
        #[derive(Clone, Debug)]
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
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum BlockState {
            Block = 0,
            NoBlock = 1,
        }
        /// The sequencer mode
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum SeqMode {
            Step = 0,
            Auto = 1,
        }
    }
    pub mod event_manager {
        /// Severity level for event filtering
        /// Similar to Fw::LogSeverity, but no FATAL event
        #[derive(Clone, Debug)]
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
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum Enabled {
            /// Enabled state
            Enabled = 0,
            /// Disabled state
            Disabled = 1,
        }
    }
    pub mod prm_db {
        /// Parameter read error
        #[derive(Clone, Debug)]
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
        #[derive(Clone, Debug)]
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
        /// Define a set of Version entries on a project-specific
        /// basis.
        #[derive(Clone, Debug)]
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
pub mod r#ref {
    #[derive(Clone, Debug)]
    pub struct SignalPair {
        pub time: f32,
        pub value: f32,
    }
    /// Array of array
    pub type TooManyChoices = [crate::r#ref::ManyChoices; 2];
    /// Some Packet Statistics
    #[derive(Clone, Debug)]
    pub struct PacketStat {
        /// Number of buffers received
        pub buff_recv: u32,
        /// Number of buffers received with errors
        pub buff_err: u32,
        /// Packet Status
        pub packet_status: crate::r#ref::PacketRecvStatus,
    }
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum SignalType {
        Triangle = 0,
        Square = 1,
        Sine = 2,
        Noise = 3,
    }
    pub type SignalPairSet = [crate::r#ref::SignalPair; 4];
    /// Enumeration type for use later
    #[derive(Clone, Debug)]
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
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum PacketRecvStatus {
        PacketStateNoPackets = 0,
        PacketStateOk = 1,
        /// Receiver has seen errors
        PacketStateErrors = 3,
    }
    /// Structure of enums (with an multi-dimensional array and structure)
    #[derive(Clone, Debug)]
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
    #[derive(Clone, Debug)]
    pub struct ChoicePair {
        /// The first choice to make
        pub first_choice: crate::r#ref::Choice,
        /// The second choice to make
        pub second_choice: crate::r#ref::Choice,
    }
    /// All scalar inputs
    #[derive(Clone, Debug)]
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
    #[derive(Clone, Debug)]
    pub struct SignalInfo {
        pub r#type: crate::r#ref::SignalType,
        pub history: crate::r#ref::SignalSet,
        pub pair_history: crate::r#ref::SignalPairSet,
    }
    pub mod dp_demo {
        pub type BoolAlias = bool;
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum ColorEnum {
            Red = 0,
            Green = 1,
            Blue = 2,
        }
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum DpReqType {
            Immediate = 0,
            Async = 1,
        }
        #[derive(Clone, Debug)]
        pub struct StructWithEverything {
            pub integer_member: crate::r#ref::dp_demo::I32Alias,
            pub float_member: f32,
            pub string_member: heapless::String<80>,
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
        #[derive(Clone, Debug)]
        pub struct ColorInfoStruct {
            pub color: crate::r#ref::dp_demo::ColorEnum,
        }
        /// Array of strings
        pub type StringArray = [heapless::String<80>; 2];
        pub type StringAlias = heapless::String<80>;
        pub type I32Alias = i32;
        #[derive(Clone, Debug)]
        pub struct StructWithStringMembers {
            pub string_member: heapless::String<80>,
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
    }
    pub mod signal_gen {
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum DpReqType {
            Immediate = 0,
            Async = 1,
        }
    }
    pub mod send_buff {
        /// Active state
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum ActiveState {
            SendIdle = 0,
            SendActive = 1,
        }
    }
}
