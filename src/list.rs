// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

use {
    crate::{traces::Trace, DEFAULT_TAG},
    nom::IResult,
    std::{collections::HashMap, fmt::Debug},
};

/// A collection of traces, each associated with a tag.
///
/// The tag system allows for multiple independent traces to be maintained simultaneously.
/// Each tag corresponds to a separate `Trace` instance, allowing for organization and
/// separation of trace events based on different criteria (e.g., parser type, subsystem, etc.).
#[derive(Default)]
pub struct TraceList {
    pub traces: HashMap<&'static str, Trace>,
}

impl TraceList {
    /// Creates a new [TraceList] with a default trace.
    ///
    /// The default trace is associated with the `DEFAULT_TAG`.
    pub fn new() -> Self {
        let mut traces = HashMap::new();
        traces.insert(DEFAULT_TAG, Trace::default());

        TraceList { traces }
    }

    /// Resets the trace for the given tag.
    ///
    /// This clears all events and resets the nesting level for the specified trace.
    /// If the trace doesn't exist, a new one is created and inserted.
    pub fn reset(&mut self, tag: &'static str) {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.reset();
    }

    /// Returns the trace for the given tag as a string, if it exists.
    pub fn get_trace(&self, tag: &'static str) -> Option<String> {
        self.traces.get(tag).map(|t| t.to_string())
    }

    /// Activates the trace for the given tag.
    ///
    /// Activated traces will record events.
    /// If the trace doesn't exist, a new one is created and activated.
    pub fn activate(&mut self, tag: &'static str) {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.active = true;
    }

    /// Deactivates the trace for the given tag.
    ///
    /// Deactivated traces will not record events, but will retain previously recorded events.
    /// If the trace doesn't exist, a new one is created (but left inactive).
    pub fn deactivate(&mut self, tag: &'static str) {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.active = false;
    }

    /// Activates real-time printing of trace events for the given tag.
    ///
    /// When activated, trace events will be printed as they occur.
    /// If the trace doesn't exist, a new one is created with real-time printing enabled.
    pub fn activate_trace_print(&mut self, tag: &'static str) {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.print = true;
    }

    /// Deactivates real-time printing of trace events for the given tag.
    ///
    /// When deactivated, trace events will not be printed as they occur.
    /// If the trace doesn't exist, a new one is created with real-time printing disabled.
    pub fn deactivate_trace_print(&mut self, tag: &'static str) {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.print = false;
    }

    /// Sets the maximum nesting level for the trace of the given tag before triggering a panic.
    ///
    /// This method allows setting a limit on how deep the parsing can go before it's considered
    /// an error (e.g., to catch an infinite recursion). When the nesting level reaches or exceeds
    /// the specified level, the next `open` operation will cause a panic.
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag of the trace to set the panic level for.
    /// * `level` - An `Option<usize>` specifying the maximum allowed nesting level.
    ///   - `Some(n)`: Sets the maximum level to `n`. The parser will panic if it reaches level `n+1`.
    ///   - `None`: Removes any previously set limit, allowing unlimited nesting.
    ///
    /// # Examples
    ///
    /// ```
    /// use nom_tracer::list::TraceList;
    /// let mut trace_list = TraceList::new();
    ///
    /// // Set maximum nesting level to 5 for the default tag
    /// trace_list.panic_on_level(nom_tracer::DEFAULT_TAG, Some(5));
    ///
    /// // Remove the nesting level limit for a custom tag
    /// trace_list.panic_on_level("my_custom_tag", None);
    /// ```
    pub fn panic_on_level(&mut self, tag: &'static str, level: Option<usize>) {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.panic_on_level = level;
    }

    /// Opens a new trace event for the given tag.
    ///
    /// This increases the nesting level for the trace and records an 'open' event.
    /// The hierarchical structure of parsing is represented by these nested open/close events.
    pub fn open<I>(
        &mut self,
        tag: &'static str,
        context: Option<&'static str>,
        input: I,
        location: &'static str,
    ) where
        I: AsRef<str>,
    {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.open(context, input, location);
    }

    /// Closes the current trace event for the given tag.
    ///
    /// This decreases the nesting level for the trace and records a 'close' event,
    /// including the result of the parsing operation (success, error, etc.).
    /// The hierarchical structure is maintained by matching each 'close' with a previous 'open'.
    pub fn close<I, O: Debug, E: Debug>(
        &mut self,
        tag: &'static str,
        context: Option<&'static str>,
        input: I,
        location: &'static str,
        result: &IResult<I, O, E>,
    ) where
        I: AsRef<str>,
    {
        let t = self.traces.entry(tag).or_insert(Trace::default());
        t.close(context, input, location, result);
    }
}

#[cfg(test)]
mod tests {
    use {super::*, nom::bytes::complete::tag};

    #[test]
    fn test_new() {
        let trace_list = TraceList::new();
        assert!(trace_list.traces.contains_key(DEFAULT_TAG));
    }

    #[test]
    fn test_reset() {
        let mut trace_list = TraceList::new();
        trace_list.open(DEFAULT_TAG, None, "input", "location");
        trace_list.reset(DEFAULT_TAG);
        assert_eq!(trace_list.traces[DEFAULT_TAG].events.len(), 0);
        assert_eq!(trace_list.traces[DEFAULT_TAG].level, 0);
    }

    #[test]
    fn test_get_trace() {
        let mut trace_list = TraceList::new();
        trace_list.open(DEFAULT_TAG, None, "input", "location");
        trace_list.close::<_, _, nom::error::VerboseError<&str>>(
            DEFAULT_TAG,
            None,
            "input",
            "location",
            &Ok(("", "result")),
        );

        let trace = trace_list.get_trace(DEFAULT_TAG);
        assert!(trace.is_some());
        let trace_str = trace.unwrap();
        assert!(trace_str.contains("location(\"input\")"));
        assert!(trace_str.contains("-> Ok(\"result\")"));
    }

    #[test]
    fn test_get_trace_nonexistent_tag() {
        let trace_list = TraceList::new();
        let trace = trace_list.get_trace("nonexistent_tag");
        assert!(trace.is_none());
    }

    #[test]
    fn test_activate_deactivate() {
        let mut trace_list = TraceList::new();
        trace_list.deactivate(DEFAULT_TAG);
        assert!(!trace_list.traces[DEFAULT_TAG].active);
        trace_list.activate(DEFAULT_TAG);
        assert!(trace_list.traces[DEFAULT_TAG].active);
    }

    #[test]
    fn test_activate_trace_print() {
        let mut trace_list = TraceList::new();

        // Initially, print should be false
        assert!(!trace_list.traces[DEFAULT_TAG].print);

        trace_list.activate_trace_print(DEFAULT_TAG);
        assert!(trace_list.traces[DEFAULT_TAG].print);

        // Test with a custom tag
        let custom_tag = "custom_tag";
        trace_list.activate_trace_print(custom_tag);
        assert!(trace_list.traces[custom_tag].print);
    }

    #[test]
    fn test_deactivate_trace_print() {
        let mut trace_list = TraceList::new();

        // Activate print first
        trace_list.activate_trace_print(DEFAULT_TAG);
        assert!(trace_list.traces[DEFAULT_TAG].print);

        // Now deactivate
        trace_list.deactivate_trace_print(DEFAULT_TAG);
        assert!(!trace_list.traces[DEFAULT_TAG].print);

        // Test with a custom tag
        let custom_tag = "custom_tag";
        trace_list.activate_trace_print(custom_tag);
        trace_list.deactivate_trace_print(custom_tag);
        assert!(!trace_list.traces[custom_tag].print);
    }

    #[test]
    fn test_panic_on_level_set() {
        let mut trace_list = TraceList::new();
        trace_list.panic_on_level(DEFAULT_TAG, Some(5));
        assert_eq!(trace_list.traces[DEFAULT_TAG].panic_on_level, Some(5));
    }

    #[test]
    fn test_panic_on_level_remove() {
        let mut trace_list = TraceList::new();
        trace_list.panic_on_level(DEFAULT_TAG, Some(5));
        trace_list.panic_on_level(DEFAULT_TAG, None);
        assert_eq!(trace_list.traces[DEFAULT_TAG].panic_on_level, None);
    }

    #[test]
    #[should_panic(expected = "Max level reached: 3")]
    fn test_panic_on_level_trigger() {
        let mut trace_list = TraceList::new();
        trace_list.panic_on_level(DEFAULT_TAG, Some(3));

        // Open traces until we reach the panic level
        trace_list.open(DEFAULT_TAG, None, "input1", "location1");
        trace_list.open(DEFAULT_TAG, None, "input2", "location2");
        trace_list.open(DEFAULT_TAG, None, "input3", "location3");

        // This should trigger a panic
        trace_list.open(DEFAULT_TAG, None, "input4", "location4");
    }

    #[test]
    fn test_panic_on_level_not_triggered() {
        let mut trace_list = TraceList::new();
        trace_list.panic_on_level(DEFAULT_TAG, Some(5));

        // Open traces but don't exceed the panic level
        trace_list.open(DEFAULT_TAG, None, "input1", "location1");
        trace_list.open(DEFAULT_TAG, None, "input2", "location2");
        trace_list.open(DEFAULT_TAG, None, "input3", "location3");
        trace_list.open(DEFAULT_TAG, None, "input4", "location4");
        trace_list.open(DEFAULT_TAG, None, "input5", "location5");

        // This should not panic
        assert_eq!(trace_list.traces[DEFAULT_TAG].level, 5);
    }

    #[test]
    fn test_open() {
        let mut trace_list = TraceList::new();
        trace_list.open(DEFAULT_TAG, None, "input", "location");

        let trace = &trace_list.traces[DEFAULT_TAG];
        assert_eq!(trace.level, 1);
        assert_eq!(trace.events.len(), 1);
        assert_eq!(trace.events[0].input, "input");
        assert_eq!(trace.events[0].location, "location");
    }

    #[test]
    fn test_open_with_context() {
        let mut trace_list = TraceList::new();
        trace_list.open(DEFAULT_TAG, Some("context"), "input", "location");

        let trace = &trace_list.traces[DEFAULT_TAG];
        assert_eq!(trace.events[0].context, Some("context"));
    }

    #[test]
    fn test_close() {
        let mut trace_list = TraceList::new();
        trace_list.open(DEFAULT_TAG, None, "input", "location");
        trace_list.close::<_, _, nom::error::VerboseError<&str>>(
            DEFAULT_TAG,
            None,
            "input",
            "location",
            &Ok(("", "result")),
        );

        let trace = &trace_list.traces[DEFAULT_TAG];
        assert_eq!(trace.level, 0);
        assert_eq!(trace.events.len(), 2);

        if let crate::events::TraceEventType::CloseOk(result) = &trace.events[1].event {
            assert_eq!(result, "\"result\"");
        } else {
            panic!("Expected CloseOk event");
        }
    }

    #[test]
    fn test_nested_open_close() {
        let mut trace_list = TraceList::new();
        trace_list.open(DEFAULT_TAG, None, "input1", "location1");
        trace_list.open(DEFAULT_TAG, None, "input2", "location2");
        trace_list.close::<_, _, nom::error::VerboseError<&str>>(
            DEFAULT_TAG,
            None,
            "input2",
            "location2",
            &Ok(("", "result2")),
        );
        trace_list.close::<_, _, nom::error::VerboseError<&str>>(
            DEFAULT_TAG,
            None,
            "input1",
            "location1",
            &Ok(("", "result1")),
        );

        let trace = &trace_list.traces[DEFAULT_TAG];
        assert_eq!(trace.level, 0);
        assert_eq!(trace.events.len(), 4);
        assert_eq!(trace.events[0].level, 0);
        assert_eq!(trace.events[1].level, 1);
        assert_eq!(trace.events[2].level, 1);
        assert_eq!(trace.events[3].level, 0);
    }

    #[test]
    fn test_open_close_with_nom_parser() {
        let mut trace_list = TraceList::new();
        let input = "hello world";
        let parser = tag::<_, _, ()>("hello");

        trace_list.open(DEFAULT_TAG, None, input, "hello_parser");
        let result = parser(input);
        trace_list.close(DEFAULT_TAG, None, input, "hello_parser", &result);

        let trace = trace_list.get_trace(DEFAULT_TAG).unwrap();
        assert!(trace.contains("hello_parser(\"hello world\")"));
        assert!(trace.contains("-> Ok(\"hello\")"));
    }
}
