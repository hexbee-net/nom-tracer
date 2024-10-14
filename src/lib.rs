// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

#[cfg(all(feature = "trace", feature = "trace-color"))]
use colored::Colorize;
#[cfg(all(feature = "trace", feature = "trace-context"))]
use nom::error::ContextError;
#[cfg(feature = "trace")]
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};
use {
    nom::{IResult, Parser},
    std::fmt::Debug,
};

/// The default tag used when no specific tag is provided.
#[cfg(feature = "trace")]
pub const DEFAULT_TAG: &str = "default";

thread_local! {
    /// Thread-local storage for the global [TraceList].
    ///
    /// [NOM_TRACE] provides a thread-safe way to access and modify the global trace list.
    /// It's implemented as thread-local storage, ensuring that each thread has its own
    /// independent trace list. This allows for concurrent tracing in multithreaded applications
    /// without the need for explicit synchronization.
    ///
    /// The [RefCell](std::cell::RefCell) allows for interior mutability, so the [TraceList] can be
    /// modified even when accessed through a shared reference.
    ///
    /// Usage of [NOM_TRACE] is typically wrapped in the [tr] and [tr_tag_ctx] functions,
    /// which provide a more convenient interface for adding trace events.
    #[cfg(feature = "trace")]
    pub static NOM_TRACE: std::cell::RefCell<TraceList> = ::std::cell::RefCell::new(TraceList::new());
}

/// A collection of traces, each associated with a tag.
///
/// The tag system allows for multiple independent traces to be maintained simultaneously.
/// Each tag corresponds to a separate `Trace` instance, allowing for organization and
/// separation of trace events based on different criteria (e.g., parser type, subsystem, etc.).
#[cfg(feature = "trace")]
#[derive(Default)]
pub struct TraceList {
    pub traces: HashMap<&'static str, Trace>,
}

#[cfg(feature = "trace")]
impl TraceList {
    /// Creates a new [TraceList] with a default trace.
    ///
    /// The default trace is associated with the `DEFAULT_TAG`.
    pub fn new() -> Self {
        let mut traces = HashMap::new();
        traces.insert(
            DEFAULT_TAG,
            Trace {
                events: Vec::new(),
                level: 0,
                active: true,
            },
        );

        TraceList { traces }
    }

    /// Resets the trace for the given tag.
    ///
    /// This clears all events and resets the nesting level for the specified trace.
    /// If the trace doesn't exist, a new one is created and inserted.
    pub fn reset(&mut self, tag: &'static str) {
        let t = self.traces.entry(tag).or_insert(Trace {
            events: Vec::new(),
            level: 0,
            active: true,
        });
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
        let t = self.traces.entry(tag).or_insert(Trace {
            events: Vec::new(),
            level: 0,
            active: true,
        });
        t.active = true;
    }

    /// Deactivates the trace for the given tag.
    ///
    /// Deactivated traces will not record events, but will retain previously recorded events.
    /// If the trace doesn't exist, a new one is created (but left inactive).
    pub fn deactivate(&mut self, tag: &'static str) {
        let t = self.traces.entry(tag).or_insert(Trace {
            events: Vec::new(),
            level: 0,
            active: true,
        });
        t.active = false;
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
        let t = self.traces.entry(tag).or_insert(Trace {
            events: Vec::new(),
            level: 0,
            active: true,
        });
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
        let t = self.traces.entry(tag).or_insert(Trace {
            events: Vec::new(),
            level: 0,
            active: true,
        });
        t.close(context, input, location, result);
    }
}

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

#[cfg(feature = "trace-context")]
pub trait TraceError<I>: Debug + ContextError<I> {}
#[cfg(feature = "trace-context")]
impl<I, E> TraceError<I> for E where E: Debug + ContextError<I> {}

#[cfg(not(feature = "trace-context"))]
pub trait TraceError<I>: Debug {}
#[cfg(not(feature = "trace-context"))]
impl<I, E> TraceError<I> for E where E: Debug {}

/// Wraps a parser with tracing, using the default tag.
///
/// This is the simplest tracing function, which wraps a parser with tracing functionality
/// using the default tag. It's ideal for basic tracing needs when you don't need to
/// categorize traces or add additional context.
///
/// # Arguments
///
/// * `name` - A static string identifying the parser.
/// * `parser` - The parser to be wrapped.
///
/// # Example
///
/// ```
/// use nom_tracer::tr;
/// use nom::bytes::complete::tag;
/// use nom::IResult;
///
/// fn parse_hello(input: &str) -> IResult<&str, &str> {
///     tr("parse_hello", tag("hello"))(input)
/// }
///
/// let result = parse_hello("hello world");
/// assert_eq!(result, Ok((" world", "hello")));
/// ```
#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn tr<I, O, E, F>(name: &'static str, mut parser: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: AsRef<str>,
    F: Parser<I, O, E>,
    I: Clone,
    O: Debug,
    E: TraceError<I>,
{
    #[cfg(feature = "trace")]
    {
        tr_tag_ctx(DEFAULT_TAG, None, name, parser)
    }

    #[cfg(not(feature = "trace"))]
    {
        move |input: I| parser.parse(input)
    }
}

/// Wraps a parser with tracing, using the default tag and a context.
///
/// This function is similar to [tr], but it allows you to specify a context
/// string that provides additional information about the parser's purpose or role.
///
/// # Arguments
///
/// * `name` - A static string identifying the parser.
/// * `context` - A static string providing context for the parser.
/// * `parser` - The parser to be wrapped.
///
/// # Example
///
/// ```
/// use nom_tracer::tr_ctx;
/// use nom::bytes::complete::tag;
/// use nom::IResult;
///
/// fn parse_greeting(input: &str) -> IResult<&str, &str> {
///     tr_ctx("parse_greeting", "Greeting parser", tag("hello"))(input)
/// }
///
/// let result = parse_greeting("hello world");
/// assert_eq!(result, Ok((" world", "hello")));
/// ```
#[allow(unused_mut)]
#[allow(unused_variables)]
pub fn tr_ctx<I, O, E, F>(
    name: &'static str,
    context: &'static str,
    mut parser: F,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: AsRef<str>,
    F: Parser<I, O, E>,
    I: Clone,
    O: Debug,
    E: TraceError<I>,
{
    #[cfg(feature = "trace")]
    {
        tr_tag_ctx(DEFAULT_TAG, Some(context), name, parser)
    }

    #[cfg(not(feature = "trace"))]
    {
        move |input: I| parser.parse(input)
    }
}

/// Wraps a parser with tracing, using a specified tag.
///
/// This function allows you to organize traces into different categories or groups
/// by specifying a custom tag.
///
/// # Arguments
///
/// * `tag` - A static string used to categorize the trace.
/// * `name` - A static string identifying the parser.
/// * `parser` - The parser to be wrapped.
///
/// # Example
///
/// ```
/// use nom_tracer::tr_tag;
/// use nom::character::complete::digit1;
/// use nom::IResult;
///
/// fn parse_number(input: &str) -> IResult<&str, &str> {
///     tr_tag("numeric", "parse_number", digit1)(input)
/// }
///
/// let result = parse_number("123 abc");
/// assert_eq!(result, Ok((" abc", "123")));
/// ```
#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn tr_tag<I, O, E, F>(
    tag: &'static str,
    name: &'static str,
    mut parser: F,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: AsRef<str>,
    F: Parser<I, O, E>,
    I: Clone,
    O: Debug,
    E: TraceError<I>,
{
    #[cfg(feature = "trace")]
    {
        tr_tag_ctx(tag, None, name, parser)
    }

    #[cfg(not(feature = "trace"))]
    {
        move |input: I| parser.parse(input)
    }
}

/// Wraps a parser with tracing, using a specified tag and optional context.
///
/// This is the most flexible tracing function, allowing you to specify both a custom tag
/// and an optional context.
///
/// # Arguments
///
/// * `tag` - A static string used to categorize the trace.
/// * `context` - An optional static string providing context for the parser.
/// * `name` - A static string identifying the parser.
/// * `parser` - The parser to be wrapped.
///
/// # Example
///
/// ```
/// use nom_tracer::tr_tag_ctx;
/// use nom::bytes::complete::tag;
/// use nom::IResult;
///
/// fn parse_complex(input: &str) -> IResult<&str, &str> {
///     tr_tag_ctx("complex", Some("Complex parser section"), "parse_complex", tag("complex"))(input)
/// }
///
/// let result = parse_complex("complex input");
/// assert_eq!(result, Ok((" input", "complex")));
/// ```
#[allow(unused_variables)]
pub fn tr_tag_ctx<I, O, E, F>(
    tag: &'static str,
    context: Option<&'static str>,
    name: &'static str,
    mut parser: F,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: AsRef<str>,
    F: Parser<I, O, E>,
    I: Clone,
    O: Debug,
    E: TraceError<I>,
{
    #[cfg(feature = "trace")]
    {
        move |input: I| {
            let input1 = input.clone();
            let input2 = input.clone();
            let input3 = input.clone();

            NOM_TRACE.with(|trace| {
                (*trace.borrow_mut()).open(tag, context, input1, name);
            });

            let res = parser.parse(input);

            NOM_TRACE.with(|trace| {
                (*trace.borrow_mut()).close(tag, context, input2, name, &res);
            });

            #[cfg(feature = "trace-context")]
            {
                match res {
                    Ok(o) => Ok(o),
                    Err(nom::Err::Error(e)) => {
                        Err(nom::Err::Error(E::add_context(input3, name, e)))
                    }
                    Err(nom::Err::Failure(e)) => {
                        Err(nom::Err::Failure(E::add_context(input3, name, e)))
                    }
                    Err(nom::Err::Incomplete(i)) => Err(nom::Err::Incomplete(i)),
                }
            }
            #[cfg(not(feature = "trace-context"))]
            {
                res
            }
        }
    }

    #[cfg(not(feature = "trace"))]
    {
        #[cfg(feature = "trace-context")]
        {
            move |input: I| match parser.parse(input.clone()) {
                Ok(o) => Ok(o),
                Err(nom::Err::Error(e)) => Err(nom::Err::Error(E::add_context(input, name, e))),
                Err(nom::Err::Failure(e)) => Err(nom::Err::Failure(E::add_context(input, name, e))),
                Err(nom::Err::Incomplete(i)) => Err(nom::Err::Incomplete(i)),
            }
        }
        #[cfg(not(feature = "trace-context"))]
        {
            move |input: I| parser.parse(input)
        }
    }
}

/// Gets the trace for the default tag.
///
/// When the "trace" feature is not enabled, this function always returns an empty string.
///
/// # Example
///
/// ```
/// # use nom::bytes::complete::tag;
/// # use nom::IResult;
/// # use nom_tracer::{get_trace, tr};
///
/// fn parse_hello(input: &str) -> IResult<&str, &str> {
///     tr("parse_hello", tag("hello"))(input)
/// }
///
/// let _ = parse_hello("hello world");
/// let trace = get_trace();
/// println!("Default trace:\n{}", trace);
/// ```
pub fn get_trace() -> String {
    #[cfg(feature = "trace")]
    {
        get_trace_for_tag(DEFAULT_TAG)
    }

    #[cfg(not(feature = "trace"))]
    {
        String::new()
    }
}

/// Gets the trace for a specified tag.
///
/// When the "trace" feature is not enabled, this function always returns an empty string.
///
/// If no trace exists for the given tag, returns an error message.
///
/// # Example
///
/// ```
/// # use nom::bytes::complete::tag;
/// # use nom::IResult;
/// # use nom_tracer::{get_trace_for_tag, tr_tag_ctx};
///
/// fn parse_world(input: &str) -> IResult<&str, &str> {
///     tr_tag_ctx("greeting",None, "parse_world", tag("world"))(input)
/// }
///
/// let _ = parse_world("world hello");
/// let trace = get_trace_for_tag("greeting");
/// println!("Greeting trace:\n{}", trace);
///
/// // Trying to get a trace for a non-existent tag
/// let non_existent_trace = get_trace_for_tag("non_existent");
/// println!("Non-existent trace: {}", non_existent_trace);
/// ```
pub fn get_trace_for_tag(tag: &'static str) -> String {
    #[cfg(feature = "trace")]
    {
        NOM_TRACE.with(|trace| {
            if let Some(trace) = trace.borrow().traces.get(tag) {
                trace.to_string()
            } else {
                format!("No trace found for tag '{}'", tag)
            }
        })
    }

    #[cfg(not(feature = "trace"))]
    {
        let _ = tag;
        String::new()
    }
}

pub(crate) fn print_colored<I: AsRef<str>>(s: I) {
    use std::io::Write;

    #[cfg(all(feature = "trace", feature = "trace-color"))]
    {
        use termcolor::{ColorChoice, StandardStream};

        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        write!(&mut stdout, "{}", s.as_ref()).unwrap();
    }
    #[cfg(not(all(feature = "trace", feature = "trace-color")))]
    {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        writeln!(handle, "{}", s.as_ref()).unwrap();
    }
}

#[cfg(all(test, feature = "trace"))]
mod tests {
    use {
        super::*,
        nom::{bytes::complete::tag, sequence::tuple, IResult},
    };

    #[test]
    fn test_trace_list_new() {
        let trace_list = TraceList::new();
        assert!(trace_list.traces.contains_key(DEFAULT_TAG));
    }

    #[test]
    fn test_trace_list_reset() {
        let mut trace_list = TraceList::new();
        trace_list.open(DEFAULT_TAG, None, "input", "location");
        trace_list.reset(DEFAULT_TAG);
        assert_eq!(trace_list.traces[DEFAULT_TAG].events.len(), 0);
        assert_eq!(trace_list.traces[DEFAULT_TAG].level, 0);
    }

    #[test]
    fn test_trace_list_activate_deactivate() {
        let mut trace_list = TraceList::new();
        trace_list.deactivate(DEFAULT_TAG);
        assert!(!trace_list.traces[DEFAULT_TAG].active);
        trace_list.activate(DEFAULT_TAG);
        assert!(trace_list.traces[DEFAULT_TAG].active);
    }

    #[test]
    fn test_tr() {
        fn parse_ab(input: &str) -> IResult<&str, (&str, &str)> {
            tuple((tag("a"), tag("b")))(input)
        }

        let mut traced_parser = tr("parse_ab", parse_ab);
        let result = traced_parser("ab");
        assert!(result.is_ok());

        let trace = get_trace();
        assert!(trace.contains("parse_ab"));
        assert!(trace.contains("-> Ok"));
    }

    #[test]
    fn test_tr_tag() {
        fn parse_cd(input: &str) -> IResult<&str, (&str, &str)> {
            tuple((tag("c"), tag("d")))(input)
        }

        let custom_tag = "custom";
        let mut traced_parser = tr_tag_ctx(custom_tag, None, "parse_cd", parse_cd);
        let result = traced_parser("cd");
        assert!(result.is_ok());

        let trace = get_trace_for_tag(custom_tag);
        assert!(trace.contains("parse_cd"));
        assert!(trace.contains("-> Ok"));
    }

    #[test]
    fn test_trace_with_error() {
        fn parse_fail(input: &str) -> IResult<&str, &str> {
            tag("nonexistent")(input)
        }

        let mut traced_parser = tr("parse_fail", parse_fail);
        let result = traced_parser("ab");
        assert!(result.is_err());

        let trace = get_trace();
        assert!(trace.contains("parse_fail"));
        assert!(trace.contains("-> Error"));
    }

    #[test]
    fn test_nested_traces() {
        fn parse_nested(input: &str) -> IResult<&str, (&str, &str)> {
            tr(
                "outer",
                tuple((tr("inner_a", tag("a")), tr("inner_b", tag("b")))),
            )(input)
        }

        let traced_parser = parse_nested;
        let result = traced_parser("ab");
        assert!(result.is_ok());

        let trace = get_trace();
        assert!(trace.contains("outer"));
        assert!(trace.contains("inner_a"));
        assert!(trace.contains("inner_b"));
    }
}
