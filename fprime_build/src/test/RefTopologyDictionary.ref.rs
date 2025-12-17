/// The type of a telemetry packet identifier
pub type FwTlmPacketizeIdType = u16;
/// The width of packet descriptors when they are serialized by the framework
pub type FwPacketDescriptorType = u16;
/// The type of a parameter identifier
pub type FwPrmIdType = FwIdType;
/// The type of a data product priority
pub type FwDpPriorityType = u32;
/// The unsigned type of larger sizes internal to the software,
/// e.g., memory buffer sizes, file sizes. Must be unsigned.
/// Supplied by platform, overridable by project.
pub type PlatformSizeType = u64;
/// The type of a telemetry channel identifier
pub type FwChanIdType = FwIdType;
/// The type used to serialize a time context value
pub type FwTimeContextStoreType = u8;
/// The type of a data product identifier
pub type FwDpIdType = FwIdType;
/// The id type.
pub type FwIdType = u32;
/// The type of a command opcode
pub type FwOpcodeType = FwIdType;
/// The unsigned type of larger sizes internal to the software,
/// e.g., memory buffer sizes, file sizes. Must be unsigned.
pub type FwSizeType = PlatformSizeType;
/// The type used to serialize a size value
pub type FwSizeStoreType = u16;
/// The type used to serialize a time base value
pub type FwTimeBaseStoreType = u16;
/// The type of an event identifier
pub type FwEventIdType = FwIdType;
mod fw {
    /// Enum representing a command response
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum CmdResponse {
        /// Command successfully executed
        Ok = 0i64,
        /// Invalid opcode dispatched
        InvalidOpcode = 1i64,
        /// Command failed validation
        ValidationError = 2i64,
        /// Command failed to deserialize
        FormatError = 3i64,
        /// Command had execution error
        ExecutionError = 4i64,
        /// Component busy
        Busy = 5i64,
    }
    #[derive(Clone, Debug)]
    #[repr(u8)]
    pub enum DpState {
        /// The untransmitted state
        Untransmitted = 0i64,
        /// The partially transmitted state
        /// A data product is in this state from the start of transmission
        /// until transmission is complete.
        Partial = 1i64,
        /// The transmitted state
        Transmitted = 2i64,
    }
    /// Enum representing parameter validity
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum ParamValid {
        Uninit = 0i64,
        Valid = 1i64,
        Invalid = 2i64,
        Default = 3i64,
    }
    /// Deserialization status
    #[derive(Clone, Debug)]
    #[repr(u8)]
    pub enum DeserialStatus {
        Ok = 0i64,
        /// Deserialization buffer was empty when trying to read data
        BufferEmpty = 3i64,
        /// Deserialization data had incorrect values (unexpected data types)
        FormatError = 4i64,
        /// Data was left in in the buffer, but not enough to deserialize
        SizeMismatch = 5i64,
        /// Deserialized type ID didn't match
        TypeMismatch = 6i64,
    }
    /// Wait or don't wait for something
    #[derive(Clone, Debug)]
    #[repr(u8)]
    pub enum Wait {
        /// Wait for something
        Wait = 0i64,
        /// Don't wait for something
        NoWait = 1i64,
    }
    /// Enabled and disabled states
    #[derive(Clone, Debug)]
    #[repr(u8)]
    pub enum Enabled {
        /// Disabled state
        Disabled = 0i64,
        /// Enabled state
        Enabled = 1i64,
    }
    mod dp_cfg {
        /// A bit mask for selecting the type of processing to perform on
        /// a container before writing it to disk.
        #[derive(Clone, Debug)]
        #[repr(u8)]
        pub enum ProcType {
            /// Processing type 0
            ProcTypeZero = 1i64,
            /// Processing type 1
            ProcTypeOne = 2i64,
            /// Processing type 2
            ProcTypeTwo = 4i64,
        }
    }
}
mod svc {
    /// An enumeration for Version Type
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum VersionType {
        /// project version
        Project = 0i64,
        /// framework version
        Framework = 1i64,
        /// library version
        Library = 2i64,
        /// custom version
        Custom = 3i64,
        /// all above versions
        All = 4i64,
    }
    /// Array of queue depths for Fw::Com types
    #[derive(Clone, Debug)]
    pub struct ComQueueDepth([u32; 2u32]);
    /// Array of queue depths for Fw::Buffer types
    #[derive(Clone, Debug)]
    pub struct BuffQueueDepth([u32; 1u32]);
    /// An enumeration for version status
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum VersionStatus {
        /// Version was good
        Ok = 0i64,
        /// Failure to get version
        Failure = 1i64,
    }
    /// Tracks versions for project, framework and user defined versions etc
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum VersionEnabled {
        /// verbosity disabled
        Disabled = 0i64,
        /// verbosity enabled
        Enabled = 1i64,
    }
    /// Data structure representing a data product.
    #[derive(Clone, Debug)]
    pub struct DpRecord {
        pub id: FwDpIdType,
        pub t_sec: u32,
        pub t_sub: u32,
        pub priority: u32,
        pub size: u64,
        pub blocks: u32,
        pub state: fw::DpState,
    }
    /// Send file status enum
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum SendFileStatus {
        StatusOk = 0i64,
        StatusError = 1i64,
        StatusInvalid = 2i64,
        StatusBusy = 3i64,
    }
    /// An enumeration of queue data types
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum QueueType {
        ComQueue = 0i64,
        BufferQueue = 1i64,
    }
    /// Header validation error
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum DpHdrField {
        Descriptor = 0i64,
        Id = 1i64,
        Priority = 2i64,
        Crc = 3i64,
    }
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum SystemResourceEnabled {
        Disabled = 0i64,
        Enabled = 1i64,
    }
    /// Data Structure for custom version Tlm
    #[derive(Clone, Debug)]
    pub struct CustomVersionDb {
        /// enumeration/name of the custom version
        pub version_enum: svc::version_cfg::VersionEnum,
        /// string containing custom version
        pub version_value: heapless::String<80u32>,
        /// status of the custom version
        pub version_status: svc::VersionStatus,
    }
    mod prm_db {
        /// Parameter read error
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum PrmReadError {
            Open = 0i64,
            Delimiter = 1i64,
            DelimiterSize = 2i64,
            DelimiterValue = 3i64,
            RecordSize = 4i64,
            RecordSizeSize = 5i64,
            RecordSizeValue = 6i64,
            ParameterId = 7i64,
            ParameterIdSize = 8i64,
            ParameterValue = 9i64,
            ParameterValueSize = 10i64,
        }
        /// Parameter write error
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum PrmWriteError {
            Open = 0i64,
            Delimiter = 1i64,
            DelimiterSize = 2i64,
            RecordSize = 3i64,
            RecordSizeSize = 4i64,
            ParameterId = 5i64,
            ParameterIdSize = 6i64,
            ParameterValue = 7i64,
            ParameterValueSize = 8i64,
        }
    }
    mod version_cfg {
        /// Define a set of Version entries on a project-specific
        /// basis.
        #[derive(Clone, Debug)]
        #[repr(u32)]
        pub enum VersionEnum {
            /// Entry 0
            ProjectVersion00 = 0i64,
            /// Entry 1
            ProjectVersion01 = 1i64,
            /// Entry 2
            ProjectVersion02 = 2i64,
            /// Entry 3
            ProjectVersion03 = 3i64,
            /// Entry 4
            ProjectVersion04 = 4i64,
            /// Entry 5
            ProjectVersion05 = 5i64,
            /// Entry 6
            ProjectVersion06 = 6i64,
            /// Entry 7
            ProjectVersion07 = 7i64,
            /// Entry 8
            ProjectVersion08 = 8i64,
            /// Entry 9
            ProjectVersion09 = 9i64,
        }
    }
    mod event_manager {
        /// Severity level for event filtering
        /// Similar to Fw::LogSeverity, but no FATAL event
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum FilterSeverity {
            /// Filter WARNING_HI events
            WarningHi = 0i64,
            /// Filter WARNING_LO events
            WarningLo = 1i64,
            /// Filter COMMAND events
            Command = 2i64,
            /// Filter ACTIVITY_HI events
            ActivityHi = 3i64,
            /// Filter ACTIVITY_LO events
            ActivityLo = 4i64,
            /// Filter DIAGNOSTIC events
            Diagnostic = 5i64,
        }
        /// Enabled and disabled state
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum Enabled {
            /// Enabled state
            Enabled = 0i64,
            /// Disabled state
            Disabled = 1i64,
        }
    }
    mod cmd_sequencer {
        /// The stage of the file read operation
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum FileReadStage {
            ReadHeader = 0i64,
            ReadHeaderSize = 1i64,
            DeserSize = 2i64,
            DeserNumRecords = 3i64,
            DeserTimeBase = 4i64,
            DeserTimeContext = 5i64,
            ReadSeqCrc = 6i64,
            ReadSeqData = 7i64,
            ReadSeqDataSize = 8i64,
        }
        /// Sequencer blocking state
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum BlockState {
            Block = 0i64,
            NoBlock = 1i64,
        }
        /// The sequencer mode
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum SeqMode {
            Step = 0i64,
            Auto = 1i64,
        }
    }
}
mod r#ref {
    #[derive(Clone, Debug)]
    pub struct SignalPair {
        pub time: f32,
        pub value: f32,
    }
    /// Array of array
    #[derive(Clone, Debug)]
    pub struct TooManyChoices([r#ref::ManyChoices; 2u32]);
    /// Some Packet Statistics
    #[derive(Clone, Debug)]
    pub struct PacketStat {
        /// Number of buffers received
        pub buff_recv: u32,
        /// Number of buffers received with errors
        pub buff_err: u32,
        /// Packet Status
        pub packet_status: r#ref::PacketRecvStatus,
    }
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum SignalType {
        Triangle = 0i64,
        Square = 1i64,
        Sine = 2i64,
        Noise = 3i64,
    }
    #[derive(Clone, Debug)]
    pub struct SignalPairSet([r#ref::SignalPair; 4u32]);
    /// Enumeration type for use later
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum Choice {
        One = 0i64,
        Two = 1i64,
        Red = 2i64,
        Blue = 3i64,
    }
    /// Enumeration array
    #[derive(Clone, Debug)]
    pub struct ManyChoices([r#ref::Choice; 2u32]);
    /// Packet receive status
    #[derive(Clone, Debug)]
    #[repr(i32)]
    pub enum PacketRecvStatus {
        PacketStateNoPackets = 0i64,
        PacketStateOk = 1i64,
        /// Receiver has seen errors
        PacketStateErrors = 3i64,
    }
    /// Structure of enums (with an multi-dimensional array and structure)
    #[derive(Clone, Debug)]
    pub struct ChoiceSlurry {
        /// A large set of disorganized choices
        pub too_many_choices: r#ref::TooManyChoices,
        /// A singular choice
        pub separate_choice: r#ref::Choice,
        /// A pair of choices
        pub choice_pair: r#ref::ChoicePair,
        /// An array of choices defined as member array
        pub choice_as_member_array: [u8; 2u32],
    }
    /// Set of floating points to emit
    #[derive(Clone, Debug)]
    pub struct FloatSet([f32; 3u32]);
    /// Structure of enums
    #[derive(Clone, Debug)]
    pub struct ChoicePair {
        /// The first choice to make
        pub first_choice: r#ref::Choice,
        /// The second choice to make
        pub second_choice: r#ref::Choice,
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
    #[derive(Clone, Debug)]
    pub struct SignalSet([f32; 4u32]);
    #[derive(Clone, Debug)]
    pub struct SignalInfo {
        pub r#type: r#ref::SignalType,
        pub history: r#ref::SignalSet,
        pub pair_history: r#ref::SignalPairSet,
    }
    mod signal_gen {
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum DpReqType {
            Immediate = 0i64,
            Async = 1i64,
        }
    }
    mod send_buff {
        /// Active state
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum ActiveState {
            SendIdle = 0i64,
            SendActive = 1i64,
        }
    }
    mod dp_demo {
        pub type BoolAlias = bool;
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum ColorEnum {
            Red = 0i64,
            Green = 1i64,
            Blue = 2i64,
        }
        #[derive(Clone, Debug)]
        #[repr(i32)]
        pub enum DpReqType {
            Immediate = 0i64,
            Async = 1i64,
        }
        #[derive(Clone, Debug)]
        pub struct StructWithEverything {
            pub integer_member: r#ref::dp_demo::I32Alias,
            pub float_member: f32,
            pub string_member: heapless::String<80u32>,
            pub boolean_member: bool,
            pub enum_member: r#ref::dp_demo::ColorEnum,
            pub array_member_u_32: [r#ref::dp_demo::U32Array; 2u32],
            pub f_32_array: r#ref::dp_demo::F32Array,
            pub u_32_array: r#ref::dp_demo::U32Array,
            pub enum_array: r#ref::dp_demo::EnumArray,
            pub string_array: r#ref::dp_demo::StringArray,
            pub boolean_array: r#ref::dp_demo::BooleanArray,
            pub struct_with_strings: r#ref::dp_demo::StructWithStringMembers,
            pub nested_arrays: r#ref::dp_demo::ArrayOfStringArray,
        }
        /// Array of integers
        #[derive(Clone, Debug)]
        pub struct U32Array([u32; 5u32]);
        pub type F64Alias = f64;
        #[derive(Clone, Debug)]
        pub struct ArrayOfStructs([r#ref::dp_demo::StructWithStringMembers; 3u32]);
        #[derive(Clone, Debug)]
        pub struct ColorInfoStruct {
            pub color: r#ref::dp_demo::ColorEnum,
        }
        /// Array of strings
        #[derive(Clone, Debug)]
        pub struct StringArray([heapless::String<80u32>; 2u32]);
        pub type StringAlias = heapless::String<80u32>;
        pub type I32Alias = i32;
        #[derive(Clone, Debug)]
        pub struct StructWithStringMembers {
            pub string_member: heapless::String<80u32>,
            pub string_array_member: r#ref::dp_demo::StringArray,
        }
        /// Array of floats
        #[derive(Clone, Debug)]
        pub struct F32Array([f32; 3u32]);
        /// Array of enumerations
        #[derive(Clone, Debug)]
        pub struct EnumArray([r#ref::dp_demo::ColorEnum; 3u32]);
        /// Array of booleans
        #[derive(Clone, Debug)]
        pub struct BooleanArray([bool; 2u32]);
        /// Array of array of strings
        #[derive(Clone, Debug)]
        pub struct ArrayOfStringArray([r#ref::dp_demo::StringArray; 3u32]);
    }
}
