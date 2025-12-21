#[allow(unused_imports)]
use fprime_core::*;
/// The type of smaller indices internal to the software, used
/// for array indices, e.g., port indices. Must be signed.
pub type FwIndexType = crate::PlatformIndexType;
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
/// The type of smaller indices internal to the software, used
/// for array indices, e.g., port indices. Must be signed.
pub type PlatformIndexType = i16;
/// The type of a telemetry channel identifier
pub type FwChanIdType = crate::FwIdType;
/// The type used to serialize a time context value
pub type FwTimeContextStoreType = u8;
/// The type of a data product identifier
pub type FwDpIdType = crate::FwIdType;
/// The id type.
pub type FwIdType = u32;
/// The type of a telemetry packet identifier
pub type FwTlmPacketizeIdType = u16;
/// The type of a command opcode
pub type FwOpcodeType = crate::FwIdType;
/// The unsigned type of larger sizes internal to the software,
/// e.g., memory buffer sizes, file sizes. Must be unsigned.
pub type FwSizeType = crate::PlatformSizeType;
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
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// No-op string command
        ///
        ///  * `arg1` - The String command argument
        pub fn cmd_no_op_string(arg_1: &str) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<40> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<40> as StrTruncate<40>>::truncate(arg_1)
                .serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// No-op command
        ///
        ///  * `arg1` - The I32 command argument
        ///  * `arg2` - The F32 command argument
        ///  * `arg3` - The U8 command argument
        pub fn cmd_test_cmd_1(
            arg_1: i32,
            arg_2: f32,
            arg_3: u8,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Clear command tracking info to recover from components not returning status
        pub fn cmd_clear_tracking() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
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
    pub mod events {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Set filter for reporting events. Events are not stored in component.
        ///
        ///  * `filterLevel` - Filter level
        ///  * `filterEnabled` - Filter state
        pub fn set_event_filter(
            filter_level: crate::svc::event_manager::FilterSeverity,
            filter_enabled: crate::svc::event_manager::Enabled,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Filter a particular ID
        ///
        ///  * `ID`
        ///  * `idFilterEnabled` - ID filter state
        pub fn set_id_filter(
            id: crate::FwEventIdType,
            id_filter_enabled: crate::svc::event_manager::Enabled,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Dump the filter states via events
        pub fn dump_filter_state() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
    }
    pub mod health {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// A command to enable or disable health checks
        ///
        ///  * `enable` - whether or not health checks are enabled
        pub fn hlth_enable(enable: crate::fw::Enabled) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::fw::Enabled::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1002000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            enable.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Ignore a particular ping entry
        ///
        ///  * `entry` - The entry to enable/disable
        ///  * `enable` - whether or not a port is pinged
        pub fn hlth_ping_enable(
            entry: &str,
            enable: crate::fw::Enabled,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<40> as Serializable>::SIZE + crate::fw::Enabled::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1002001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<40> as StrTruncate<40>>::truncate(entry)
                .serialize_to(&mut __encoded, &mut __offset);
            enable.serialize_to(&mut __encoded, &mut __offset);
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
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<40> as Serializable>::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1002002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<40> as StrTruncate<40>>::truncate(entry)
                .serialize_to(&mut __encoded, &mut __offset);
            warning_value.serialize_to(&mut __encoded, &mut __offset);
            fatal_value.serialize_to(&mut __encoded, &mut __offset);
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
    pub mod version {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// A command to enable or disable Event verbosity and Telemetry
        ///
        ///  * `enable` - whether or not Version telemetry is enabled
        pub fn enable(enable: crate::svc::VersionEnabled) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::svc::VersionEnabled::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1003000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            enable.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Report version as Event
        ///
        ///  * `version_type` - which version type Event is requested
        pub fn version(version_type: crate::svc::VersionType) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::svc::VersionType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1003001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            version_type.serialize_to(&mut __encoded, &mut __offset);
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
            (crate::svc::CustomVersionDb, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x1003002, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::svc::CustomVersionDb as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        pub fn custom_version_02() -> FprimeResult<
            (crate::svc::CustomVersionDb, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x1003003, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::svc::CustomVersionDb as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        pub fn custom_version_03() -> FprimeResult<
            (crate::svc::CustomVersionDb, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x1003004, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::svc::CustomVersionDb as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        pub fn custom_version_04() -> FprimeResult<
            (crate::svc::CustomVersionDb, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x1003005, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::svc::CustomVersionDb as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        pub fn custom_version_05() -> FprimeResult<
            (crate::svc::CustomVersionDb, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x1003006, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::svc::CustomVersionDb as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        pub fn custom_version_06() -> FprimeResult<
            (crate::svc::CustomVersionDb, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x1003007, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::svc::CustomVersionDb as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        pub fn custom_version_07() -> FprimeResult<
            (crate::svc::CustomVersionDb, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x1003008, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::svc::CustomVersionDb as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        pub fn custom_version_08() -> FprimeResult<
            (crate::svc::CustomVersionDb, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x1003009, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::svc::CustomVersionDb as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        pub fn custom_version_09() -> FprimeResult<
            (crate::svc::CustomVersionDb, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x100300A, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::svc::CustomVersionDb as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        pub fn custom_version_10() -> FprimeResult<
            (crate::svc::CustomVersionDb, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::svc::CustomVersionDb as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x100300B, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::svc::CustomVersionDb as Serializable>::deserialize(value_buf),
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
}
pub mod com_ccsds {
    #[allow(unused_imports)]
    use fprime_core::*;
    pub mod com_queue {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Flush a specific queue. This will discard all queued data in the specified queue removing it from eventual
        /// downlink. Buffers requiring ownership return will be returned via the bufferReturnOut port.
        ///
        ///  * `queueType` - The Queue data type
        ///  * `indexType` - The index of the queue (within the supplied type) to flush
        pub fn flush_queue(
            queue_type: crate::svc::QueueType,
            index_type: crate::FwIndexType,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::svc::QueueType::SIZE + crate::FwIndexType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x2000000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            queue_type.serialize_to(&mut __encoded, &mut __offset);
            index_type.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Flush all queues. This will discard all queued data removing it from eventual downlink. Buffers requiring
        /// ownership return will be returned via the bufferReturnOut port.
        pub fn flush_all_queues() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x2000001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Depth of queues of Fw::ComBuffer type
        pub fn com_queue_depth() -> FprimeResult<
            (crate::svc::ComQueueDepth, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::svc::ComQueueDepth as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x2000000, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::svc::ComQueueDepth as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Depth of queues of Fw::Buffer type
        pub fn buff_queue_depth() -> FprimeResult<
            (crate::svc::BuffQueueDepth, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::svc::BuffQueueDepth as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x2000001, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::svc::BuffQueueDepth as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
    }
    pub mod comms_buffer_manager {
        #[allow(unused_imports)]
        use fprime_core::*;
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
}
pub mod com_cfg {
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
}
pub mod data_products {
    #[allow(unused_imports)]
    use fprime_core::*;
    pub mod dp_buffer_manager {
        #[allow(unused_imports)]
        use fprime_core::*;
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
    pub mod dp_cat {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Build catalog from data product directory. Will block until complete
        pub fn build_catalog() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4000000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Start transmitting catalog
        ///
        ///  * `wait` - have START_XMIT command complete wait for catalog to complete transmitting
        pub fn start_xmit_catalog(wait: crate::fw::Wait) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + crate::fw::Wait::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4000001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            wait.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Stop transmitting catalog
        pub fn stop_xmit_catalog() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4000002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// clear existing catalog
        pub fn clear_catalog() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4000003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
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
    pub mod dp_mgr {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Clear event throttling
        pub fn clear_event_throttle() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4001000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// The number of successful buffer allocations
        pub fn num_successful_allocations() -> FprimeResult<
            (u32, crate::fw::TimeValue),
        > {
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
    pub mod dp_writer {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Clear event throttling
        pub fn clear_event_throttle() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x4002000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
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
}
pub mod file_handling {
    #[allow(unused_imports)]
    use fprime_core::*;
    pub mod file_downlink {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Read a named file off the disk. Divide it into packets and send the packets for transmission to the ground.
        ///
        ///  * `sourceFileName` - The name of the on-board file to send
        ///  * `destFileName` - The name of the destination file on the ground
        pub fn send_file(
            source_file_name: &str,
            dest_file_name: &str,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<100> as Serializable>::SIZE
                + <String<100> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5001000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<100> as StrTruncate<100>>::truncate(source_file_name)
                .serialize_to(&mut __encoded, &mut __offset);
            <String<100> as StrTruncate<100>>::truncate(dest_file_name)
                .serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Cancel the downlink in progress, if any
        pub fn cancel() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5001001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
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
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<100> as Serializable>::SIZE
                + <String<100> as Serializable>::SIZE + u32::SIZE + u32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5001002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<100> as StrTruncate<100>>::truncate(source_file_name)
                .serialize_to(&mut __encoded, &mut __offset);
            <String<100> as StrTruncate<100>>::truncate(dest_file_name)
                .serialize_to(&mut __encoded, &mut __offset);
            start_offset.serialize_to(&mut __encoded, &mut __offset);
            length.serialize_to(&mut __encoded, &mut __offset);
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
    pub mod file_manager {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Create a directory
        ///
        ///  * `dirName` - The directory to create
        pub fn create_directory(dir_name: &str) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<200> as StrTruncate<200>>::truncate(dir_name)
                .serialize_to(&mut __encoded, &mut __offset);
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
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<200> as StrTruncate<200>>::truncate(source_file_name)
                .serialize_to(&mut __encoded, &mut __offset);
            <String<200> as StrTruncate<200>>::truncate(dest_file_name)
                .serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Remove a directory, which must be empty
        ///
        ///  * `dirName` - The directory to remove
        pub fn remove_directory(dir_name: &str) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<200> as StrTruncate<200>>::truncate(dir_name)
                .serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Remove a file
        ///
        ///  * `fileName` - The file to remove
        ///  * `ignoreErrors` - Ignore nonexistent files
        pub fn remove_file(
            file_name: &str,
            ignore_errors: bool,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE + bool::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<200> as StrTruncate<200>>::truncate(file_name)
                .serialize_to(&mut __encoded, &mut __offset);
            ignore_errors.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Perform a Linux shell command and write the output to a log file.
        ///
        ///  * `command` - The shell command string
        ///  * `logFileName` - The name of the log file
        pub fn shell_command(
            command: &str,
            log_file_name: &str,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<256> as Serializable>::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002004;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<256> as StrTruncate<256>>::truncate(command)
                .serialize_to(&mut __encoded, &mut __offset);
            <String<200> as StrTruncate<200>>::truncate(log_file_name)
                .serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Append 1 file's contents to the end of another.
        ///
        ///  * `source` - The name of the file to take content from
        ///  * `target` - The name of the file to append to
        pub fn append_file(source: &str, target: &str) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002005;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<200> as StrTruncate<200>>::truncate(source)
                .serialize_to(&mut __encoded, &mut __offset);
            <String<200> as StrTruncate<200>>::truncate(target)
                .serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        ///
        ///
        ///  * `fileName` - The file to get the size of
        pub fn file_size(file_name: &str) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002006;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<200> as StrTruncate<200>>::truncate(file_name)
                .serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// List the contents of a directory
        ///
        ///  * `dirName` - The directory to list
        pub fn list_directory(dir_name: &str) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5002007;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<200> as StrTruncate<200>>::truncate(dir_name)
                .serialize_to(&mut __encoded, &mut __offset);
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
    pub mod file_uplink {
        #[allow(unused_imports)]
        use fprime_core::*;
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
    pub mod prm_db {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Command to save parameter image to file. Uses file name passed to constructor
        pub fn prm_save_file() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5003000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Loads a file from storage into the staging database. The file could have selective IDs and not the whole set.
        ///
        ///  * `fileName` - The name of the on-board file to set parameters from
        ///  * `merge` - Whether to merge or fully reset the parameter database from the file contents
        pub fn prm_load_file(
            file_name: &str,
            merge: crate::svc::prm_db::Merge,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE
                + crate::svc::prm_db::Merge::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5003001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<200> as StrTruncate<200>>::truncate(file_name)
                .serialize_to(&mut __encoded, &mut __offset);
            merge.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Commits the backup database to become the prime (active) database
        pub fn prm_commit_staged() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x5003002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
    }
}
pub mod fw {
    #[allow(unused_imports)]
    use fprime_core::*;
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum TlmValid {
        Valid = 0,
        Invalid = 1,
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
    /// Wait or don't wait for something
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum Wait {
        /// Wait for something
        Wait = 0,
        /// Don't wait for something
        NoWait = 1,
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
    pub mod dp_cfg {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// A bit mask for selecting the type of processing to perform on
        /// a container before writing it to disk.
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
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
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(i32)]
    pub enum SignalType {
        Triangle = 0,
        Square = 1,
        Sine = 2,
        Noise = 3,
    }
    pub type SignalPairSet = [crate::r#ref::SignalPair; 4];
    /// Enumeration type for use later
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(i32)]
    pub enum Choice {
        One = 0,
        Two = 1,
        Red = 2,
        Blue = 3,
    }
    #[derive(Clone, Debug, Serializable)]
    pub struct SignalPair {
        pub time: f32,
        pub value: f32,
    }
    /// Enumeration array
    pub type ManyChoices = [crate::r#ref::Choice; 2];
    /// Packet receive status
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
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
    /// Array of array
    pub type TooManyChoices = [crate::r#ref::ManyChoices; 2];
    #[derive(Clone, Debug, Serializable)]
    pub struct SignalInfo {
        pub r#type: crate::r#ref::SignalType,
        pub history: crate::r#ref::SignalSet,
        pub pair_history: crate::r#ref::SignalPairSet,
    }
    pub mod block_drv {
        #[allow(unused_imports)]
        use fprime_core::*;
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
    pub mod cmd_seq {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Loads, validates and runs a sequence
        ///
        ///  * `fileName` - The name of the sequence file
        ///  * `block` - Return command status when complete or not
        pub fn run(
            file_name: &str,
            block: crate::svc::wasm_sequencer::BlockState,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE
                + crate::svc::wasm_sequencer::BlockState::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<200> as StrTruncate<200>>::truncate(file_name)
                .serialize_to(&mut __encoded, &mut __offset);
            block.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Loads and validates a sequence
        ///
        ///  * `fileName` - The name of the sequence file
        pub fn validate(file_name: &str) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + <String<200> as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            <String<200> as StrTruncate<200>>::truncate(file_name)
                .serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Must be called after VALIDATE. Runs the sequence that was validated.
        ///
        ///  * `block` - Return command status when complete or not
        pub fn run_validated(
            block: crate::svc::wasm_sequencer::BlockState,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::svc::wasm_sequencer::BlockState::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            block.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Cancels a running or validated sequence. After running CANCEL, the sequencer
        /// should return to IDLE
        pub fn cancel() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10006003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
    }
    pub mod dp_demo {
        #[allow(unused_imports)]
        use fprime_core::*;
        pub type BoolAlias = bool;
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(i32)]
        pub enum ColorEnum {
            Red = 0,
            Green = 1,
            Blue = 2,
        }
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
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
        ///
        ///  * `color`
        pub fn select_color(
            color: crate::r#ref::dp_demo::ColorEnum,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::dp_demo::ColorEnum::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0xA10;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            color.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Command for generating a DP
        ///
        ///  * `reqType`
        ///  * `priority`
        pub fn dp(
            req_type: crate::r#ref::dp_demo::DpReqType,
            priority: u32,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::dp_demo::DpReqType::SIZE + u32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0xA11;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            req_type.serialize_to(&mut __encoded, &mut __offset);
            priority.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
    }
    pub mod ping_rcvr {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Command to disable ping response
        pub fn pr_stop_pings() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10004000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
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
    pub mod rate_group_1_comp {
        #[allow(unused_imports)]
        use fprime_core::*;
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
    pub mod rate_group_2_comp {
        #[allow(unused_imports)]
        use fprime_core::*;
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
    pub mod rate_group_3_comp {
        #[allow(unused_imports)]
        use fprime_core::*;
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
    pub mod recv_buff_comp {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// A test parameter
        ///
        ///  * `val`
        pub fn parameter_1_prm_set(val: u32) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10022000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// A test parameter
        pub fn parameter_1_prm_save() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10022001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// A test parameter
        ///
        ///  * `val`
        pub fn parameter_2_prm_set(val: i16) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + i16::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10022002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// A test parameter
        pub fn parameter_2_prm_save() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10022003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Packet Statistics
        pub fn pkt_state() -> FprimeResult<
            (crate::r#ref::PacketStat, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::PacketStat as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10022000, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::PacketStat as Serializable>::deserialize(value_buf),
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
    pub mod send_buff {
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
    pub mod send_buff_comp {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Command to start sending packets
        pub fn sb_start_pkts() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10010000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Send a bad packet
        pub fn sb_inject_pkt_error() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10010001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Generate a FATAL EVR
        ///
        ///  * `arg1` - First FATAL Argument
        ///  * `arg2` - Second FATAL Argument
        ///  * `arg3` - Third FATAL Argument
        pub fn sb_gen_fatal(
            arg_1: u32,
            arg_2: u32,
            arg_3: u32,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// A test parameter
        ///
        ///  * `val`
        pub fn parameter_3_prm_set(val: u8) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + u8::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001000A;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// A test parameter
        pub fn parameter_3_prm_save() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001000B;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// A test parameter
        ///
        ///  * `val`
        pub fn parameter_4_prm_set(val: f32) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE + f32::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001000C;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// A test parameter
        pub fn parameter_4_prm_save() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1001000D;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
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
            (crate::r#ref::send_buff::ActiveState, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::send_buff::ActiveState as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10010004, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::send_buff::ActiveState as Serializable>::deserialize(
                    value_buf,
                ),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
    }
    pub mod sg_1 {
        #[allow(unused_imports)]
        use fprime_core::*;
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
            sig_type: crate::r#ref::SignalType,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Toggle Signal Generator On/Off.
        pub fn toggle() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10011001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Skip next sample
        pub fn skip() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10011002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Signal Generator Settings
        ///
        ///  * `reqType`
        ///  * `records`
        ///  * `priority`
        pub fn dp(
            req_type: crate::r#ref::signal_gen::DpReqType,
            records: u32,
            priority: u32,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Type of the output signal: SINE, TRIANGLE, etc.
        pub fn r#type() -> FprimeResult<
            (crate::r#ref::SignalType, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalType as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10011000, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalType as Serializable>::deserialize(value_buf),
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
            (crate::r#ref::SignalPair, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalPair as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10011002, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalPair as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Last 10 Y values of the signal
        pub fn history() -> FprimeResult<
            (crate::r#ref::SignalSet, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalSet as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10011003, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalSet as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Last 10 (time, value) pairs of the signal
        pub fn pair_history() -> FprimeResult<
            (crate::r#ref::SignalPairSet, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalPairSet as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10011004, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalPairSet as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Composite field of signal information, containing histories, pairs etc
        pub fn info() -> FprimeResult<(crate::r#ref::SignalInfo, crate::fw::TimeValue)> {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalInfo as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10011005, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalInfo as Serializable>::deserialize(value_buf),
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
    pub mod sg_2 {
        #[allow(unused_imports)]
        use fprime_core::*;
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
            sig_type: crate::r#ref::SignalType,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Toggle Signal Generator On/Off.
        pub fn toggle() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10012001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Skip next sample
        pub fn skip() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10012002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Signal Generator Settings
        ///
        ///  * `reqType`
        ///  * `records`
        ///  * `priority`
        pub fn dp(
            req_type: crate::r#ref::signal_gen::DpReqType,
            records: u32,
            priority: u32,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Type of the output signal: SINE, TRIANGLE, etc.
        pub fn r#type() -> FprimeResult<
            (crate::r#ref::SignalType, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalType as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10012000, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalType as Serializable>::deserialize(value_buf),
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
            (crate::r#ref::SignalPair, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalPair as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10012002, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalPair as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Last 10 Y values of the signal
        pub fn history() -> FprimeResult<
            (crate::r#ref::SignalSet, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalSet as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10012003, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalSet as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Last 10 (time, value) pairs of the signal
        pub fn pair_history() -> FprimeResult<
            (crate::r#ref::SignalPairSet, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalPairSet as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10012004, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalPairSet as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Composite field of signal information, containing histories, pairs etc
        pub fn info() -> FprimeResult<(crate::r#ref::SignalInfo, crate::fw::TimeValue)> {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalInfo as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10012005, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalInfo as Serializable>::deserialize(value_buf),
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
    pub mod sg_3 {
        #[allow(unused_imports)]
        use fprime_core::*;
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
            sig_type: crate::r#ref::SignalType,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Toggle Signal Generator On/Off.
        pub fn toggle() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10013001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Skip next sample
        pub fn skip() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10013002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Signal Generator Settings
        ///
        ///  * `reqType`
        ///  * `records`
        ///  * `priority`
        pub fn dp(
            req_type: crate::r#ref::signal_gen::DpReqType,
            records: u32,
            priority: u32,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Type of the output signal: SINE, TRIANGLE, etc.
        pub fn r#type() -> FprimeResult<
            (crate::r#ref::SignalType, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalType as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10013000, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalType as Serializable>::deserialize(value_buf),
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
            (crate::r#ref::SignalPair, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalPair as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10013002, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalPair as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Last 10 Y values of the signal
        pub fn history() -> FprimeResult<
            (crate::r#ref::SignalSet, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalSet as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10013003, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalSet as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Last 10 (time, value) pairs of the signal
        pub fn pair_history() -> FprimeResult<
            (crate::r#ref::SignalPairSet, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalPairSet as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10013004, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalPairSet as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Composite field of signal information, containing histories, pairs etc
        pub fn info() -> FprimeResult<(crate::r#ref::SignalInfo, crate::fw::TimeValue)> {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalInfo as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10013005, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalInfo as Serializable>::deserialize(value_buf),
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
    pub mod sg_4 {
        #[allow(unused_imports)]
        use fprime_core::*;
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
            sig_type: crate::r#ref::SignalType,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Toggle Signal Generator On/Off.
        pub fn toggle() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10014001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Skip next sample
        pub fn skip() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10014002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Signal Generator Settings
        ///
        ///  * `reqType`
        ///  * `records`
        ///  * `priority`
        pub fn dp(
            req_type: crate::r#ref::signal_gen::DpReqType,
            records: u32,
            priority: u32,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Type of the output signal: SINE, TRIANGLE, etc.
        pub fn r#type() -> FprimeResult<
            (crate::r#ref::SignalType, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalType as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10014000, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalType as Serializable>::deserialize(value_buf),
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
            (crate::r#ref::SignalPair, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalPair as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10014002, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalPair as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Last 10 Y values of the signal
        pub fn history() -> FprimeResult<
            (crate::r#ref::SignalSet, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalSet as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10014003, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalSet as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Last 10 (time, value) pairs of the signal
        pub fn pair_history() -> FprimeResult<
            (crate::r#ref::SignalPairSet, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalPairSet as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10014004, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalPairSet as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Composite field of signal information, containing histories, pairs etc
        pub fn info() -> FprimeResult<(crate::r#ref::SignalInfo, crate::fw::TimeValue)> {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalInfo as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10014005, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalInfo as Serializable>::deserialize(value_buf),
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
    pub mod sg_5 {
        #[allow(unused_imports)]
        use fprime_core::*;
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
            sig_type: crate::r#ref::SignalType,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Toggle Signal Generator On/Off.
        pub fn toggle() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10015001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Skip next sample
        pub fn skip() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10015002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Signal Generator Settings
        ///
        ///  * `reqType`
        ///  * `records`
        ///  * `priority`
        pub fn dp(
            req_type: crate::r#ref::signal_gen::DpReqType,
            records: u32,
            priority: u32,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Type of the output signal: SINE, TRIANGLE, etc.
        pub fn r#type() -> FprimeResult<
            (crate::r#ref::SignalType, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalType as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10015000, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalType as Serializable>::deserialize(value_buf),
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
            (crate::r#ref::SignalPair, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalPair as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10015002, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalPair as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Last 10 Y values of the signal
        pub fn history() -> FprimeResult<
            (crate::r#ref::SignalSet, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalSet as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10015003, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalSet as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Last 10 (time, value) pairs of the signal
        pub fn pair_history() -> FprimeResult<
            (crate::r#ref::SignalPairSet, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalPairSet as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10015004, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalPairSet as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Composite field of signal information, containing histories, pairs etc
        pub fn info() -> FprimeResult<(crate::r#ref::SignalInfo, crate::fw::TimeValue)> {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::SignalInfo as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10015005, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::SignalInfo as Serializable>::deserialize(value_buf),
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
    pub mod signal_gen {
        #[allow(unused_imports)]
        use fprime_core::*;
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
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
        ///
        ///  * `enable` - whether or not system resource telemetry is enabled
        pub fn enable(
            enable: crate::svc::SystemResourceEnabled,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::svc::SystemResourceEnabled::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10023000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            enable.serialize_to(&mut __encoded, &mut __offset);
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
    pub mod type_demo {
        #[allow(unused_imports)]
        use fprime_core::*;
        /// Single choice command
        ///
        ///  * `choice` - A single choice
        pub fn choice(choice: crate::r#ref::Choice) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::Choice::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005000;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            choice.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Single enumeration parameter
        ///
        ///  * `val`
        pub fn choice_prm_prm_set(val: crate::r#ref::Choice) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::Choice::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005001;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Single enumeration parameter
        pub fn choice_prm_prm_save() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005002;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Multiple choice command via Array
        ///
        ///  * `choices` - A set of choices
        pub fn choices(choices: crate::r#ref::ManyChoices) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ManyChoices::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005003;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
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
            choices: crate::r#ref::ManyChoices,
            repeat_max: u8,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Multiple enumeration parameter via Array
        ///
        ///  * `val`
        pub fn choices_prm_prm_set(
            val: crate::r#ref::ManyChoices,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ManyChoices::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005005;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Multiple enumeration parameter via Array
        pub fn choices_prm_prm_save() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005006;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Too many choice command via Array
        ///
        ///  * `choices` - Way to many choices to make
        pub fn extra_choices(
            choices: crate::r#ref::TooManyChoices,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::TooManyChoices::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005007;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
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
            choices: crate::r#ref::TooManyChoices,
            repeat_max: u8,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Too many enumeration parameter via Array
        ///
        ///  * `val`
        pub fn extra_choices_prm_prm_set(
            val: crate::r#ref::ManyChoices,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ManyChoices::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005009;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Too many enumeration parameter via Array
        pub fn extra_choices_prm_prm_save() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500A;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Multiple choice command via Structure
        ///
        ///  * `choices` - A pair of choices
        pub fn choice_pair(choices: crate::r#ref::ChoicePair) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ChoicePair::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500B;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
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
            choices: crate::r#ref::ChoicePair,
            repeat_max: u8,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Multiple enumeration parameter via Structure
        ///
        ///  * `val`
        pub fn choice_pair_prm_prm_set(
            val: crate::r#ref::ChoicePair,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ChoicePair::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500D;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Multiple enumeration parameter via Structure
        pub fn choice_pair_prm_prm_save() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500E;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Multiple choice command via Complex Structure
        ///
        ///  * `choices` - A phenomenal amount of choice
        pub fn glutton_of_choice(
            choices: crate::r#ref::ChoiceSlurry,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ChoiceSlurry::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x1000500F;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            choices.serialize_to(&mut __encoded, &mut __offset);
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
            choices: crate::r#ref::ChoiceSlurry,
            repeat_max: u8,
        ) -> crate::fw::CmdResponse {
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
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Multiple enumeration parameter via Complex Structure
        ///
        ///  * `val`
        pub fn glutton_of_choice_prm_prm_set(
            val: crate::r#ref::ChoiceSlurry,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ChoiceSlurry::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005011;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            val.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Multiple enumeration parameter via Complex Structure
        pub fn glutton_of_choice_prm_prm_save() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005012;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Dump the typed parameters
        pub fn dump_typed_parameters() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005013;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Dump the float values
        pub fn dump_floats() -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005014;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Send scalars
        ///
        ///  * `scalar_input`
        pub fn send_scalars(
            scalar_input: crate::r#ref::ScalarStruct,
        ) -> crate::fw::CmdResponse {
            let mut __encoded: [u8; crate::FwOpcodeType::SIZE
                + crate::r#ref::ScalarStruct::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut __offset: usize = 0;
            let __opcode: crate::FwOpcodeType = 0x10005015;
            __opcode.serialize_to(&mut __encoded, &mut __offset);
            scalar_input.serialize_to(&mut __encoded, &mut __offset);
            let res = unsafe { sys::command(&__encoded[0..__offset]) };
            unsafe { core::mem::transmute(res as u8) }
        }
        /// Single choice channel
        pub fn choice_ch() -> FprimeResult<
            (crate::r#ref::Choice, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::Choice as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10005000, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::Choice as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Multiple choice channel via Array
        pub fn choices_ch() -> FprimeResult<
            (crate::r#ref::ManyChoices, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::ManyChoices as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10005001, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::ManyChoices as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Too many choice channel via Array
        pub fn extra_choices_ch() -> FprimeResult<
            (crate::r#ref::TooManyChoices, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::TooManyChoices as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10005002, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::TooManyChoices as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Multiple choice channel via Structure
        pub fn choice_pair_ch() -> FprimeResult<
            (crate::r#ref::ChoicePair, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::ChoicePair as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10005003, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::ChoicePair as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Multiple choice channel via Complex Structure
        pub fn choice_slurry_ch() -> FprimeResult<
            (crate::r#ref::ChoiceSlurry, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::ChoiceSlurry as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10005004, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::ChoiceSlurry as Serializable>::deserialize(value_buf),
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
        pub fn float_set() -> FprimeResult<
            (crate::r#ref::FloatSet, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::FloatSet as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10005008, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::FloatSet as Serializable>::deserialize(value_buf),
                crate::fw::TimeValue::deserialize(time_buf),
            ))
        }
        /// Scalar struct channel
        pub fn scalar_struct_ch() -> FprimeResult<
            (crate::r#ref::ScalarStruct, crate::fw::TimeValue),
        > {
            let mut time_buf: [u8; crate::fw::TimeValue::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            let mut value_buf: [u8; <crate::r#ref::ScalarStruct as Serializable>::SIZE] = unsafe {
                #[allow(invalid_value)] core::mem::MaybeUninit::uninit().assume_init()
            };
            unsafe { sys::telemetry(0x10005009, &mut time_buf, &mut value_buf) }?;
            Ok((
                <crate::r#ref::ScalarStruct as Serializable>::deserialize(value_buf),
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
}
pub mod svc {
    #[allow(unused_imports)]
    use fprime_core::*;
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
    /// Array of queue depths for Fw::Com types
    pub type ComQueueDepth = [u32; 2];
    /// Array of queue depths for Fw::Buffer types
    pub type BuffQueueDepth = [u32; 1];
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
    /// Tracks versions for project, framework and user defined versions etc
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
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
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum SystemResourceEnabled {
        Disabled = 0,
        Enabled = 1,
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
    /// Header validation error
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
    #[repr(u8)]
    pub enum DpHdrField {
        Descriptor = 0,
        Id = 1,
        Priority = 2,
        Crc = 3,
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
    pub mod event_manager {
        #[allow(unused_imports)]
        use fprime_core::*;
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
        /// Enabled and disabled state
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum Enabled {
            /// Enabled state
            Enabled = 0,
            /// Disabled state
            Disabled = 1,
        }
    }
    pub mod fprime_router {
        #[allow(unused_imports)]
        use fprime_core::*;
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum AllocationReason {
            /// Buffer allocation for file uplink
            FileUplink = 0,
            /// Buffer allocation for user handled buffer
            UserBuffer = 1,
        }
    }
    pub mod prm_db {
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
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum Merge {
            Merge = 0,
            Reset = 1,
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
        }
        /// State of parameter DB file load operations
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(u8)]
        pub enum PrmDbFileLoadState {
            Idle = 0,
            LoadingFileUpdates = 1,
            FileUpdatesStaged = 2,
        }
    }
    pub mod version_cfg {
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
        #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serializable)]
        #[repr(i32)]
        pub enum BlockState {
            Block = 0,
            NoBlock = 1,
        }
    }
}
