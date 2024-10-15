// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

use {
    nom::{bytes::complete::tag, sequence::tuple, IResult},
    nom_tracer::{
        activate_trace,
        deactivate_trace,
        get_trace,
        get_trace_for_tag,
        reset_trace,
        trace,
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
