// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "trace")]
use crate::tags::TraceTags;
#[cfg(feature = "trace-silencing")]
use crate::traces::Trace;
#[cfg(feature = "trace-context")]
use nom::error::ContextError;
use {
    nom::{IResult, Parser},
    std::fmt::Debug,
};

#[cfg(feature = "trace-color")]
#[allow(dead_code)]
pub(crate) mod ansi;
#[cfg(feature = "trace")]
pub mod events;
#[cfg(feature = "trace")]
pub mod tags;
#[cfg(feature = "trace")]
pub mod traces;

pub mod macros;

pub const DEFAULT_TAG: &str = "default";

thread_local! {
    /// Thread-local storage for the global [TraceTags].
    ///
    /// [TRACE_TAGS] provides a thread-safe way to access and modify the global trace list.
    /// It's implemented as thread-local storage, ensuring that each thread has its own
    /// independent trace list. This allows for concurrent tracing in multithreaded applications
    /// without the need for explicit synchronization.
    ///
    /// The [RefCell](std::cell::RefCell) allows for interior mutability, so the [TraceTags] can be
    /// modified even when accessed through a shared reference.
    ///
    /// Usage of [TRACE_TAGS] is typically wrapped in the [tr] and [tr_tag_ctx] functions,
    /// which provide a more convenient interface for adding trace events.
    #[cfg(feature = "trace")]
    pub static TRACE_TAGS: std::cell::RefCell<TraceTags> = std::cell::RefCell::new(TraceTags::new());

    /// Thread-local storage for silent tracing (used with trace-silencing feature)
    #[cfg(feature = "trace-silencing")]
    pub static TRACE_SILENT: std::cell::RefCell<Trace> = std::cell::RefCell::new(Trace::default());

    /// Thread-local storage for tree silence levels (used with trace-silencing feature)
    #[cfg(feature = "trace-silencing")]
    pub static TREE_SILENCE_LEVELS: std::cell::RefCell<Vec<usize>> = const { std::cell::RefCell::new(vec![]) };
}

#[cfg(feature = "trace-context")]
pub trait TraceError<I>: Debug + ContextError<I> {}
#[cfg(feature = "trace-context")]
impl<I, E> TraceError<I> for E where E: Debug + ContextError<I> {}

#[cfg(not(feature = "trace-context"))]
pub trait TraceError<I>: Debug {}
#[cfg(not(feature = "trace-context"))]
impl<I, E> TraceError<I> for E where E: Debug {}

/// Main tracing function that wraps a parser with tracing functionality.
///
/// This function is the core of nom-tracer. It wraps a parser and adds tracing capabilities,
/// recording the parser's execution path and results.
///
/// # Arguments
///
/// * `tag` - A static string used to categorize the trace events.
/// * `context` - An optional static string providing additional context for the trace.
/// * `name` - A static string identifying the parser being traced.
/// * `parser` - The parser function to be wrapped with tracing.
pub fn tr<I, O, E, F>(
    #[cfg(feature = "trace")] tag: &'static str,
    #[cfg(not(feature = "trace"))] _tag: &'static str,
    context: Option<&'static str>,
    #[cfg(feature = "trace")] name: &'static str,
    #[cfg(not(feature = "trace"))] _name: &'static str,
    mut parser: F,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: AsRef<str>,
    F: Parser<I, O, E>,
    I: Clone,
    O: Debug,
    E: TraceError<I>,
{
    #[cfg(feature = "trace")]
    {
        move |input: I| {
            let input1 = input.clone();
            let input2 = input.clone();
            #[cfg(feature = "trace-context")]
            let input3 = input.clone();

            #[cfg(feature = "trace-silencing")]
            let silent = TREE_SILENCE_LEVELS.with(|levels| !levels.borrow().is_empty());

            #[cfg(feature = "trace-silencing")]
            if silent {
                TRACE_SILENT.with(|trace| {
                    (*trace.borrow_mut()).open(context, input1, name, true);
                });
            } else {
                TRACE_TAGS.with(|tags| {
                    (*tags.borrow_mut()).open(tag, context, input1, name, false);
                });
            };
            #[cfg(not(feature = "trace-silencing"))]
            TRACE_TAGS.with(|tags| {
                (*tags.borrow_mut()).open(tag, context, input1, name, false);
            });

            let res = parser.parse(input);

            #[cfg(feature = "trace-silencing")]
            if silent {
                TRACE_SILENT.with(|trace| {
                    (*trace.borrow_mut()).close(context, input2, name, &res, true);
                });
            } else {
                TRACE_TAGS.with(|tags| {
                    (*tags.borrow_mut()).close(tag, context, input2, name, &res, false);
                });
            }

            #[cfg(not(feature = "trace-silencing"))]
            TRACE_TAGS.with(|tags| {
                (*tags.borrow_mut()).close(tag, context, input2, name, &res, false);
            });

            #[cfg(not(feature = "trace-context"))]
            return res;

            #[cfg(feature = "trace-context")]
            if let Some(context) = context {
                add_context_to_err(context, input3, res)
            } else {
                res
            }
        }
    }

    #[cfg(not(feature = "trace"))]
    {
        #[cfg(feature = "trace-context")]
        {
            move |input: I| {
                if let Some(context) = context {
                    add_context_to_err(context, input.clone(), parser.parse(input))
                } else {
                    parser.parse(input)
                }
            }
        }

        #[cfg(not(feature = "trace-context"))]
        move |input: I| parser.parse(input)
    }
}

/// Function to silence tracing for a subtree of parsers.
///
/// This is used to reduce noise in the trace output for well-tested or less interesting
/// parts of the parser.
///
/// # Arguments
///
/// * `tag` - A static string used to categorize the trace events.
/// * `context` - An optional static string providing additional context for the trace.
/// * `name` - A static string identifying the parser being silenced.
/// * `parser` - The parser function to be silenced.
#[cfg(feature = "trace-silencing")]
pub fn silence_tree<I, O, E, F>(
    tag: &'static str,
    context: Option<&'static str>,
    name: &'static str,
    mut parser: F,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: AsRef<str>,
    F: Parser<I, O, E>,
    I: Clone,
    O: Debug,
    E: TraceError<I>,
{
    move |input: I| {
        let input1 = input.clone();
        let input2 = input.clone();
        let input3 = input.clone();

        let cut_level = TRACE_TAGS.with(|tags| (*tags.borrow_mut()).level_for_tag(tag));

        TREE_SILENCE_LEVELS.with(|levels| {
            (*levels.borrow_mut()).push(cut_level);
        });

        TRACE_SILENT.with(|trace| {
            (*trace.borrow_mut()).set_level(cut_level);
            (*trace.borrow_mut()).open(context, input1, name, true);
        });

        let res = parser.parse(input);

        TRACE_SILENT.with(|trace| {
            (*trace.borrow_mut()).close(context, input2, name, &res, true);
        });

        TREE_SILENCE_LEVELS.with(|levels| {
            (*levels.borrow_mut()).pop();
        });

        #[cfg(feature = "trace-context")]
        return add_context_to_err(name, input3, res);

        #[cfg(not(feature = "trace-context"))]
        res
    }
}

/// Helper function to add context to error results.
///
/// This is used when the trace-context feature is enabled to provide more
/// detailed error information.
#[cfg(feature = "trace-context")]
fn add_context_to_err<I, O, E>(
    name: &'static str,
    input: I,
    res: IResult<I, O, E>,
) -> IResult<I, O, E>
where
    I: AsRef<str>,
    I: Clone,
    O: Debug,
    E: TraceError<I>,
{
    match res {
        Ok(o) => Ok(o),
        Err(nom::Err::Error(e)) => Err(nom::Err::Error(E::add_context(input, name, e))),
        Err(nom::Err::Failure(e)) => Err(nom::Err::Failure(E::add_context(input, name, e))),
        Err(nom::Err::Incomplete(i)) => Err(nom::Err::Incomplete(i)),
    }
}

// TODO: Return `Option<String>` instead.
/// Retrieves the trace for a specific tag.
///
/// # Arguments
///
/// * `tag` - A static string identifying the tag for which to retrieve the trace.
///
/// # Returns
///
/// Returns a string representation of the trace, or a message if no trace is found.
pub fn get_trace_for_tag(
    #[cfg(feature = "trace")] tag: &'static str,
    #[cfg(not(feature = "trace"))] _tag: &'static str,
) -> String {
    #[cfg(feature = "trace")]
    {
        TRACE_TAGS.with(|trace| {
            if let Some(trace) = trace.borrow().traces.get(tag) {
                trace.to_string()
            } else {
                format!("No trace found for tag '{}'", tag)
            }
        })
    }

    #[cfg(not(feature = "trace"))]
    String::new()
}

/// Prints the trace for a specific tag.
///
/// # Arguments
///
/// * `tag` - A static string identifying the tag for which to print the trace.
pub fn print_trace_for_tag(tag: &'static str) {
    print(get_trace_for_tag(tag));
}

// TODO: Remove and use `std` instead.
/// Helper function to print a string.
///
/// # Arguments
///
/// * `s` - The string to be printed.
pub(crate) fn print<I: AsRef<str>>(s: I) {
    use std::io::Write;
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    write!(handle, "{}", s.as_ref()).unwrap();
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        nom::{bytes::complete::tag, error::VerboseError},
    };

    #[cfg(feature = "trace")]
    mod trace_tests {
        use {super::*, nom::sequence::tuple};

        #[test]
        fn test_tr_no_context() {
            let mut parser = tr(
                DEFAULT_TAG,
                None,
                "test_parser",
                tag::<_, _, VerboseError<_>>("hello"),
            );
            let result = parser("hello world");
            assert!(result.is_ok());

            let trace = get_trace_for_tag(DEFAULT_TAG);
            assert!(trace.contains("test_parser"));
            assert!(trace.contains("hello world"));
            assert!(trace.contains("-> Ok"));
        }

        #[test]
        fn test_tr_context() {
            let mut parser = tr(
                DEFAULT_TAG,
                Some("context"),
                "test_parser",
                tag::<_, _, VerboseError<_>>("hello"),
            );
            let result = parser("hello world");
            assert!(result.is_ok());

            let trace = get_trace_for_tag(DEFAULT_TAG);
            assert!(trace.contains("test_parser"));
            assert!(trace.contains("context"));
            assert!(trace.contains("hello world"));
            assert!(trace.contains("-> Ok"));
        }

        #[test]
        fn test_nested_traces() {
            fn parse_nested(input: &str) -> IResult<&str, (&str, &str)> {
                tr(
                    DEFAULT_TAG,
                    None,
                    "outer",
                    tuple((
                        tr(DEFAULT_TAG, None, "inner_a", tag("a")),
                        tr(DEFAULT_TAG, None, "inner_b", tag("b")),
                    )),
                )(input)
            }

            let traced_parser = parse_nested;
            let result = traced_parser("ab");
            assert!(result.is_ok());

            let trace = get_trace_for_tag(DEFAULT_TAG);
            assert!(trace.contains("outer"));
            assert!(trace.contains("inner_a"));
            assert!(trace.contains("inner_b"));
        }

        #[test]
        fn test_get_trace_for_tag() {
            let mut parser = tr(
                DEFAULT_TAG,
                None,
                "test_parser",
                tag::<_, _, VerboseError<_>>("hello"),
            );
            let _ = parser("hello world");

            let trace = get_trace_for_tag(DEFAULT_TAG);
            assert!(trace.contains("test_parser"));
            assert!(trace.contains("hello world"));
        }

        #[test]
        fn test_get_trace_for_nonexistent_tag() {
            let trace = get_trace_for_tag("nonexistent");
            assert_eq!(trace, "No trace found for tag 'nonexistent'");
        }
    }

    #[cfg(feature = "trace-silencing")]
    mod trace_silencing_tests {
        use super::*;

        #[test]
        fn test_silence_tree() {
            let mut parser = silence_tree(
                DEFAULT_TAG,
                Some("context"),
                "silent_parser",
                tag::<_, _, VerboseError<_>>("hello"),
            );
            let result = parser("hello world");
            assert!(result.is_ok());

            let trace = get_trace_for_tag(DEFAULT_TAG);
            assert!(!trace.contains("silent_parser"));
        }

        #[test]
        fn test_silence_tree_with_nested_parsers() {
            let mut outer_parser = tr(
                DEFAULT_TAG,
                None,
                "outer_parser",
                silence_tree(
                    DEFAULT_TAG,
                    None,
                    "inner_parser",
                    tr(
                        DEFAULT_TAG,
                        None,
                        "inner",
                        tag::<_, _, VerboseError<_>>("hello"),
                    ),
                ),
            );

            let result = outer_parser("hello world");
            assert!(result.is_ok());

            let trace = get_trace_for_tag(DEFAULT_TAG);
            assert!(trace.contains("outer_parser"));
            assert!(!trace.contains("inner_parser"));
        }
    }

    #[cfg(all(feature = "trace", feature = "trace-context"))]
    mod trace_context_tests {
        use {
            super::*,
            nom::error::{ErrorKind, VerboseErrorKind},
        };

        #[test]
        fn test_add_context_to_err() {
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

    #[cfg(not(feature = "trace"))]
    mod no_trace_tests {
        use {
            super::*,
            nom::error::{ErrorKind, ParseError, VerboseErrorKind},
        };

        #[test]
        fn test_tr_without_trace() {
            let mut parser = tr(
                DEFAULT_TAG,
                Some("context"),
                "test_parser",
                tag::<_, _, VerboseError<_>>("hello"),
            );
            let result = parser("hello world");
            assert!(result.is_ok());
            assert_eq!(result, Ok((" world", "hello")));
        }

        #[test]
        fn test_get_trace_for_tag_without_trace() {
            let trace = get_trace_for_tag(DEFAULT_TAG);
            assert_eq!(trace, "");
        }

        #[cfg(feature = "trace-context")]
        mod context_without_trace_tests {
            use {
                super::*,
                nom::error::{ErrorKind, ParseError, VerboseErrorKind},
            };

            #[test]
            fn test_context_addition_without_trace() {
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

        #[cfg(not(feature = "trace-context"))]
        #[test]
        fn test_error_without_trace_and_context() {
            let mut parser = tr(
                DEFAULT_TAG,
                Some("context"),
                "test_parser",
                tag::<_, _, VerboseError<_>>("hello"),
            );
            let result = parser("world");

            assert!(result.is_err());

            if let Err(nom::Err::Error(e)) = result {
                println!("{:?}", e);
                assert_eq!(e.errors.len(), 1);
                assert_eq!(e.errors[0].1, VerboseErrorKind::Nom(ErrorKind::Tag));
            } else {
                panic!("Expected Err(nom::Err::Error)");
            }
        }
    }
}
