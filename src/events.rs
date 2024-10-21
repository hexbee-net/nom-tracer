// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "trace-color")]
use crate::ansi;
use std::fmt::{Display, Formatter};

/// Represents the type of a trace event.
///
/// This enum is used to categorize different stages or outcomes of a parsing operation.
#[derive(Clone, Debug)]
pub enum TraceEventType {
    /// Indicates the start of a parsing operation.
    Open,
    /// Indicates a successful parsing operation, containing the result.
    CloseOk(String),
    /// Indicates a parsing error, containing the error message.
    CloseError(String),
    /// Indicates a parsing failure, containing the failure message.
    CloseFailure(String),
    /// Indicates an incomplete parse, containing the additional data needed.
    CloseIncomplete(nom::Needed),
}

/// Represents a single trace event in the parsing process.
///
/// This struct contains all the information about a specific event that occurred
/// during parsing, including its type, location, and context.
#[derive(Clone)]
pub struct TraceEvent {
    /// The nesting level of this event in the parsing tree.
    pub level: usize,
    /// The location (usually function name) where this event occurred.
    pub location: &'static str,
    /// Optional context information for this event.
    pub context: Option<&'static str>,
    /// The input string being parsed at this point.
    pub input: String,
    /// The type of this trace event.
    pub event: TraceEventType,
}

impl Display for TraceEvent {
    /// Formats the TraceEvent for display.
    ///
    /// This implementation provides a detailed, possibly colored representation of the trace event,
    /// including indentation to represent nesting level, and different formatting for different
    /// event types.
    ///
    /// The exact format depends on whether the `trace-color` feature is enabled.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let indent = "| ".repeat(self.level);

        #[allow(unused_mut)]
        let mut input = self.input.clone();

        #[allow(unused_mut)]
        let mut ctx = if let Some(context) = self.context {
            format!("[{}]", context)
        } else {
            "".to_string()
        };

        #[cfg(feature = "trace-color")]
        {
            ctx = format!("{}{}", ansi::BG_BLUE, ctx);
        }

        #[cfg(feature = "trace-color")]
        {
            let content = match &self.event {
                TraceEventType::Open => {
                    let input = format!(
                        "{}{}{}",
                        ansi::TEXT_INVERSE,
                        input,
                        ansi::TEXT_INVERSE_RESET
                    );
                    format!(
                        "{}{}{}(\"{}\")",
                        ansi::TEXT_UNDERLINE,
                        self.location,
                        ansi::TEXT_UNDERLINE_RESET,
                        input
                    )
                }
                TraceEventType::CloseOk(result) => format!(
                    "{}-> Ok({}{}{})",
                    ansi::FG_GREEN,
                    ansi::TEXT_INVERSE,
                    result,
                    ansi::TEXT_INVERSE_RESET
                ),
                TraceEventType::CloseError(e) => format!(
                    "{}-> Error({}{}{})",
                    ansi::FG_RED,
                    ansi::TEXT_INVERSE,
                    e,
                    ansi::TEXT_INVERSE_RESET
                ),
                TraceEventType::CloseFailure(e) => format!(
                    "{}-> Failure({}{}{})",
                    ansi::FG_MAGENTA,
                    ansi::TEXT_INVERSE,
                    e,
                    ansi::TEXT_INVERSE_RESET
                ),
                TraceEventType::CloseIncomplete(i) => format!(
                    "{}-> Incomplete({}{:?}{})",
                    ansi::FG_YELLOW,
                    ansi::TEXT_INVERSE,
                    i,
                    ansi::TEXT_INVERSE_RESET
                ),
            };

            writeln!(
                f,
                "{}{}{}{}{}",
                indent,
                content,
                ansi::FG_BLACK,
                ctx,
                ansi::RESET
            )
        }

        #[cfg(not(feature = "trace-color"))]
        {
            let content = match &self.event {
                TraceEventType::Open => format!("{}(\"{}\")", self.location, input),
                TraceEventType::CloseOk(result) => format!("-> Ok({})", result),
                TraceEventType::CloseError(e) => format!("-> Error({})", e),
                TraceEventType::CloseFailure(e) => format!("-> Failure({})", e),
                TraceEventType::CloseIncomplete(i) => format!("-> Incomplete({:?})", i),
            };

            writeln!(f, "{}{}{}", indent, content, ctx)
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        crate::events::{TraceEvent, TraceEventType},
        std::num::NonZero,
    };

    #[test]
    fn test_display_open() {
        println!(
            "{:#}",
            TraceEvent {
                level: 2,
                location: "test_location",
                context: Some("test_context"),
                input: "test_input".to_string(),
                event: TraceEventType::Open,
            }
        );
    }

    #[test]
    fn test_display_close_ok() {
        println!(
            "{:#}",
            TraceEvent {
                level: 2,
                location: "test_location",
                context: Some("test_context"),
                input: "test_input".to_string(),
                event: TraceEventType::CloseOk("ok".to_string()),
            }
        );
    }

    #[test]
    fn test_display_close_error() {
        println!(
            "{:#}",
            TraceEvent {
                level: 2,
                location: "test_location",
                context: Some("test_context"),
                input: "test_input".to_string(),
                event: TraceEventType::CloseError("error".to_string()),
            }
        );
    }

    #[test]
    fn test_display_close_failure() {
        println!(
            "{:#}",
            TraceEvent {
                level: 2,
                location: "test_location",
                context: Some("test_context"),
                input: "test_input".to_string(),
                event: TraceEventType::CloseFailure("failure".to_string()),
            }
        );
    }

    #[test]
    fn test_display_close_incomplete() {
        println!(
            "{:#}",
            TraceEvent {
                level: 2,
                location: "test_location",
                context: Some("test_context"),
                input: "test_input".to_string(),
                event: TraceEventType::CloseIncomplete(nom::Needed::Size(NonZero::new(5).unwrap())),
            }
        );
    }
}
