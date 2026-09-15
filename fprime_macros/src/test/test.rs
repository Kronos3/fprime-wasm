use crate::infer_path::test_support::{render_block, rewrite_block};
use pretty_assertions::assert_eq;

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
