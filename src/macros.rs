// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

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

/// Adds tracing to a parser.
///
/// This macro wraps a parser with tracing functionality.
/// It can be used with different combinations of tags, contexts, and parsers.
///
/// # Usage
///
/// - `trace!(parser)`: Uses the default tag and no context.
/// - `trace!(tag, parser)`: Uses a custom tag and no context.
/// - `trace!("context", parser)`: Uses the default tag and a custom context.
/// - `trace!(tag, "context", parser)`: Uses a custom tag and a custom context.
///
/// When the trace feature is disabled, this macro becomes a no-op and simply returns the parser.
#[cfg(any(feature = "trace", feature = "trace-context"))]
#[macro_export]
macro_rules! trace {
    ($parser:expr $(,)?) => {
        $crate::tr($crate::DEFAULT_TAG, None, $crate::__fn_name!(), $parser)
    };

    ($tag:ident, $parser:expr $(,)?) => {
        $crate::tr(stringify!($tag), None, $crate::__fn_name!(), $parser)
    };

    ($context:expr, $parser:expr $(,)?) => {
        $crate::tr(
            $crate::DEFAULT_TAG,
            Some($context),
            $crate::__fn_name!(),
            $parser,
        )
    };

    ($tag:ident, $context:expr, $parser:expr $(,)?) => {
        $crate::tr(
            stringify!($tag),
            Some($context),
            $crate::__fn_name!(),
            $parser,
        )
    };

    ($caller:expr, $tag:ident, $parser:expr $(,)?) => {{
        $crate::tr(stringify!($tag), None, $caller, $parser)
    }};

    ($caller:expr, $context:expr, $parser:expr $(,)?) => {{
        $crate::tr($crate::DEFAULT_TAG, Some($context), $caller, $parser)
    }};

    ($caller:expr, $tag:ident, $context:expr, $parser:expr $(,)?) => {{
        $crate::tr(stringify!($tag), Some($context), $caller, $parser)
    }};
}
#[cfg(not(any(feature = "trace", feature = "trace-context")))]
#[macro_export]
macro_rules! trace {
    ($parser:expr $(,)?) => {
        $parser
    };
    ($tag:ident, $parser:expr $(,)?) => {
        $parser
    };
    ($context:expr, $parser:expr $(,)?) => {
        $parser
    };
    ($tag:ident, $context:expr, $parser:expr $(,)?) => {
        $parser
    };

    ($caller:expr, $tag:ident, $parser:expr $(,)?) => {{
        $parser
    }};

    ($caller:expr, $context:expr, $parser:expr $(,)?) => {{
        $parser
    }};

    ($caller:expr, $tag:ident, $context:expr, $parser:expr $(,)?) => {{
        $parser
    }};
}

/// Silences the tracing for a subtree of parsers.
///
/// This macro wraps a parser and prevents it and its sub-parsers from generating trace output.
/// It's useful for reducing noise in the trace output for well-tested or less interesting parts of your parser.
///
/// # Usage
///
/// - `silence_tree!(parser)`: Silences the default tag.
/// - `silence_tree!(tag, parser)`: Silences a specific tag.
/// - `silence_tree!("context", parser)`: Silences the default tag with a context.
/// - `silence_tree!(tag, "context", parser)`: Silences a specific tag with a context.
#[cfg(feature = "trace-silencing")]
#[macro_export]
macro_rules! silence_tree {
    ($parser:expr $(,)?) => {{
        let caller = $crate::__fn_name!();
        $crate::silence_tree($crate::DEFAULT_TAG, None, caller, $parser)
    }};

    ($tag:ident, $parser:expr $(,)?) => {{
        let caller = $crate::__fn_name!();
        $crate::silence_tree(stringify!($tag), None, caller, $parser)
    }};

    ($context:expr, $parser:expr $(,)?) => {{
        let caller = $crate::__fn_name!();
        $crate::silence_tree($crate::DEFAULT_TAG, Some($context), caller, $parser)
    }};

    ($tag:ident, $context:expr, $parser:expr $(,)?) => {{
        let caller = $crate::__fn_name!();
        $crate::silence_tree(stringify!($tag), Some($context), caller, $parser)
    }};

    ($caller:expr, $tag:ident, $parser:expr $(,)?) => {{
        $crate::silence_tree(stringify!($tag), None, $caller, $parser)
    }};

    ($caller:expr, $context:expr, $parser:expr $(,)?) => {{
        $crate::silence_tree($crate::DEFAULT_TAG, Some($context), $caller, $parser)
    }};

    ($caller:expr, $tag:ident, $context:expr, $parser:expr $(,)?) => {{
        $crate::silence_tree(stringify!($tag), Some($context), $caller, $parser)
    }};
}

/// Activates tracing for a specific tag or the default tag.
///
/// # Usage
///
/// - `activate_trace!()`: Activates tracing for the default tag.
/// - `activate_trace!(tag)`: Activates tracing for a specific tag.
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! activate_trace (
    () => {
        $crate::TRACE_TAGS.with(|trace| {
            trace.borrow_mut().activate($crate::DEFAULT_TAG);
        });
    };
    ($tag:ident) => {
        $crate::TRACE_TAGS.with(|trace| {
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

/// Deactivates tracing for a specific tag or the default tag.
///
/// # Usage
///
/// - `deactivate_trace!()`: Deactivates tracing for the default tag.
/// - `deactivate_trace!(tag)`: Deactivates tracing for a specific tag.
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! deactivate_trace (
    () => {
        $crate::TRACE_TAGS.with(|trace| {
            trace.borrow_mut().deactivate($crate::DEFAULT_TAG);
        });
    };

    ($tag:ident) => {
        $crate::TRACE_TAGS.with(|trace| {
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

/// Activates real-time printing of trace events for a specific tag or the default tag.
///
/// # Usage
///
/// - `activate_trace_print!()`: Activates trace printing for the default tag.
/// - `activate_trace_print!(tag)`: Activates trace printing for a specific tag.
#[cfg(feature = "trace-print")]
#[macro_export]
macro_rules! activate_trace_print (
    () => {
        $crate::TRACE_TAGS.with(|trace| {
            trace.borrow_mut().activate_trace_print($crate::DEFAULT_TAG);
        });
    };
    ($tag:ident) => {
        $crate::TRACE_TAGS.with(|trace| {
            trace.borrow_mut().activate_trace_print(stringify!($tag));
        });
    };
);
#[cfg(not(feature = "trace-print"))]
#[macro_export]
macro_rules! activate_trace_print (
    () => {};
    ($tag:ident) => {};
);

/// Deactivates real-time printing of trace events for a specific tag or the default tag.
///
/// # Usage
///
/// - `deactivate_trace_print!()`: Deactivates trace printing for the default tag.
/// - `deactivate_trace_print!(tag)`: Deactivates trace printing for a specific tag.
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! deactivate_trace_print (
    () => {
        $crate::TRACE_TAGS.with(|trace| {
            trace.borrow_mut().deactivate_trace_print($crate::DEFAULT_TAG);
        });
    };

    ($tag:ident) => {
        $crate::TRACE_TAGS.with(|trace| {
            trace.borrow_mut().deactivate_trace_print(stringify!($tag));
        });
    };
);
#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! deactivate_trace_print (
    () => {};
    ($tag:ident) => {};
);

/// Resets the trace for a specific tag or the default tag.
///
/// This clears all recorded events for the specified tag.
///
/// # Usage
///
/// - `reset_trace!()`: Resets the trace for the default tag.
/// - `reset_trace!(tag)`: Resets the trace for a specific tag.
#[cfg(feature = "trace")]
#[macro_export]
macro_rules! reset_trace (
    () => {
        $crate::TRACE_TAGS.with(|trace| {
            trace.borrow_mut().clear($crate::DEFAULT_TAG);
        });
    };

    ($tag:ident) => {
        $crate::TRACE_TAGS.with(|trace| {
            trace.borrow_mut().clear(stringify!($tag));
        });
    };
);
#[cfg(not(feature = "trace"))]
#[macro_export]
macro_rules! reset_trace (
    () => {};
    ($tag:ident) => {};
);

/// Sets the maximum nesting level for tracing.
///
/// When the nesting level exceeds this value, the parser will panic. This is useful for
/// detecting infinite recursion or excessively deep parser nesting.
///
/// # Usage
///
/// - `set_max_level!(level)`: Sets the max level for the default tag.
/// - `set_max_level!(tag, level)`: Sets the max level for a specific tag.
///
/// The `level` parameter should be an `Option<usize>`. Use `None` to remove the limit.
#[cfg(feature = "trace-max-level")]
#[macro_export]
macro_rules! set_max_level (
    ($level:expr) => {
        $crate::TRACE_TAGS.with(|trace| {
            trace.borrow_mut().panic_on_level($crate::DEFAULT_TAG, $level);
        });
    };
    ($tag:ident, $level:expr) => {
        $crate::TRACE_TAGS.with(|trace| {
            trace.borrow_mut().panic_on_level(stringify!($tag), $level);
        });
    };
);
#[cfg(not(feature = "trace-max-level"))]
#[macro_export]
macro_rules! set_max_level (
    ($level:expr) => {};
    ($tag:ident, $level:expr) => {};
);

/// Retrieves the trace for a specific tag or the default tag.
///
/// # Usage
///
/// - `get_trace!()`: Gets the trace for the default tag.
/// - `get_trace!(tag)`: Gets the trace for a specific tag.
///
/// # Returns
///
/// Returns a `String` containing the trace output.
#[macro_export]
macro_rules! get_trace {
    () => {
        $crate::get_trace_for_tag($crate::DEFAULT_TAG);
    };
    ($tag:ident) => {
        $crate::get_trace_for_tag(stringify!($tag));
    };
}

/// Prints the trace for a specific tag or the default tag.
///
/// # Usage
///
/// - `print_trace!()`: Prints the trace for the default tag.
/// - `print_trace!(tag)`: Prints the trace for a specific tag.
#[macro_export]
macro_rules! print_trace {
    () => {
        $crate::print_trace_for_tag($crate::DEFAULT_TAG);
    };
    ($tag:ident) => {
        $crate::print_trace_for_tag(stringify!($tag));
    };
}
