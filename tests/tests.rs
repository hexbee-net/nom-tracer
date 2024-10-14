// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

// tests/comprehensive_tests.rs

use {
    nom::{bytes::complete::tag, sequence::tuple, IResult},
    nom_tracer::{
        get_trace,
        get_trace_for_tag,
        tr,
        tr_ctx,
        tr_tag,
        tr_tag_ctx,
        TraceList,
        DEFAULT_TAG,
    },
};

fn parse_ab(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((tag("a"), tag("b")))(input)
}

#[test]
fn test_trace_list_new() {
    let trace_list = TraceList::new();
    assert!(trace_list.traces.contains_key(DEFAULT_TAG));
}

#[test]
fn test_trace_list_reset() {
    let mut trace_list = TraceList::new();
    trace_list.open(DEFAULT_TAG, None, "input", "location");
    trace_list.reset(DEFAULT_TAG);
    assert_eq!(trace_list.traces[DEFAULT_TAG].events.len(), 0);
    assert_eq!(trace_list.traces[DEFAULT_TAG].level, 0);
}

#[test]
fn test_trace_list_activate_deactivate() {
    let mut trace_list = TraceList::new();
    trace_list.deactivate(DEFAULT_TAG);
    assert!(!trace_list.traces[DEFAULT_TAG].active);
    trace_list.activate(DEFAULT_TAG);
    assert!(trace_list.traces[DEFAULT_TAG].active);
}

#[test]
fn test_tr() {
    let mut traced_parser = tr("parse_ab", parse_ab);
    let result = traced_parser("ab");
    assert!(result.is_ok());

    let trace = get_trace();
    assert!(trace.contains("parse_ab"));
    assert!(trace.contains("-> Ok"));
}

#[test]
fn test_tr_ctx() {
    let context = "custom";
    let mut traced_parser = tr_ctx("parse_cd", context, parse_ab);
    let result = traced_parser("ab");
    assert!(result.is_ok());

    let trace = get_trace();
    println!("{:?}", trace);
    assert!(trace.contains("parse_cd"));
    assert!(trace.contains("custom"));
    assert!(trace.contains("-> Ok"));
}

#[test]
fn test_tr_tag() {
    let custom_tag = "custom";
    let mut traced_parser = tr_tag(custom_tag, "parse_cd", parse_ab);
    let result = traced_parser("ab");
    assert!(result.is_ok());

    let trace = get_trace_for_tag(custom_tag);
    assert!(trace.contains("parse_cd"));
    assert!(trace.contains("-> Ok"));
}

#[test]
fn test_tr_tag_ctx() {
    let custom_tag = "custom";
    let mut traced_parser = tr_tag_ctx(custom_tag, None, "parse_cd", parse_ab);
    let result = traced_parser("ab");
    assert!(result.is_ok());

    let trace = get_trace_for_tag(custom_tag);
    assert!(trace.contains("parse_cd"));
    assert!(trace.contains("-> Ok"));
}

#[test]
fn test_trace_with_error() {
    fn parse_fail(input: &str) -> IResult<&str, &str> {
        tag("nonexistent")(input)
    }

    let mut traced_parser = tr("parse_fail", parse_fail);
    let result = traced_parser("ab");
    assert!(result.is_err());

    let trace = get_trace();
    assert!(trace.contains("parse_fail"));
    assert!(trace.contains("-> Error"));
}

#[test]
fn test_nested_traces() {
    fn parse_nested(input: &str) -> IResult<&str, (&str, &str)> {
        tr(
            "outer",
            tuple((tr("inner_a", tag("a")), tr("inner_b", tag("b")))),
        )(input)
    }

    let traced_parser = parse_nested;
    let result = traced_parser("ab");
    assert!(result.is_ok());

    let trace = get_trace();
    assert!(trace.contains("outer"));
    assert!(trace.contains("inner_a"));
    assert!(trace.contains("inner_b"));
}
