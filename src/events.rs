// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "trace-color")]
use crate::ansi;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub enum TraceEventType {
    Open,
    CloseOk(String),
    CloseError(String),
    CloseFailure(String),
    CloseIncomplete(nom::Needed),
}

#[derive(Clone)]
pub struct TraceEvent {
    pub level: usize,
    pub location: &'static str,
    pub context: Option<&'static str>,
    pub input: String,
    pub event: TraceEventType,
}

impl Display for TraceEvent {
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
            ctx = format!("{}{}", ansi::BG_BRIGHT_BLUE, ctx);
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
                    ansi::FG_BRIGHT_GREEN,
                    ansi::TEXT_INVERSE,
                    result,
                    ansi::TEXT_INVERSE_RESET
                ),
                TraceEventType::CloseError(e) => format!(
                    "{}-> Error({}{}{})",
                    ansi::FG_BRIGHT_RED,
                    ansi::TEXT_INVERSE,
                    e,
                    ansi::TEXT_INVERSE_RESET
                ),
                TraceEventType::CloseFailure(e) => format!(
                    "{}-> Failure({}{}{})",
                    ansi::FG_BRIGHT_MAGENTA,
                    ansi::TEXT_INVERSE,
                    e,
                    ansi::TEXT_INVERSE_RESET
                ),
                TraceEventType::CloseIncomplete(i) => format!(
                    "{}-> Incomplete({}{:?}{})",
                    ansi::FG_BRIGHT_YELLOW,
                    ansi::TEXT_INVERSE,
                    i,
                    ansi::TEXT_INVERSE_RESET
                ),
            };

            return writeln!(
                f,
                "{}{}{}{}{}",
                indent,
                content,
                ansi::FG_BLACK,
                ctx,
                ansi::RESET
            );
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
            "{}",
            format!(
                "{:#}",
                TraceEvent {
                    level: 2,
                    location: "test_location",
                    context: Some("test_context"),
                    input: "test_input".to_string(),
                    event: TraceEventType::Open,
                }
            )
        );
    }

    #[test]
    fn test_display_close_ok() {
        println!(
            "{}",
            format!(
                "{:#}",
                TraceEvent {
                    level: 2,
                    location: "test_location",
                    context: Some("test_context"),
                    input: "test_input".to_string(),
                    event: TraceEventType::CloseOk("ok".to_string()),
                }
            )
        );
    }

    #[test]
    fn test_display_close_error() {
        println!(
            "{}",
            format!(
                "{:#}",
                TraceEvent {
                    level: 2,
                    location: "test_location",
                    context: Some("test_context"),
                    input: "test_input".to_string(),
                    event: TraceEventType::CloseError("error".to_string()),
                }
            )
        );
    }

    #[test]
    fn test_display_close_failure() {
        println!(
            "{}",
            format!(
                "{:#}",
                TraceEvent {
                    level: 2,
                    location: "test_location",
                    context: Some("test_context"),
                    input: "test_input".to_string(),
                    event: TraceEventType::CloseFailure("failure".to_string()),
                }
            )
        );
    }

    #[test]
    fn test_display_close_incomplete() {
        println!(
            "{}",
            format!(
                "{:#}",
                TraceEvent {
                    level: 2,
                    location: "test_location",
                    context: Some("test_context"),
                    input: "test_input".to_string(),
                    event: TraceEventType::CloseIncomplete(nom::Needed::Size(
                        NonZero::new(5).unwrap()
                    )),
                }
            )
        );
    }
}
