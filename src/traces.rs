// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

use {
    crate::events::{TraceEvent, TraceEventType},
    nom::IResult,
    std::fmt::{Debug, Display, Formatter},
};

/// The main structure holding trace events.
///
/// A [Trace] maintains a list of events, a current nesting level, and an active state.
/// The nesting level represents the depth of the current parsing operation in the overall
/// structure of the parser combinators.
#[cfg(feature = "trace")]
pub struct Trace {
    pub events: Vec<TraceEvent>,
    pub level: usize,
    pub active: bool,
}

#[cfg(feature = "trace")]
impl Trace {
    /// Resets the trace, clearing all events and setting the level to 0.
    pub fn reset(&mut self) {
        self.events.clear();
        self.level = 0;
    }

    /// Opens a new trace event.
    ///
    /// This increases the nesting level and adds an 'open' event to the trace.
    /// The hierarchical structure of parsing is represented by these nested open/close events.
    pub fn open<I: AsRef<str>>(
        &mut self,
        context: Option<&'static str>,
        input: I,
        location: &'static str,
    ) {
        if self.active {
            let event = TraceEvent {
                level: self.level,
                context,
                input: String::from(input.as_ref()),
                location,
                event: TraceEventType::Open,
            };

            #[cfg(all(feature = "trace", feature = "trace-print"))]
            {
                print_colored(format!("{}", event));
            }

            self.events.push(event);

            self.level += 1;
        }
    }

    /// Closes the current trace event.
    ///
    /// This decreases the nesting level and adds a 'close' event to the trace,
    /// including the result of the parsing operation. The type of 'close' event
    /// ([Ok](TraceEventType::CloseOk), [Error](TraceEventType::CloseError),
    /// [Failure](TraceEventType::CloseFailure), [Incomplete](TraceEventType::CloseIncomplete)
    /// corresponds to the result of the parse operation.
    pub fn close<I: AsRef<str>, O: Debug, E: Debug>(
        &mut self,
        context: Option<&'static str>,
        input: I,
        location: &'static str,
        result: &IResult<I, O, E>,
    ) {
        if self.active {
            self.level -= 1;
            let event_type = match result {
                Ok((_, o)) => TraceEventType::CloseOk(format!("{:?}", o)),
                Err(nom::Err::Error(e)) => TraceEventType::CloseError(format!("{:?}", e)),
                Err(nom::Err::Failure(e)) => TraceEventType::CloseFailure(format!("{:?}", e)),
                Err(nom::Err::Incomplete(i)) => TraceEventType::CloseIncomplete(*i),
            };

            let event = TraceEvent {
                level: self.level,
                context,
                input: String::from(input.as_ref()),
                location,
                event: event_type,
            };

            #[cfg(all(feature = "trace", feature = "trace-print"))]
            {
                print_colored(format!("{}", event));
            }

            self.events.push(event);
        }
    }
}

#[cfg(feature = "trace")]
impl Display for Trace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for event in self.events.iter() {
            event.fmt(f)?;
        }
        Ok(())
    }
}
