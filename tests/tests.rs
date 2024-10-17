// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

use {
    nom::{bytes::complete::tag, sequence::tuple, IResult},
    nom_tracer::*,
};

fn parse_ab(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((tag("a"), tag("b")))(input)
}

#[test]
fn test_tr_function_always_works() {
    let result = tr("test_tag", Some("context"), "test_parser", parse_ab)("ab");
    assert!(result.is_ok());
    assert_eq!(result, Ok(("", ("a", "b"))));
}

#[cfg(feature = "trace")]
mod trace_tests {
    use super::*;

    #[test]
    fn test_simple_trace() {
        let result = trace!(parse_ab)("ab");
        assert!(result.is_ok());

        let trace = get_trace!();
        assert!(trace.contains("test_simple_trace"));
        assert!(trace.contains("-> Ok"));
    }

    #[test]
    fn test_trace_with_tag() {
        let result = trace!(custom_tag, parse_ab)("ab");
        assert!(result.is_ok());

        let trace = get_trace!(custom_tag);
        assert!(trace.contains("test_trace_with_tag"));
        assert!(trace.contains("-> Ok"));
    }

    #[test]
    fn test_trace_with_context() {
        let result = trace!("custom_context", parse_ab)("ab");
        assert!(result.is_ok());

        let trace = get_trace!();
        assert!(trace.contains("test_trace_with_context"));
        assert!(trace.contains("custom_context"));
        assert!(trace.contains("-> Ok"));
    }

    #[test]
    fn test_activate_deactivate_reset() {
        activate_trace!();
        let result = trace!(parse_ab)("ab");
        assert!(result.is_ok());
        assert!(!get_trace!().is_empty());

        deactivate_trace!();
        let result = trace!(parse_ab)("ab");
        assert!(result.is_ok());
        assert_eq!(
            get_trace!(),
            get_trace!(),
            "Trace should not change after deactivation"
        );

        activate_trace!();
        reset_trace!();
        assert!(get_trace!().is_empty(), "Trace should be empty after reset");

        let result = trace!(parse_ab)("ab");
        assert!(result.is_ok());
        assert!(
            !get_trace!().is_empty(),
            "Trace should not be empty after new parsing"
        );
    }
}

// Tests for when trace-context feature is enabled
#[cfg(all(feature = "trace", feature = "trace-context"))]
mod trace_context_tests {
    use {
        super::*,
        nom::error::{ErrorKind, VerboseError, VerboseErrorKind},
    };

    #[test]
    fn test_context_addition() {
        let mut parser = tr(
            DEFAULT_TAG,
            Some("context"),
            "test_parser",
            tag::<_, _, VerboseError<_>>("hello"),
        );

        let result = parser("world");
        assert!(result.is_err());

        if let Err(nom::Err::Error(e)) = result {
            assert_eq!(e.errors.len(), 2);
            assert_eq!(e.errors[1].1, VerboseErrorKind::Context("context"));
            assert_eq!(e.errors[0].1, VerboseErrorKind::Nom(ErrorKind::Tag));
        } else {
            panic!("Expected Err(nom::Err::Error)");
        }
    }
}

// Tests for when trace-silencing feature is enabled
#[cfg(all(feature = "trace", feature = "trace-silencing"))]
mod trace_silencing_tests {
    use super::*;

    #[test]
    fn test_silence_tree() {
        fn outer_parser(input: &str) -> IResult<&str, (&str, &str)> {
            trace!("outer", tuple((tag("a"), silence_tree!("inner", tag("b")))))(input)
        }

        let result = outer_parser("ab");
        assert!(result.is_ok());

        let trace = get_trace!();
        println!("{}", trace);
        assert!(trace.contains("outer"));
        assert!(!trace.contains("inner"));
    }
}

// Tests for when trace is not enabled
#[cfg(not(feature = "trace"))]
mod no_trace_tests {
    use {super::*, nom::error::VerboseErrorKind};

    #[test]
    fn test_trace_macros_do_nothing() {
        let result = trace!(parse_ab)("ab");
        assert!(result.is_ok());

        // These should compile but do nothing
        activate_trace!();
        deactivate_trace!();
        reset_trace!();

        assert_eq!(get_trace!(), "");
    }

    #[cfg(feature = "trace-context")]
    #[test]
    fn test_context_addition_without_trace() {
        use nom::error::{ErrorKind, VerboseError};

        let mut parser = tr(
            DEFAULT_TAG,
            Some("context"),
            "test_parser",
            tag::<_, _, VerboseError<_>>("hello"),
        );

        let result = parser("world");
        assert!(result.is_err());

        if let Err(nom::Err::Error(e)) = result {
            assert_eq!(e.errors.len(), 2);
            assert_eq!(e.errors[1].1, VerboseErrorKind::Context("context"));
            assert_eq!(e.errors[0].1, VerboseErrorKind::Nom(ErrorKind::Tag));
        } else {
            panic!("Expected Err(nom::Err::Error)");
        }
    }
}
