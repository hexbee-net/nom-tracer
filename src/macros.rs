// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

/// Internal macro to get the current function name.
///
/// This macro is used internally by other macros to automatically capture
/// the name of the function where it is called.
#[doc(hidden)]
#[macro_export]
macro_rules! __fn_name {
    () => {{
        struct Here;
        const PREFIX: &str = concat!(module_path!(), "::");
        const SUFFIX: &str = "::Here";
        let here = core::any::type_name::<Here>();
        &here[PREFIX.len()..(here.len() - SUFFIX.len())]
    }};
}

/// Traces a parser with optional tag and context, automatically capturing the function name.
///
/// This macro provides a flexible interface to trace parsers, combining the functionality
/// of `tr_tag_ctx` function with automatic function name capture. It allows for various
/// combinations of parser, tag, and context arguments.
///
/// # Arguments
///
/// The macro accepts different combinations of arguments:
///
/// * `$parser`: The parser to be wrapped (required in all cases).
/// * `$tag`: An optional identifier used to categorize the trace.
/// * `$context`: An optional expression providing additional context for the parser.
///
/// # Usage Patterns
///
/// 1. `trace!(parser)`:
///    Uses the default tag and no context.
///
/// 2. `trace!(tag, parser)`:
///    Uses a custom tag and no context.
///
/// 3. `trace!("context", parser)`:
///    Uses the default tag and a custom context.
///
/// 4. `trace!(tag, "context", parser)`:
///    Uses a custom tag and a custom context.
///
/// # Examples
///
/// Basic usage with default tag and no context:
///
/// ```
/// use nom_tracer::trace;
/// use nom::bytes::complete::tag;
/// use nom::IResult;
///
/// fn parse_hello(input: &str) -> IResult<&str, &str> {
///     trace!(tag("hello"))(input)
/// }
/// ```
///
/// Using a custom tag:
///
/// ```
/// use nom_tracer::trace;
/// use nom::bytes::complete::tag;
/// use nom::IResult;
///
/// fn parse_greeting(input: &str) -> IResult<&str, &str> {
///     trace!(greeting, tag("hello"))(input)
/// }
/// ```
///
/// Using a custom context:
///
/// ```
/// use nom_tracer::trace;
/// use nom::bytes::complete::tag;
/// use nom::IResult;
///
/// fn parse_farewell(input: &str) -> IResult<&str, &str> {
///     trace!("Parsing hello", tag("hello"))(input)
/// }
/// ```
///
/// Using both custom tag and context:
///
/// ```
/// use nom_tracer::trace;
/// use nom::bytes::complete::tag;
/// use nom::IResult;
///
/// fn parse_greeting(input: &str) -> IResult<&str, &str> {
///     trace!(greeting, "Parsing hello", tag("hello"))(input)
/// }
/// ```
///
/// # Notes
///
/// - The function name is automatically captured and used as the location in the trace.
/// - When using a custom tag, it should be provided as an identifier, not a string literal.
/// - The context, if provided, can be any expression that evaluates to a `&str`.
/// - This macro internally uses `tr_tag_ctx` for all tracing operations.
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! trace {
    ($parser:expr $(,)?) => {{
        let caller = $crate::__fn_name!();
        $crate::tr_tag_ctx($crate::DEFAULT_TAG, None, caller, $parser)
    }};

    ($tag:ident, $parser:expr $(,)?) => {{
        let caller = $crate::__fn_name!();
        $crate::tr_tag_ctx(stringify!($tag), None, caller, $parser)
    }};

    ($context:expr, $parser:expr $(,)?) => {{
        let caller = $crate::__fn_name!();
        $crate::tr_tag_ctx($crate::DEFAULT_TAG, Some($context), caller, $parser)
    }};

    ($tag:ident, $context:expr, $parser:expr $(,)?) => {{
        let caller = $crate::__fn_name!();
        $crate::tr_tag_ctx(stringify!($tag), Some($context), caller, $parser)
    }};
}
#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! trace {
    ($parser:expr $(,)?) => {{ $parser }};
    ($tag:ident, $parser:expr $(,)?) => {{ $parser }};
    ($context:expr, $parser:expr $(,)?) => {{ $parser }};
    ($tag:ident, $context:expr, $parser:expr $(,)?) => {{ $parser }};
}

/// Activates tracing for either the default tag or a specified tag.
///
/// This macro enables the recording of trace events for parsers wrapped with tracing functions.
///
/// # Examples
///
/// Activate tracing for the default tag:
///
/// ```
/// use nom_tracer::activate_trace;
///
/// activate_trace!();
/// // Tracing is now active for parsers using the default tag
/// ```
///
/// Activate tracing for a custom tag:
///
/// ```
/// use nom_tracer::activate_trace;
///
/// activate_trace!(my_custom_tag);
/// // Tracing is now active for parsers using the "my_custom_tag" tag
/// ```
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! activate_trace (
    () => {
        $crate::NOM_TRACE.with(|trace| {
            trace.borrow_mut().activate($crate::DEFAULT_TAG);
        });
    };
    ($tag:ident) => {
        $crate::NOM_TRACE.with(|trace| {
            trace.borrow_mut().activate(stringify!($tag));
        });
    };
);
#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! activate_trace (
    () => {};
    ($tag:ident) => {};
);

/// Deactivates tracing for either the default tag or a specified tag.
///
/// This macro disables the recording of trace events for parsers wrapped with tracing functions.
/// Previously recorded events are retained, but no new events will be recorded until tracing is reactivated.
///
/// # Examples
///
/// Deactivate tracing for the default tag:
///
/// ```
/// use nom_tracer::deactivate_trace;
///
/// deactivate_trace!();
/// // Tracing is now inactive for parsers using the default tag
/// ```
///
/// Deactivate tracing for a custom tag:
///
/// ```
/// use nom_tracer::deactivate_trace;
///
/// deactivate_trace!(my_custom_tag);
/// // Tracing is now inactive for parsers using the "my_custom_tag" tag
/// ```
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! deactivate_trace (
    () => {
        $crate::NOM_TRACE.with(|trace| {
            trace.borrow_mut().deactivate($crate::DEFAULT_TAG);
        });
    };

    ($tag:ident) => {
        $crate::NOM_TRACE.with(|trace| {
            trace.borrow_mut().deactivate(stringify!($tag));
        });
    };
);
#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! deactivate_trace (
    () => {};
    ($tag:ident) => {};
);

/// Resets the trace for either the default tag or a specified tag.
///
/// This macro clears all recorded events and resets the nesting level for the specified trace.
/// If the trace doesn't exist, a new one is created.
///
/// # Examples
///
/// Reset the trace for the default tag:
///
/// ```
/// use nom_tracer::reset_trace;
///
/// reset_trace!();
/// // All trace events for the default tag are now cleared
/// ```
///
/// Reset the trace for a custom tag:
///
/// ```
/// use nom_tracer::reset_trace;
///
/// reset_trace!(my_custom_tag);
/// // All trace events for the "my_custom_tag" tag are now cleared
/// ```
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! reset_trace (
    () => {
        $crate::NOM_TRACE.with(|trace| {
            trace.borrow_mut().reset($crate::DEFAULT_TAG);
        });
    };

    ($tag:ident) => {
        $crate::NOM_TRACE.with(|trace| {
            trace.borrow_mut().reset(stringify!($tag));
        });
    };
);
#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! reset_trace (
    () => {};
    ($tag:ident) => {};
);

/// Sets the maximum nesting level for tracing before panic.
///
/// This macro sets a limit on the nesting level of traces. If the nesting level
/// exceeds this limit, the program will panic. This can be useful for detecting
/// infinite recursion or excessively deep parser nesting.
///
/// # Arguments
///
/// * `$level`: An expression that evaluates to `Option<usize>`. Use `Some(n)` to set a limit,
///   or `None` to remove the limit.
///
/// # Examples
///
/// Set a maximum level for the default tag:
///
/// ```
/// use nom_tracer::max_level;
///
/// max_level!(Some(10));
/// // Tracing will panic if nesting exceeds 10 levels for the default tag
/// ```
///
/// Set a maximum level for a custom tag:
///
/// ```
/// use nom_tracer::max_level;
///
/// max_level!(my_custom_tag, Some(5));
/// // Tracing will panic if nesting exceeds 5 levels for "my_custom_tag"
/// ```
///
/// Remove the level limit:
///
/// ```
/// use nom_tracer::max_level;
///
/// max_level!(None);
/// // Removes the nesting level limit for the default tag
/// ```
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! max_level (
    ($level:expr) => {
        $crate::NOM_TRACE.with(|trace| {
            trace.borrow_mut().panic_on_level($crate::DEFAULT_TAG, $level);
        });
    };
    ($tag:ident, $level:expr) => {
        $crate::NOM_TRACE.with(|trace| {
            trace.borrow_mut().panic_on_level(stringify!($tag, $level));
        });
    };
);
#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! max_level (
    ($level:expr) => {};
    ($tag:ident, $level:expr) => {};
);

/// Retrieves the trace for either the default tag or a specified tag.
///
/// This macro provides a convenient way to get the trace as a string for either
/// the default tag or a custom tag.
///
/// # Examples
///
/// Get trace for the default tag:
///
/// ```
/// use nom_tracer::get_trace;
///
/// let default_trace = get_trace!();
/// println!("Default trace:\n{}", default_trace);
/// ```
///
/// Get trace for a custom tag:
///
/// ```
/// use nom_tracer::get_trace;
///
/// let custom_trace = get_trace!(my_custom_tag);
/// println!("Custom trace:\n{}", custom_trace);
/// ```
#[macro_export]
macro_rules! get_trace {
    () => {
        $crate::get_trace_for_tag($crate::DEFAULT_TAG);
    };
    ($tag:ident) => {
        $crate::get_trace_for_tag(stringify!($tag));
    };
}

/// Prints the trace for either the default tag or a specified tag.
///
/// # Examples
///
/// ```
/// use nom_tracer::{trace, print_trace};
/// use nom::bytes::complete::tag;
///
/// let _ = trace!(tag::<&str, &str, nom::error::VerboseError<_>>("hello"))("hello world");
/// print_trace!(); // Prints trace for default tag
///
/// let _ = trace!(my_tag, tag::<&str, &str, nom::error::VerboseError<_>>("hello"))("hello world");
/// print_trace!(my_tag); // Prints trace for "my_tag"
/// ```
#[macro_export]
macro_rules! print_trace {
    () => {
        $crate::print_trace_for_tag($crate::DEFAULT_TAG);
    };
    ($tag:ident) => {
        $crate::print_trace_for_tag(stringify!($tag));
    };
}
