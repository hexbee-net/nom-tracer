// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

use {
    nom::{bytes::complete::tag, sequence::tuple, IResult},
    nom_tracer::{
        activate_trace,
        activate_trace_print,
        deactivate_trace,
        deactivate_trace_print,
        get_trace,
        get_trace_for_tag,
        reset_trace,
        trace,
        DEFAULT_TAG,
        NOM_TRACE,
    },
};

fn parse_ab(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((tag("a"), tag("b")))(input)
}

#[test]
fn test_simple_trace() {
    let result = trace!(parse_ab)("ab");
    assert!(result.is_ok());

    let trace = get_trace();
    assert!(trace.contains("test_simple_trace"));
    assert!(trace.contains("-> Ok"));
}

#[test]
fn test_trace_with_tag() {
    let result = trace!(custom_tag, parse_ab)("ab");
    assert!(result.is_ok());

    let trace = get_trace_for_tag("custom_tag");
    assert!(trace.contains("test_trace_with_tag"));
    assert!(trace.contains("-> Ok"));
}

#[test]
fn test_trace_with_context() {
    let result = trace!("custom_context", parse_ab)("ab");
    assert!(result.is_ok());

    let trace = get_trace();
    assert!(trace.contains("test_trace_with_context"));
    assert!(trace.contains("custom_context"));
    assert!(trace.contains("-> Ok"));
}

#[test]
fn test_trace_with_tag_and_context() {
    let result = trace!(custom_tag, "custom_context", parse_ab)("ab");
    assert!(result.is_ok());

    let trace = get_trace_for_tag("custom_tag");
    assert!(trace.contains("test_trace_with_tag"));
    assert!(trace.contains("custom_context"));
    assert!(trace.contains("-> Ok"));
}

#[test]
fn test_activate_trace() {
    deactivate_trace!();
    let result = trace!(parse_ab)("ab");
    assert!(result.is_ok());

    let trace_before = get_trace();
    assert!(
        trace_before.is_empty(),
        "Trace should be empty when deactivated"
    );

    activate_trace!();
    let result = trace!(parse_ab)("ab");
    assert!(result.is_ok());

    let trace_after = get_trace();
    assert!(
        !trace_after.is_empty(),
        "Trace should not be empty after activation"
    );
    assert!(trace_after.contains("test_activate_trace"));
    assert!(trace_after.contains("-> Ok"));
}

#[test]
fn test_activate_trace_with_tag() {
    deactivate_trace!(custom_tag);
    let result = trace!(custom_tag, parse_ab)("ab");
    assert!(result.is_ok());

    let trace_before = get_trace_for_tag("custom_tag");
    assert!(
        trace_before.is_empty(),
        "Trace should be empty when deactivated"
    );

    activate_trace!(custom_tag);
    let result = trace!(custom_tag, parse_ab)("ab");
    assert!(result.is_ok());

    let trace_after = get_trace_for_tag("custom_tag");
    assert!(
        !trace_after.is_empty(),
        "Trace should not be empty after activation"
    );
    assert!(trace_after.contains("test_activate_trace_with_tag"));
    assert!(trace_after.contains("-> Ok"));
}

#[test]
fn test_deactivate_trace() {
    activate_trace!();
    let result = trace!(parse_ab)("ab");
    assert!(result.is_ok());

    let trace_before = get_trace();
    assert!(
        !trace_before.is_empty(),
        "Trace should not be empty when activated"
    );

    deactivate_trace!();
    let result = trace!(parse_ab)("ab");
    assert!(result.is_ok());

    let trace_after = get_trace();
    assert_eq!(
        trace_before, trace_after,
        "Trace should not change after deactivation"
    );
}

#[test]
fn test_deactivate_trace_with_tag() {
    activate_trace!(custom_tag);
    let result = trace!(custom_tag, parse_ab)("ab");
    assert!(result.is_ok());

    let trace_before = get_trace_for_tag("custom_tag");
    assert!(
        !trace_before.is_empty(),
        "Trace should not be empty when activated"
    );

    deactivate_trace!(custom_tag);
    let result = trace!(custom_tag, parse_ab)("ab");
    assert!(result.is_ok());

    let trace_after = get_trace_for_tag("custom_tag");
    assert_eq!(
        trace_before, trace_after,
        "Trace should not change after deactivation"
    );
}

#[test]
#[cfg(feature = "trace-print")]
fn test_activate_trace_print() {
    // Ensure trace print is initially deactivated
    NOM_TRACE.with(|trace| {
        assert!(!trace.borrow().traces[DEFAULT_TAG].print);
    });

    // Activate trace print
    activate_trace_print!();

    // Check if trace print is now activated
    NOM_TRACE.with(|trace| {
        assert!(trace.borrow().traces[DEFAULT_TAG].print);
    });

    // Test with a custom tag
    activate_trace_print!(custom_tag);

    NOM_TRACE.with(|trace| {
        assert!(trace.borrow().traces["custom_tag"].print);
    });
}

#[test]
#[cfg(feature = "trace-print")]
fn test_deactivate_trace_print() {
    // First, activate trace print
    activate_trace_print!();

    // Ensure trace print is initially activated
    NOM_TRACE.with(|trace| {
        assert!(trace.borrow().traces[DEFAULT_TAG].print);
    });

    // Deactivate trace print
    deactivate_trace_print!();

    // Check if trace print is now deactivated
    NOM_TRACE.with(|trace| {
        assert!(!trace.borrow().traces[DEFAULT_TAG].print);
    });

    // Test with a custom tag
    activate_trace_print!(custom_tag);
    deactivate_trace_print!(custom_tag);

    NOM_TRACE.with(|trace| {
        assert!(!trace.borrow().traces["custom_tag"].print);
    });
}

#[test]
#[cfg(feature = "trace-print")]
fn test_activate_deactivate_trace_print_interaction() {
    // Activate trace print for default tag
    activate_trace_print!();

    // Activate trace print for custom tag
    activate_trace_print!(custom_tag);

    // Verify both are activated
    NOM_TRACE.with(|trace| {
        let trace = trace.borrow();
        assert!(trace.traces[DEFAULT_TAG].print);
        assert!(trace.traces["custom_tag"].print);
    });

    // Deactivate trace print for default tag
    deactivate_trace_print!();

    // Verify only custom tag is still activated
    NOM_TRACE.with(|trace| {
        let trace = trace.borrow();
        assert!(!trace.traces[DEFAULT_TAG].print);
        assert!(trace.traces["custom_tag"].print);
    });

    // Deactivate trace print for custom tag
    deactivate_trace_print!(custom_tag);

    // Verify both are deactivated
    NOM_TRACE.with(|trace| {
        let trace = trace.borrow();
        assert!(!trace.traces[DEFAULT_TAG].print);
        assert!(!trace.traces["custom_tag"].print);
    });
}

#[test]
fn test_reset_trace() {
    activate_trace!();
    let result = trace!(parse_ab)("ab");
    assert!(result.is_ok());

    let trace_before = get_trace();
    assert!(
        !trace_before.is_empty(),
        "Trace should not be empty before reset"
    );

    reset_trace!();
    let trace_after = get_trace();
    assert!(trace_after.is_empty(), "Trace should be empty after reset");

    let result = trace!(parse_ab)("ab");
    assert!(result.is_ok());

    let trace_final = get_trace();
    assert!(
        !trace_final.is_empty(),
        "Trace should not be empty after new parsing"
    );
    assert!(trace_final.contains("test_reset_trace"));
    assert!(trace_final.contains("-> Ok"));
}

#[test]
fn test_reset_trace_with_tag() {
    activate_trace!(custom_tag);
    let result = trace!(custom_tag, parse_ab)("ab");
    assert!(result.is_ok());

    let trace_before = get_trace_for_tag("custom_tag");
    assert!(
        !trace_before.is_empty(),
        "Trace should not be empty before reset"
    );

    reset_trace!(custom_tag);
    let trace_after = get_trace_for_tag("custom_tag");
    assert!(trace_after.is_empty(), "Trace should be empty after reset");

    let result = trace!(custom_tag, parse_ab)("ab");
    assert!(result.is_ok());

    let trace_final = get_trace_for_tag("custom_tag");
    assert!(
        !trace_final.is_empty(),
        "Trace should not be empty after new parsing"
    );
    assert!(trace_final.contains("test_reset_trace_with_tag"));
    assert!(trace_final.contains("-> Ok"));
}
