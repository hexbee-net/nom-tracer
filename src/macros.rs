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

#[cfg(feature = "trace")]
#[macro_export]
macro_rules! reset_trace (
    () => {
        $crate::TRACE_TAGS.with(|trace| {
            trace.borrow_mut().reset($crate::DEFAULT_TAG);
        });
    };

    ($tag:ident) => {
        $crate::TRACE_TAGS.with(|trace| {
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
            trace.borrow_mut().panic_on_level(stringify!($tag, $level));
        });
    };
);
#[cfg(not(feature = "trace-max-level"))]
#[macro_export]
macro_rules! set_max_level (
    ($level:expr) => {};
    ($tag:ident, $level:expr) => {};
);

#[macro_export]
macro_rules! get_trace {
    () => {
        $crate::get_trace_for_tag($crate::DEFAULT_TAG);
    };
    ($tag:ident) => {
        $crate::get_trace_for_tag(stringify!($tag));
    };
}

#[macro_export]
macro_rules! print_trace {
    () => {
        $crate::print_trace_for_tag($crate::DEFAULT_TAG);
    };
    ($tag:ident) => {
        $crate::print_trace_for_tag(stringify!($tag));
    };
}
