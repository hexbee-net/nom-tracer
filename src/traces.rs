// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "trace-print")]
use crate::print;
use {
    crate::events::{TraceEvent, TraceEventType},
    nom::IResult,
    std::fmt::{Debug, Display, Formatter},
};

/// Represents a single trace in the parsing process.
///
/// A `Trace` keeps track of parsing events, maintains the current nesting level,
/// and can be configured for various tracing behaviors.
pub struct Trace {
    /// The collection of trace events.
    pub events: Vec<TraceEvent>,
    /// The current nesting level of the trace.
    pub level: usize,
    /// Whether the trace is currently active and recording events.
    pub active: bool,
    /// Whether to print trace events in real-time.
    #[cfg(feature = "trace-print")]
    pub print: bool,
    /// The maximum nesting level before panicking, if set.
    #[cfg(feature = "trace-max-level")]
    pub panic_on_level: Option<usize>,
}

impl Default for Trace {
    fn default() -> Self {
        Self {
            events: Vec::new(),
            level: 0,
            active: true,
            #[cfg(feature = "trace-print")]
            print: false,
            #[cfg(feature = "trace-max-level")]
            panic_on_level: None,
        }
    }
}

impl Trace {
    /// Clears all recorded events and resets the nesting level to 0.
    pub fn clear(&mut self) {
        self.events.clear();
        self.level = 0;
    }

    /// Records the opening of a parser in the trace.
    ///
    /// # Arguments
    ///
    /// * `context` - Optional context information for the event.
    /// * `input` - The input being parsed.
    /// * `location` - The location (usually function name) where this event occurred.
    /// * `silent` - Whether to suppress real-time printing of this event.
    ///
    /// # Returns
    ///
    /// The new nesting level after recording this event.
    pub fn open<I: AsRef<str>>(
        &mut self,
        context: Option<&'static str>,
        input: I,
        location: &'static str,
        #[cfg(feature = "trace-print")] silent: bool,
        #[cfg(not(feature = "trace-print"))] _silent: bool,
    ) -> usize {
        if self.active {
            #[cfg(feature = "trace-max-level")]
            if let Some(level) = self.panic_on_level {
                if self.level >= level {
                    panic!("Max level reached: {}", level);
                }
            }

            let event = TraceEvent {
                level: self.level,
                location,
                context,
                input: String::from(input.as_ref()),
                event: TraceEventType::Open,
            };

            #[cfg(feature = "trace-print")]
            if self.print && !silent {
                print(format!("{}", event));
            }

            self.events.push(event);
            self.level += 1;
        }

        self.level
    }

    /// Records the closing of a parser in the trace.
    ///
    /// # Arguments
    ///
    /// * `context` - Optional context information for the event.
    /// * `input` - The input being parsed.
    /// * `location` - The location (usually function name) where this event occurred.
    /// * `result` - The result of the parsing operation.
    /// * `silent` - Whether to suppress real-time printing of this event.
    ///
    /// # Returns
    ///
    /// The new nesting level after recording this event.
    pub fn close<I: AsRef<str>, O: Debug, E: Debug>(
        &mut self,
        context: Option<&'static str>,
        input: I,
        location: &'static str,
        result: &IResult<I, O, E>,
        #[cfg(feature = "trace-print")] silent: bool,
        #[cfg(not(feature = "trace-print"))] _silent: bool,
    ) -> usize {
        if self.active {
            if self.level == 0 {
                panic!("Cannot close at level 0: location=\"{}\"", location);
            }
            self.level -= 1;

            let event_type = match result {
                Ok((_, o)) => TraceEventType::CloseOk(format!("{:?}", o)),
                Err(nom::Err::Error(e)) => TraceEventType::CloseError(format!("{:?}", e)),
                Err(nom::Err::Failure(e)) => TraceEventType::CloseFailure(format!("{:?}", e)),
                Err(nom::Err::Incomplete(i)) => TraceEventType::CloseIncomplete(*i),
            };

            let event = TraceEvent {
                level: self.level,
                location,
                context,
                input: String::from(input.as_ref()),
                event: event_type,
            };

            #[cfg(feature = "trace-print")]
            if self.print && !silent {
                print(format!("{}", event));
            }

            self.events.push(event);
        }

        self.level
    }

    /// Sets the current nesting level of the trace.
    ///
    /// # Arguments
    ///
    /// * `level` - The new nesting level to set.
    pub fn set_level(&mut self, level: usize) {
        self.level = level;
    }
}

impl Display for Trace {
    /// Formats the entire trace for display.
    ///
    /// This will format and display all events in the trace sequentially.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for event in self.events.iter() {
            event.fmt(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_default() {
        let trace = Trace::default();
        assert!(trace.events.is_empty());
        assert_eq!(trace.level, 0);
        assert!(trace.active);

        #[cfg(feature = "trace-print")]
        assert!(!trace.print);

        #[cfg(feature = "trace-max-level")]
        assert_eq!(trace.panic_on_level, None);
    }

    #[test]
    fn test_trace_clear() {
        let mut trace = Trace::default();
        trace.events.push(TraceEvent {
            level: 0,
            location: "test",
            context: None,
            input: "input".to_string(),
            event: TraceEventType::Open,
        });
        trace.level = 1;

        trace.clear();
        assert!(trace.events.is_empty());
        assert_eq!(trace.level, 0);
    }

    #[test]
    fn test_trace_open() {
        let mut trace = Trace::default();
        let level = trace.open(Some("context"), "input", "location", false);
        assert_eq!(level, 1);
        assert_eq!(trace.events.len(), 1);
        assert!(matches!(trace.events[0].event, TraceEventType::Open));
    }

    #[test]
    fn test_trace_close_ok() {
        let mut trace = Trace::default();
        trace.open(None, "input", "location", false);
        trace.close::<_, _, nom::error::VerboseError<&str>>(
            None,
            "input",
            "location",
            &Ok(("", "result")),
            false,
        );
        assert_eq!(trace.events.len(), 2);
        assert!(matches!(trace.events[1].event, TraceEventType::CloseOk(_)));
    }

    #[test]
    fn test_trace_set_level() {
        let mut trace = Trace::default();
        trace.set_level(5);
        assert_eq!(trace.level, 5);
    }

    #[cfg(feature = "trace-max-level")]
    mod max_level_tests {
        use super::*;

        #[test]
        #[should_panic(expected = "Max level reached: 2")]
        fn test_trace_max_level_panic() {
            let mut trace = Trace {
                panic_on_level: Some(2),
                ..Default::default()
            };
            trace.open(None, "input", "location", false);
            trace.open(None, "input", "location", false);
            trace.open(None, "input", "location", false); // This should panic
        }

        #[test]
        fn test_trace_max_level_no_panic() {
            let mut trace = Trace {
                panic_on_level: Some(3),
                ..Default::default()
            };
            trace.open(None, "input", "location", false);
            trace.open(None, "input", "location", false);
            trace.open(None, "input", "location", false);
            assert_eq!(trace.level, 3);
        }
    }
}
