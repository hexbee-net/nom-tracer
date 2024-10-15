// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

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
/// activate_trace!("my_custom_tag");
/// // Tracing is now active for parsers using the "my_custom_tag" tag
/// ```
#[macro_export]
macro_rules! activate_trace (
    () => {
        $crate::NOM_TRACE.with(|trace| {
            trace.borrow_mut().activate($crate::DEFAULT_TAG);
        });
    };
    ($tag:expr) => {
        $crate::NOM_TRACE.with(|trace| {
            trace.borrow_mut().activate($tag);
        });
    };
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
/// deactivate_trace!("my_custom_tag");
/// // Tracing is now inactive for parsers using the "my_custom_tag" tag
/// ```
#[macro_export]
macro_rules! deactivate_trace (
    () => {
        $crate::NOM_TRACE.with(|trace| {
            trace.borrow_mut().deactivate($crate::DEFAULT_TAG);
        });
    };

    ($tag:expr) => {
        $crate::NOM_TRACE.with(|trace| {
            trace.borrow_mut().deactivate($tag);
        });
    };
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
/// reset_trace!("my_custom_tag");
/// // All trace events for the "my_custom_tag" tag are now cleared
/// ```
#[macro_export]
macro_rules! reset_trace (
    () => {
        $crate::NOM_TRACE.with(|trace| {
            trace.borrow_mut().reset($crate::DEFAULT_TAG);
        });
    };

    ($tag:expr) => {
        $crate::NOM_TRACE.with(|trace| {
            trace.borrow_mut().reset($tag);
        });
    };
);
