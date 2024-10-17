// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

use {
    crate::{traces::Trace, DEFAULT_TAG},
    nom::IResult,
    std::{collections::HashMap, fmt::Debug},
};

/// Manages multiple traces, each associated with a unique tag.
///
/// This struct allows for organizing and managing multiple trace instances,
/// each identified by a static string tag. It provides methods for manipulating
/// traces, such as activating, deactivating, and resetting them.
#[derive(Default)]
pub struct TraceTags {
    pub traces: HashMap<&'static str, Trace>,
}

impl TraceTags {
    /// Creates a new `TraceTags` instance with a default trace.
    ///
    /// The default trace is associated with the [DEFAULT_TAG].
    pub fn new() -> Self {
        let mut traces = HashMap::new();
        traces.insert(DEFAULT_TAG, Trace::default());

        TraceTags { traces }
    }

    /// Resets the trace associated with the given tag.
    ///
    /// If the tag doesn't exist, a new trace is created and then reset.
    pub fn clear(&mut self, tag: &'static str) {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.clear();
    }

    /// Retrieves the trace associated with the given tag as a string.
    ///
    /// Returns `None` if the tag doesn't exist.
    pub fn get_trace(&self, tag: &'static str) -> Option<String> {
        self.traces.get(tag).map(|t| t.to_string())
    }

    /// Activates the trace associated with the given tag.
    ///
    /// If the tag doesn't exist, a new trace is created and activated.
    pub fn activate(&mut self, tag: &'static str) {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.active = true;
    }

    /// Deactivates the trace associated with the given tag.
    ///
    /// If the tag doesn't exist, a new trace is created (but remains inactive).
    pub fn deactivate(&mut self, tag: &'static str) {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.active = false;
    }

    /// Activates real-time printing for the trace associated with the given tag.
    ///
    /// This method is only available when the `trace-print` feature is enabled.
    #[cfg(feature = "trace-print")]
    pub fn activate_trace_print(&mut self, tag: &'static str) {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.print = true;
    }

    /// Deactivates real-time printing for the trace associated with the given tag.
    ///
    /// This method is only available when the `trace-print` feature is enabled.
    #[cfg(feature = "trace-print")]
    pub fn deactivate_trace_print(&mut self, tag: &'static str) {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.print = false;
    }

    /// Sets the maximum nesting level for the trace associated with the given tag.
    ///
    /// When the nesting level exceeds this value, the parser will panic.
    /// This method is only available when the `trace-max-level` feature is enabled.
    #[cfg(feature = "trace-max-level")]
    pub fn panic_on_level(&mut self, tag: &'static str, level: Option<usize>) {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.panic_on_level = level;
    }

    /// Records the opening of a parser in the trace associated with the given tag.
    pub fn open<I>(
        &mut self,
        tag: &'static str,
        context: Option<&'static str>,
        input: I,
        location: &'static str,
        silent: bool,
    ) where
        I: AsRef<str>,
    {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.open(context, input, location, silent);
    }

    /// Records the closing of a parser in the trace associated with the given tag.
    pub fn close<I, O: Debug, E: Debug>(
        &mut self,
        tag: &'static str,
        context: Option<&'static str>,
        input: I,
        location: &'static str,
        result: &IResult<I, O, E>,
        silent: bool,
    ) where
        I: AsRef<str>,
    {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.close(context, input, location, result, silent);
    }

    /// Returns the current nesting level for the trace associated with the given tag.
    ///
    /// If the tag doesn't exist, returns 0.
    pub fn level_for_tag(&self, tag: &'static str) -> usize {
        self.traces.get(tag).map(|t| t.level).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use {super::*, nom::bytes::complete::tag};

    #[test]
    fn test_trace_tags_new() {
        let trace_tags = TraceTags::new();
        assert!(trace_tags.traces.contains_key(DEFAULT_TAG));
    }

    #[test]
    fn test_reset() {
        let mut trace_tags = TraceTags::new();
        trace_tags.open(DEFAULT_TAG, None, "input", "location", false);
        trace_tags.clear(DEFAULT_TAG);
        assert_eq!(trace_tags.traces[DEFAULT_TAG].events.len(), 0);
        assert_eq!(trace_tags.traces[DEFAULT_TAG].level, 0);
    }

    #[test]
    fn test_get_trace() {
        let mut trace_tags = TraceTags::new();
        trace_tags.open(DEFAULT_TAG, None, "input", "location", false);
        trace_tags.close::<_, _, nom::error::VerboseError<&str>>(
            DEFAULT_TAG,
            None,
            "input",
            "location",
            &Ok(("", "result")),
            false,
        );

        let trace = trace_tags.get_trace(DEFAULT_TAG);
        assert!(trace.is_some());
        let trace_str = trace.unwrap();
        assert!(trace_str.contains("location"));
        assert!(trace_str.contains("input"));
        assert!(trace_str.contains("-> Ok"));
        assert!(trace_str.contains("result"));
    }

    #[test]
    fn test_get_trace_nonexistent_tag() {
        let trace_tags = TraceTags::new();
        let trace = trace_tags.get_trace("nonexistent_tag");
        assert!(trace.is_none());
    }

    #[test]
    fn test_activate_deactivate() {
        let mut trace_tags = TraceTags::new();
        trace_tags.deactivate(DEFAULT_TAG);
        assert!(!trace_tags.traces[DEFAULT_TAG].active);
        trace_tags.activate(DEFAULT_TAG);
        assert!(trace_tags.traces[DEFAULT_TAG].active);
    }

    #[test]
    fn test_open_close() {
        let mut trace_tags = TraceTags::new();
        trace_tags.open(DEFAULT_TAG, None, "input", "location", false);
        trace_tags.close::<_, _, nom::error::VerboseError<&str>>(
            DEFAULT_TAG,
            None,
            "input",
            "location",
            &Ok(("", "result")),
            false,
        );

        let trace = &trace_tags.traces[DEFAULT_TAG];
        assert_eq!(trace.level, 0);
        assert_eq!(trace.events.len(), 2);
    }

    #[test]
    fn test_level_for_tag() {
        let mut trace_tags = TraceTags::new();
        trace_tags.open(DEFAULT_TAG, None, "input1", "location1", false);
        trace_tags.open(DEFAULT_TAG, None, "input2", "location2", false);
        assert_eq!(trace_tags.level_for_tag(DEFAULT_TAG), 2);
        assert_eq!(trace_tags.level_for_tag("nonexistent_tag"), 0);
    }

    #[test]
    fn test_open_close_with_nom_parser() {
        let mut trace_tags = TraceTags::new();
        let input = "hello world";
        let parser = tag::<_, _, ()>("hello");

        trace_tags.open(DEFAULT_TAG, None, input, "hello_parser", false);
        let result = parser(input);
        trace_tags.close(DEFAULT_TAG, None, input, "hello_parser", &result, false);

        let trace = trace_tags.get_trace(DEFAULT_TAG).unwrap();
        assert!(trace.contains("hello_parser"));
        assert!(trace.contains("hello world"));
        assert!(trace.contains("-> Ok"));
    }

    #[cfg(feature = "trace-print")]
    mod print_tests {
        use super::*;

        #[test]
        fn test_activate_deactivate_trace_print() {
            let mut trace_tags = TraceTags::new();

            trace_tags.activate_trace_print(DEFAULT_TAG);
            assert!(trace_tags.traces[DEFAULT_TAG].print);

            trace_tags.deactivate_trace_print(DEFAULT_TAG);
            assert!(!trace_tags.traces[DEFAULT_TAG].print);

            // Test with a custom tag
            let custom_tag = "custom_tag";
            trace_tags.activate_trace_print(custom_tag);
            assert!(trace_tags.traces[custom_tag].print);
            trace_tags.deactivate_trace_print(custom_tag);
            assert!(!trace_tags.traces[custom_tag].print);
        }
    }

    #[cfg(feature = "trace-max-level")]
    mod max_level_tests {
        use super::*;

        #[test]
        fn test_panic_on_level() {
            let mut trace_tags = TraceTags::new();
            trace_tags.panic_on_level(DEFAULT_TAG, Some(5));
            assert_eq!(trace_tags.traces[DEFAULT_TAG].panic_on_level, Some(5));

            trace_tags.panic_on_level(DEFAULT_TAG, None);
            assert_eq!(trace_tags.traces[DEFAULT_TAG].panic_on_level, None);
        }

        #[test]
        #[should_panic(expected = "Max level reached: 3")]
        fn test_panic_on_level_trigger() {
            let mut trace_tags = TraceTags::new();
            trace_tags.panic_on_level(DEFAULT_TAG, Some(3));

            // Open traces until we reach the panic level
            trace_tags.open(DEFAULT_TAG, None, "input1", "location1", false);
            trace_tags.open(DEFAULT_TAG, None, "input2", "location2", false);
            trace_tags.open(DEFAULT_TAG, None, "input3", "location3", false);

            // This should trigger a panic
            trace_tags.open(DEFAULT_TAG, None, "input4", "location4", false);
        }

        #[test]
        fn test_panic_on_level_not_triggered() {
            let mut trace_tags = TraceTags::new();
            trace_tags.panic_on_level(DEFAULT_TAG, Some(5));

            // Open traces but don't exceed the panic level
            trace_tags.open(DEFAULT_TAG, None, "input1", "location1", false);
            trace_tags.open(DEFAULT_TAG, None, "input2", "location2", false);
            trace_tags.open(DEFAULT_TAG, None, "input3", "location3", false);
            trace_tags.open(DEFAULT_TAG, None, "input4", "location4", false);
            trace_tags.open(DEFAULT_TAG, None, "input5", "location5", false);

            // This should not panic
            assert_eq!(trace_tags.traces[DEFAULT_TAG].level, 5);
        }
    }
}
