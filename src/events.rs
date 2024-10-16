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
#[derive(Clone)]
pub struct TraceEvent {
    pub level: usize,
    pub context: Option<&'static str>,
    pub input: String,
    pub location: &'static str,
    pub event: TraceEventType,
}

impl Display for TraceEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let location = self.location;
        #[allow(unused_mut)]
        let mut input = self.input.clone();
        let indent = "| ".repeat(self.level);

        #[cfg(feature = "trace-color")]
        {
            input = input.on_bright_blue().to_string();
        }

        let alt = if f.alternate() {
            format!("{}(\"{}\")", location, input)
        } else {
            "".to_string()
        };

        #[allow(unused_mut)]
        let mut ctx = if let Some(context) = self.context {
            format!("[{}]", context)
        } else {
            "".to_string()
        };

        #[cfg(feature = "trace-color")]
        {
            ctx = ctx.on_cyan().to_string();
        }

        #[allow(unused_mut)]
        let mut content = match &self.event {
            TraceEventType::Open => {
                format!("{}(\"{}\")", self.location, input,)
            }
            TraceEventType::CloseOk(result) => format!("{} -> Ok({})", alt, result),
            TraceEventType::CloseError(e) => format!("{} -> Error({})", alt, e),
            TraceEventType::CloseFailure(e) => format!("{} -> Failure({})", alt, e),
            TraceEventType::CloseIncomplete(i) => format!("{} -> Error({:?})", alt, i),
        };

        #[cfg(feature = "trace-color")]
        {
            content = match &self.event {
                TraceEventType::Open => content,
                TraceEventType::CloseOk(_) => content.green().to_string(),
                TraceEventType::CloseError(_) => content.red().to_string(),
                TraceEventType::CloseFailure(_) => content.magenta().to_string(),
                TraceEventType::CloseIncomplete(_) => content.yellow().to_string(),
            };
        }

        writeln!(f, "{}{}{}", indent, content, ctx)
    }
}
