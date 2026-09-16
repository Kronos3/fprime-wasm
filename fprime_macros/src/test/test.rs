use crate::{command, parameter, telemetry};
use pretty_assertions::assert_eq;
use proc_macro2::TokenStream;
use quote::quote;

/// Compare an expansion against the code it is expected to produce.
fn assert_expands(actual: syn::Result<TokenStream>, expected: TokenStream) {
    let actual = actual.expect("expansion failed");
    assert_eq!(expected.to_string(), actual.to_string())
}

/// The rendered message of an expansion that is expected to fail.
fn error(actual: syn::Result<TokenStream>) -> String {
    actual.expect_err("expansion succeeded").to_string()
}

#[test]
fn command_without_arguments() {
    assert_expands(
        command::command(
            quote! { opcode = 0x1000000 },
            quote! {
                /// No-op command
                pub fn CMD_NO_OP(&self) -> super::Defs::Fw::CmdResponse {}
            },
        ),
        quote! {
            /// No-op command
            pub fn CMD_NO_OP(&self) -> super::Defs::Fw::CmdResponse {
                let __encoded = unsafe {
                    let ptr = (&raw mut __SCRATCH) as *mut u8;
                    let len = __SCRATCH_SIZE;

                    core::slice::from_raw_parts_mut(ptr, len)
                };

                let mut __offset: usize = 0;
                let __opcode: FwOpcodeType = 0x1000000;
                __opcode.serialize_to(__encoded, &mut __offset);

                let __response = unsafe { command(&__encoded[0..__offset]) };
                unsafe {
                    <super::Defs::Fw::CmdResponse as ReprEnum>::from_raw_unchecked(__response)
                }
            }
        },
    )
}

#[test]
fn command_serializes_arguments_in_order() {
    assert_expands(
        command::command(
            quote! { opcode = 0x1000002 },
            quote! {
                pub fn CMD_TEST_CMD_1(
                    &self,
                    arg1: i32,
                    arg2: crate::Defs::Ref::DpDemo::DpReqType,
                ) -> super::Defs::Fw::CmdResponse {}
            },
        ),
        quote! {
            pub fn CMD_TEST_CMD_1(
                &self,
                arg1: i32,
                arg2: crate::Defs::Ref::DpDemo::DpReqType,
            ) -> super::Defs::Fw::CmdResponse {
                let __encoded = unsafe {
                    let ptr = (&raw mut __SCRATCH) as *mut u8;
                    let len = __SCRATCH_SIZE;

                    core::slice::from_raw_parts_mut(ptr, len)
                };

                let mut __offset: usize = 0;
                let __opcode: FwOpcodeType = 0x1000002;
                __opcode.serialize_to(__encoded, &mut __offset);
                arg1.serialize_to(__encoded, &mut __offset);
                arg2.serialize_to(__encoded, &mut __offset);

                let __response = unsafe { command(&__encoded[0..__offset]) };
                unsafe {
                    <super::Defs::Fw::CmdResponse as ReprEnum>::from_raw_unchecked(__response)
                }
            }
        },
    )
}

/// A `String<N>` argument is the wire type, callers pass a truncated `&str`
#[test]
fn command_truncates_string_arguments() {
    assert_expands(
        command::command(
            quote! { opcode = 0x1000001 },
            quote! {
                pub fn CMD_NO_OP_STRING(&self, arg1: String<40>) -> super::Defs::Fw::CmdResponse {}
            },
        ),
        quote! {
            pub fn CMD_NO_OP_STRING(&self, arg1: &str) -> super::Defs::Fw::CmdResponse {
                let __encoded = unsafe {
                    let ptr = (&raw mut __SCRATCH) as *mut u8;
                    let len = __SCRATCH_SIZE;

                    core::slice::from_raw_parts_mut(ptr, len)
                };

                let mut __offset: usize = 0;
                let __opcode: FwOpcodeType = 0x1000001;
                __opcode.serialize_to(__encoded, &mut __offset);
                <String<40> as StrTruncate<40>>::truncate(arg1)
                    .serialize_to(__encoded, &mut __offset);

                let __response = unsafe { command(&__encoded[0..__offset]) };
                unsafe {
                    <super::Defs::Fw::CmdResponse as ReprEnum>::from_raw_unchecked(__response)
                }
            }
        },
    )
}

/// A modeled type named `String` keeps its `crate::Defs::` path, and so is
/// serialized by value rather than mistaken for a `fprime_core::String<N>`
#[test]
fn command_passes_modeled_string_types_by_value() {
    assert_expands(
        command::command(
            quote! { opcode = 0x1 },
            quote! {
                pub fn CMD(&self, arg1: crate::Defs::Fw::String) -> super::Defs::Fw::CmdResponse {}
            },
        ),
        quote! {
            pub fn CMD(&self, arg1: crate::Defs::Fw::String) -> super::Defs::Fw::CmdResponse {
                let __encoded = unsafe {
                    let ptr = (&raw mut __SCRATCH) as *mut u8;
                    let len = __SCRATCH_SIZE;

                    core::slice::from_raw_parts_mut(ptr, len)
                };

                let mut __offset: usize = 0;
                let __opcode: FwOpcodeType = 0x1;
                __opcode.serialize_to(__encoded, &mut __offset);
                arg1.serialize_to(__encoded, &mut __offset);

                let __response = unsafe { command(&__encoded[0..__offset]) };
                unsafe {
                    <super::Defs::Fw::CmdResponse as ReprEnum>::from_raw_unchecked(__response)
                }
            }
        },
    )
}

#[test]
fn telemetry_channel() {
    assert_expands(
        telemetry::telemetry(
            quote! { id = 0x1000000 },
            quote! {
                /// Number of commands dispatched
                pub fn CommandsDispatched(&self) -> (u32, super::Defs::Fw::TimeValue) {}
            },
        ),
        quote! {
            /// Number of commands dispatched
            pub fn CommandsDispatched(&self) -> (u32, super::Defs::Fw::TimeValue) {
                let mut __time: [u8; <super::Defs::Fw::TimeValue as Serializable>::SIZE] = unsafe {
                    #[allow(invalid_value)]
                    core::mem::MaybeUninit::uninit().assume_init()
                };

                let mut __value: [u8; <u32 as Serializable>::SIZE] = unsafe {
                    #[allow(invalid_value)]
                    core::mem::MaybeUninit::uninit().assume_init()
                };

                unsafe {
                    telemetry(0x1000000, &mut __time, &mut __value)
                };

                (
                    <u32 as Serializable>::deserialize(&__value),
                    <super::Defs::Fw::TimeValue as Serializable>::deserialize(&__time)
                )
            }
        },
    )
}

#[test]
fn parameter() {
    assert_expands(
        parameter::parameter(
            quote! { id = 0x10022000 },
            quote! {
                /// A choice
                pub fn CHOICE_PRM(&self) -> crate::Defs::Ref::Choice {}
            },
        ),
        quote! {
            /// A choice
            pub fn CHOICE_PRM(&self) -> crate::Defs::Ref::Choice {
                let mut __value: [u8; <crate::Defs::Ref::Choice as Serializable>::SIZE] = unsafe {
                    #[allow(invalid_value)]
                    core::mem::MaybeUninit::uninit().assume_init()
                };

                unsafe {
                    parameter(0x10022000, &mut __value)
                };

                <crate::Defs::Ref::Choice as Serializable>::deserialize(&__value)
            }
        },
    )
}

#[test]
fn rejects_a_declared_body() {
    assert_eq!(
        "expected an empty body, the implementation is generated from the signature",
        error(parameter::parameter(
            quote! { id = 0x1 },
            quote! { pub fn PRM(&self) -> u32 { 0 } },
        ))
    )
}

#[test]
fn rejects_a_missing_return_type() {
    assert_eq!(
        "expected a return type naming the modeled type",
        error(parameter::parameter(
            quote! { id = 0x1 },
            quote! { pub fn PRM(&self) {} },
        ))
    )
}

#[test]
fn rejects_a_channel_without_a_time() {
    assert_eq!(
        "expected a `(value, time)` return type",
        error(telemetry::telemetry(
            quote! { id = 0x1 },
            quote! { pub fn CHANNEL(&self) -> u32 {} },
        ))
    )
}

#[test]
fn rejects_a_missing_identifier() {
    assert_eq!(
        "expected `opcode = <value>`",
        error(command::command(
            quote! {},
            quote! { pub fn CMD(&self) -> super::Defs::Fw::CmdResponse {} },
        ))
    );

    assert_eq!(
        "expected `id`, found `opcode`",
        error(parameter::parameter(
            quote! { opcode = 0x1 },
            quote! { pub fn PRM(&self) -> u32 {} },
        ))
    )
}
