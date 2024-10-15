use {
    nom::{bytes::complete::tag, sequence::tuple, IResult},
    nom_tracer::{get_trace, get_trace_for_tag, tr, tr_ctx, tr_tag, tr_tag_ctx, DEFAULT_TAG},
};

fn debug_print<I: AsRef<str>>(s: I) {
    use {
        std::io::Write,
        termcolor::{ColorChoice, StandardStream},
    };
    let mut handle = StandardStream::stdout(ColorChoice::Always);
    write!(handle, "{}", s.as_ref()).unwrap();
}

fn parse_ab(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((tag("a"), tag("b")))(input)
}

#[test]
fn test_tr() {
    let mut traced_parser = tr("parse_ab", parse_ab);
    let result = traced_parser("ab");
    assert!(result.is_ok());

    let trace = get_trace();
    debug_print(&trace);
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
    debug_print(&trace);
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
    debug_print(&trace);
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
    debug_print(&trace);
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
    debug_print(&trace);
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
    debug_print(&trace);
    assert!(trace.contains("outer"));
    assert!(trace.contains("inner_a"));
    assert!(trace.contains("inner_b"));
}
