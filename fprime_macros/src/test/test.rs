use crate::infer_path::test_support::{render_block, rewrite_block};
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

/// Apply the DSL to a sequence body.
fn rewrite(body: &str) -> String {
    rewrite_block(body).expect("failed to rewrite body")
}

/// The body the DSL is expected to produce, written as ordinary Rust.
fn expect(body: &str) -> String {
    render_block(body).expect("failed to parse expected body")
}

#[test]
fn qualifies_by_formal_parameter_type() {
    assert_eq!(
        rewrite("Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);"),
        expect(
            "Ref.dpDemo.Dp(
                 crate::Defs::Ref::DpDemo::DpReqType::IMMEDIATE,
                 0,
                 crate::Defs::Fw::DpCfg::ProcType::PROC_TYPE_NONE
             );"
        )
    )
}

/// `DpReqType` is declared by both `Ref.DpDemo` and `Ref.SignalGen`, so only the
/// receiver decides which enum `IMMEDIATE` belongs to.
#[test]
fn same_constant_resolves_per_receiver() {
    assert_eq!(
        rewrite("Ref.SG1.Dp(IMMEDIATE, 0, 1);"),
        expect("Ref.SG1.Dp(crate::Defs::Ref::SignalGen::DpReqType::IMMEDIATE, 0, 1);")
    )
}

/// `ACTIVITY_HI` is shared by two severity enums and `DISABLED` by four
/// `Enabled` style enums; only the parameter type tells them apart.
#[test]
fn qualifies_ambiguous_constants() {
    assert_eq!(
        rewrite("CdhCore.events.SET_EVENT_FILTER(ACTIVITY_HI, DISABLED);"),
        expect(
            "CdhCore.events.SET_EVENT_FILTER(
                 crate::Defs::Svc::EventManager::FilterSeverity::ACTIVITY_HI,
                 crate::Defs::Svc::EventManager::Enabled::DISABLED
             );"
        )
    )
}

/// `RED` and `BLUE` are declared by both `Ref.Choice` and `Ref.DpDemo.ColorEnum`.
#[test]
fn qualifies_array_elements() {
    assert_eq!(
        rewrite("Ref.typeDemo.CHOICES([RED, BLUE]);"),
        expect("Ref.typeDemo.CHOICES([crate::Defs::Ref::Choice::RED, crate::Defs::Ref::Choice::BLUE]);")
    )
}

#[test]
fn qualifies_struct_literal_and_members() {
    assert_eq!(
        rewrite("Ref.typeDemo.CHOICE_PAIR(ChoicePair { firstChoice: RED, secondChoice: TWO });"),
        expect(
            "Ref.typeDemo.CHOICE_PAIR(crate::Defs::Ref::ChoicePair {
                 firstChoice: crate::Defs::Ref::Choice::RED,
                 secondChoice: crate::Defs::Ref::Choice::TWO
             });"
        )
    )
}

#[test]
fn qualifies_chained_and_nested_commands() {
    assert_eq!(
        rewrite(
            "Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE).check();
             if ready { Ref.dpDemo.SelectColor(BLUE); }"
        ),
        expect(
            "Ref.dpDemo
                 .Dp(
                     crate::Defs::Ref::DpDemo::DpReqType::IMMEDIATE,
                     0,
                     crate::Defs::Fw::DpCfg::ProcType::PROC_TYPE_NONE
                 )
                 .check();
             if ready { Ref.dpDemo.SelectColor(crate::Defs::Ref::DpDemo::ColorEnum::BLUE); }"
        )
    )
}

/// An attribute on the argument belongs to the argument, not to the name, so
/// qualifying the name must not drop it -- a `#[cfg]` gated argument that
/// quietly stopped being gated would change what is sent.
#[test]
fn keeps_attributes_on_the_argument() {
    assert_eq!(
        rewrite("Ref.dpDemo.SelectColor(#[cfg(feature = \"red\")] RED);"),
        expect("Ref.dpDemo.SelectColor(#[cfg(feature = \"red\")] crate::Defs::Ref::DpDemo::ColorEnum::RED);")
    )
}

/// An editor asks about a body by splicing a marker identifier in at the cursor
/// and expanding. It then finds the cursor in the real expansion at the offset the
/// marker sits at in this one, so the two have to agree up to that point: a name
/// carrying the marker must resolve to whatever the name without it resolves to,
/// and must come out still carrying it.
#[test]
fn resolves_a_name_an_editor_is_asking_about() {
    // A finished constant with the cursor in it: qualified, marker preserved, so
    // this differs from the real expansion by the marker alone.
    assert_eq!(
        rewrite("Ref.dpDemo.SelectColor(REraCompletionMarkerD);"),
        expect("Ref.dpDemo.SelectColor(crate::Defs::Ref::DpDemo::ColorEnum::REraCompletionMarkerD);")
    );

    // Half typed, so not a constant: left alone, exactly as the real expansion
    // leaves the `PROC_` the author has actually typed.
    let half_typed = "Ref.dpDemo.Dp(IMMEDIATE, 2, PROC_raCompletionMarker);";
    assert_eq!(
        rewrite(half_typed),
        expect("Ref.dpDemo.Dp(crate::Defs::Ref::DpDemo::DpReqType::IMMEDIATE, 2, PROC_raCompletionMarker);")
    );

    // Nothing typed yet: likewise left alone, since the real expansion has no
    // argument there at all.
    assert_eq!(
        rewrite("Ref.dpDemo.Dp(IMMEDIATE, 2, raCompletionMarker);"),
        expect("Ref.dpDemo.Dp(crate::Defs::Ref::DpDemo::DpReqType::IMMEDIATE, 2, raCompletionMarker);")
    )
}

/// A call with too few arguments keeps the arguments it does have qualified, so
/// the count is the only thing left for the compiler to complain about.
#[test]
fn qualifies_what_it_can_of_a_short_call() {
    assert_eq!(
        rewrite("Ref.dpDemo.Dp(IMMEDIATE, 2);"),
        expect("Ref.dpDemo.Dp(crate::Defs::Ref::DpDemo::DpReqType::IMMEDIATE, 2);")
    )
}

/// Everything the dictionary does not describe survives untouched.
/// The reason the DSL reads tokens instead of a parsed function: a body is
/// half written most of the time it is looked at, and giving up on one that does
/// not parse costs every finished statement in it its resolution. Here a `;` is
/// missing, which is what the compiler hands the macro verbatim.
#[test]
fn qualifies_across_a_syntax_error() {
    assert_eq!(
        rewrite(
            "CdhCore.events.SET_EVENT_FILTER(ACTIVITY_HI, DISABLED)
             Ref.dpDemo.SelectColor(GREEN);"
        ),
        expect(
            "CdhCore.events.SET_EVENT_FILTER(
                 crate::Defs::Svc::EventManager::FilterSeverity::ACTIVITY_HI,
                 crate::Defs::Svc::EventManager::Enabled::DISABLED
             )
             Ref.dpDemo.SelectColor(crate::Defs::Ref::DpDemo::ColorEnum::GREEN);"
        )
    )
}

/// An argument deleted out of the middle of a call leaves the slot it was in
/// empty, rather than shifting every later argument onto the parameter before
/// it -- which is what splitting on the commas as written buys over parsing the
/// argument list.
#[test]
fn keeps_arguments_on_their_own_parameters_across_a_gap() {
    assert_eq!(
        rewrite("Ref.dpDemo.Dp(, 0, PROC_TYPE_NONE);"),
        expect("Ref.dpDemo.Dp(, 0, crate::Defs::Fw::DpCfg::ProcType::PROC_TYPE_NONE);")
    )
}

/// The limitation of that, stated: an argument only resolves if what is written
/// in its slot is an expression. Delete a comma and the two names it separated
/// read as one argument, which is not, so both keep their own spelling -- and
/// the rest of the body is qualified regardless.
#[test]
fn leaves_an_argument_that_is_not_yet_an_expression() {
    assert_eq!(
        rewrite(
            "Ref.dpDemo.Dp(IMMEDIATE 0, PROC_TYPE_NONE);
             Ref.dpDemo.SelectColor(GREEN);"
        ),
        expect(
            "Ref.dpDemo.Dp(IMMEDIATE 0, PROC_TYPE_NONE);
             Ref.dpDemo.SelectColor(crate::Defs::Ref::DpDemo::ColorEnum::GREEN);"
        )
    )
}

/// A command is qualified wherever it sits, including inside an argument of
/// something that is not a command at all.
#[test]
fn qualifies_a_command_nested_in_an_argument() {
    assert_eq!(
        rewrite("record(Ref.dpDemo.SelectColor(BLUE));"),
        expect("record(Ref.dpDemo.SelectColor(crate::Defs::Ref::DpDemo::ColorEnum::BLUE));")
    )
}

/// A chain that merely *ends* in a command's name is not that command. This is
/// the shape a trailing `.` leaves behind: `Ref.` and the statement under it read
/// as one field chain, and qualifying its argument would be resolving against a
/// call nobody wrote.
#[test]
fn does_not_qualify_a_chain_that_only_ends_in_a_command() {
    let absorbed = "Ref.Ref.dpDemo.SelectColor(GREEN);";
    assert_eq!(rewrite(absorbed), expect(absorbed));

    // Nor one that is a command's name with something in front of it.
    let extended = "topology.Ref.dpDemo.SelectColor(GREEN);";
    assert_eq!(rewrite(extended), expect(extended));
}

/// The instance may be written as a qualified path, of which the dictionary knows
/// only the final segment.
#[test]
fn qualifies_through_a_qualified_instance() {
    assert_eq!(
        rewrite("crate::Ref.dpDemo.SelectColor(BLUE);"),
        expect("crate::Ref.dpDemo.SelectColor(crate::Defs::Ref::DpDemo::ColorEnum::BLUE);")
    )
}

/// The DSL is applied to the whole item, so everything that is not a command
/// argument -- attributes, visibility, the signature, a parameter list that
/// looks like a call -- has to come out as it went in.
#[test]
fn leaves_everything_but_the_arguments_alone() {
    assert_eq!(
        rewrite(
            "#[inline]
             pub fn sequence(retries: u32) -> u32 {
                 Ref.dpDemo.SelectColor(BLUE);
                 retries
             }"
        ),
        expect(
            "#[inline]
             pub fn sequence(retries: u32) -> u32 {
                 Ref.dpDemo.SelectColor(crate::Defs::Ref::DpDemo::ColorEnum::BLUE);
                 retries
             }"
        )
    )
}

/// An editor asks about a name by expanding a copy of the body with a marker
/// spliced in, and most of the time it asks the body does not parse. The marker
/// has to survive that path too.
#[test]
fn resolves_a_name_an_editor_is_asking_about_in_a_broken_body() {
    assert_eq!(
        rewrite(
            "Ref.dpDemo.SelectColor(BLraCompletionMarkerUE)
             Ref.dpDemo.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);"
        ),
        expect(
            "Ref.dpDemo.SelectColor(crate::Defs::Ref::DpDemo::ColorEnum::BLraCompletionMarkerUE)
             Ref.dpDemo.Dp(
                 crate::Defs::Ref::DpDemo::DpReqType::IMMEDIATE,
                 0,
                 crate::Defs::Fw::DpCfg::ProcType::PROC_TYPE_NONE
             );"
        )
    )
}

#[test]
fn passes_through_unknown_names() {
    // A constant of the user's own in an enum slot, a local in a numeric slot
    // and an already qualified path
    let untouched = "Ref.dpDemo.Dp(MY_REQ_TYPE, priority, ProcType::PROC_TYPE_NONE);";
    assert_eq!(rewrite(untouched), expect(untouched));

    // An instance the dictionary does not know
    let unknown_command = "Ref.notAThing.Dp(IMMEDIATE, 0, PROC_TYPE_NONE);";
    assert_eq!(rewrite(unknown_command), expect(unknown_command));

    // Outside a command argument there is no expected type to resolve against
    let outside_a_command = "let request = IMMEDIATE;";
    assert_eq!(rewrite(outside_a_command), expect(outside_a_command));

    // Non enum parameters are never rewritten
    let string_argument = "Ref.wasmSeq.LOAD(\"helloworld\");";
    assert_eq!(rewrite(string_argument), expect(string_argument));
}
