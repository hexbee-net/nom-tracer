// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "trace-color")]
use colored::Colorize;
use std::fmt::{Display, Formatter};

/// Represents the type of a trace event.
///
/// This enum captures the different states of a parsing operation:
/// - Open: Beginning of a parsing operation
/// - CloseOk: Successful completion of a parsing operation
/// - CloseError: Parser encountered a recoverable error
/// - CloseFailure: Parser encountered an unrecoverable error
/// - CloseIncomplete: Parser needs more input to complete
#[cfg(feature = "trace")]
#[derive(Clone, Debug)]
pub enum TraceEventType {
    Open,
    CloseOk(String),
    CloseError(String),
    CloseFailure(String),
    CloseIncomplete(nom::Needed),
}

/// Represents a single trace event.
///
/// Each event includes:
/// - The nesting level at which it occurred
/// - The input string at that point
/// - The location (usually a function or parser name)
/// - The type of event (open, close with result)
#[cfg(feature = "trace")]
#[derive(Clone)]
pub struct TraceEvent {
    pub level: usize,
    pub context: Option<&'static str>,
    pub input: String,
    pub location: &'static str,
    pub event: TraceEventType,
}

#[cfg(feature = "trace")]
impl Display for TraceEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        #[cfg(feature = "trace-color")]
        {
            let indent = "| ".repeat(self.level).white();
            match &self.event {
                TraceEventType::Open => {
                    if let Some(context) = self.context {
                        writeln!(
                            f,
                            "{}{}[{}](\"{}\")",
                            indent,
                            self.location,
                            context.on_cyan(),
                            self.input.on_bright_blue(),
                        )
                    } else {
                        writeln!(
                            f,
                            "{}{}({})",
                            indent,
                            self.location,
                            self.input.on_bright_blue()
                        )
                    }
                }
                TraceEventType::CloseOk(result) => {
                    if let Some(context) = self.context {
                        writeln!(
                            f,
                            "{}[{}]",
                            format!(
                                "{}{}(\"{}\") -> Ok({})",
                                indent, self.location, self.input, result
                            )
                            .green(),
                            context.on_cyan()
                        )
                    } else {
                        writeln!(
                            f,
                            "{}",
                            format!(
                                "{}{}(\"{}\") -> Ok({})",
                                indent, self.location, self.input, result
                            )
                            .green()
                        )
                    }
                }
                TraceEventType::CloseError(e) => {
                    if let Some(context) = self.context {
                        writeln!(
                            f,
                            "{}[{}]",
                            format!(
                                "{}{}(\"{}\") -> Error({})",
                                indent, self.location, self.input, e
                            )
                            .red(),
                            context.on_cyan()
                        )
                    } else {
                        writeln!(
                            f,
                            "{}",
                            format!(
                                "{}{}(\"{}\") -> Error({})",
                                indent, self.location, self.input, e
                            )
                            .red()
                        )
                    }
                }
                TraceEventType::CloseFailure(e) => {
                    if let Some(context) = self.context {
                        writeln!(
                            f,
                            "{}[{}]",
                            format!(
                                "{}{}(\"{}\") -> Failure({})",
                                indent, self.location, self.input, e
                            )
                            .magenta(),
                            context.on_cyan()
                        )
                    } else {
                        writeln!(
                            f,
                            "{}",
                            format!(
                                "{}{}(\"{}\") -> Error({})",
                                indent, self.location, self.input, e
                            )
                            .magenta()
                        )
                    }
                }
                TraceEventType::CloseIncomplete(i) => {
                    if let Some(context) = self.context {
                        writeln!(
                            f,
                            "{}[{}]",
                            format!(
                                "{}{}(\"{}\") -> Error({:?})",
                                indent, self.location, self.input, i
                            )
                            .yellow(),
                            context.on_cyan()
                        )
                    } else {
                        writeln!(
                            f,
                            "{}",
                            format!(
                                "{}{}(\"{}\") -> Error({:?})",
                                indent, self.location, self.input, i
                            )
                            .yellow()
                        )
                    }
                }
            }
        }

        #[cfg(not(feature = "trace-color"))]
        {
            let indent = "| ".repeat(self.level);

            match &self.event {
                TraceEventType::Open => {
                    if let Some(context) = self.context {
                        writeln!(
                            f,
                            "{}{}[{}](\"{}\")",
                            indent, self.location, context, self.input
                        )
                    } else {
                        writeln!(f, "{}{}(\"{}\")", indent, self.location, self.input)
                    }
                }
                TraceEventType::CloseOk(result) => {
                    if let Some(context) = self.context {
                        writeln!(
                            f,
                            "{}{}(\"{}\") -> Ok({})[{}]",
                            indent, self.location, self.input, result, context
                        )
                    } else {
                        writeln!(
                            f,
                            "{}{}(\"{}\") -> Ok({})",
                            indent, self.location, self.input, result
                        )
                    }
                }
                TraceEventType::CloseError(e) => {
                    if let Some(context) = self.context {
                        writeln!(
                            f,
                            "{}{}(\"{}\") -> Error({})[{}]",
                            indent, self.location, self.input, e, context
                        )
                    } else {
                        writeln!(
                            f,
                            "{}{}(\"{}\") -> Error({})",
                            indent, self.location, self.input, e
                        )
                    }
                }
                TraceEventType::CloseFailure(e) => {
                    if let Some(context) = self.context {
                        writeln!(
                            f,
                            "{}{}(\"{}\") -> Failure({})[{}]",
                            indent, self.location, self.input, e, context
                        )
                    } else {
                        writeln!(
                            f,
                            "{}{}(\"{}\") -> Failure({})",
                            indent, self.location, self.input, e
                        )
                    }
                }
                TraceEventType::CloseIncomplete(i) => {
                    if let Some(context) = self.context {
                        writeln!(
                            f,
                            "{}{}(\"{}\") -> Incomplete({:?})[{}]",
                            indent, self.location, self.input, i, context
                        )
                    } else {
                        writeln!(
                            f,
                            "{}{}(\"{}\") -> Incomplete({:?})",
                            indent, self.location, self.input, i
                        )
                    }
                }
            }
        }
    }
}
