# nom-tracer

[![Crates.io](https://img.shields.io/crates/v/nom-tracer.svg)](https://crates.io/crates/nom-tracer)
[![Documentation](https://docs.rs/nom-tracer/badge.svg)](https://docs.rs/nom-tracer)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

`nom-tracer` is a powerful and flexible tracing utility for the [nom](https://github.com/Geal/nom) parser combinator library. It allows you to easily trace the execution of your parsers, providing invaluable insights for debugging and optimization.

## Features

- 🔍 Trace parser execution with minimal code changes
- 🚀 **Near-zero overhead when disabled** - compile out all tracing code in release builds
- 🎨 Colorized output for easy reading (optional)
- 🏷️ Support for multiple trace tags to organize parser traces
- 📊 Hierarchical view of parser execution
- 🔧 Configurable via Cargo features

## Performance

One of the key advantages of `nom-tracer` is its minimal performance impact:

- **When disabled**: The tracing code is completely compiled out, resulting in **virtually zero overhead**. Your parsers will run at full speed in production builds.
- **When enabled**: The tracing functionality is designed to be as lightweight as possible, allowing for detailed insights with minimal performance cost during development and debugging.

## Quick Start

Add `nom-tracer` to your `Cargo.toml`:

```toml
[dependencies]
nom-tracer = "0.1"
```

Then, wrap your parsers with the `tr` function or use the `trace!` macro:

```rust
use nom_tracer::{tr, trace};
use nom::bytes::complete::tag;

fn parse_hello(input: &str) -> IResult<&str, &str> {
    tr("parse_hello", tag("hello"))(input)
}

// Or using the trace! macro
fn parse_world(input: &str) -> IResult<&str, &str> {
    trace!(tag("world"))(input)
}

let result = parse_hello("hello world");
println!("Parse result: {:?}", result);
println!("Trace:\n{}", nom_tracer::get_trace());
```

For production builds, you can disable all tracing features to ensure zero overhead:

```toml
[dependencies]
nom-tracer = { version = "0.1.0", default-features = false }
```

## Macros

`nom-tracer` provides several macros to make tracing easier and more flexible:

### `trace!`

The `trace!` macro is the primary and most flexible way to add tracing to your nom parsers. It's designed to be easy to use while providing powerful functionality.

#### Functionality

The `trace!` macro wraps your parser with tracing functionality. It automatically captures the function name where it's used and allows you to optionally specify a custom tag and context. Under the hood, it uses the `tr_tag_ctx` function, providing a more convenient interface.

#### Usage Patterns

The `trace!` macro supports four main usage patterns:

1. `trace!(parser)`
    - Uses the default tag and no context.
    - Automatically captures the function name as the parser name.

2. `trace!(tag, parser)`
    - Uses a custom tag and no context.
    - Automatically captures the function name as the parser name.

3. `trace!("context", parser)`
    - Uses the default tag and a custom context.
    - Automatically captures the function name as the parser name.

4. `trace!(tag, "context", parser)`
    - Uses a custom tag and a custom context.
    - Automatically captures the function name as the parser name.

#### Examples

Let's look at some examples to illustrate how to use the `trace!` macro in different scenarios:

1. Basic usage:

```rust
use nom_tracer::trace;
use nom::bytes::complete::tag;
use nom::IResult;

fn parse_hello(input: &str) -> IResult<&str, &str> {
    trace!(tag("hello"))(input)
}
```

2. Using a custom tag:

```rust
use nom_tracer::trace;
use nom::character::complete::alpha1;
use nom::IResult;

fn parse_name(input: &str) -> IResult<&str, &str> {
    trace!(names, alpha1)(input)
}
```

3. Adding context:

```rust
use nom_tracer::trace;
use nom::character::complete::digit1;
use nom::IResult;

fn parse_age(input: &str) -> IResult<&str, &str> {
    trace!("Parsing user age", digit1)(input)
}
```

4. Using both a custom tag and context:

```rust
use nom_tracer::trace;
use nom::sequence::tuple;
use nom::character::complete::{alpha1, char, digit1};
use nom::IResult;

fn parse_person(input: &str) -> IResult<&str, (&str, char, &str)> {
    trace!(
        person_parser,
        "Parsing person: name,separator,age",
        tuple((alpha1, char(','), digit1))
    )(input)
}
```

#### Nested Parsers

The `trace!` macro can be particularly useful when working with nested parsers:

```rust
use nom_tracer::trace;
use nom::sequence::tuple;
use nom::character::complete::{alpha1, char, digit1};
use nom::IResult;

fn parse_person(input: &str) -> IResult<&str, (&str, char, &str)> {
    trace!(
        person_parser,
        "Parsing person",
        tuple((
            trace!(name_parser, "Parsing name", alpha1),
            trace!(char(',')),
            trace!(age_parser, "Parsing age", digit1)
        ))
    )(input)
}
```

#### Benefits of Using the `trace!` Macro

1. **Automatic Function Name Capture**: You don't need to manually specify the parser name, reducing boilerplate code.
2. **Flexibility**: Easy to add tags and context as needed, without changing the function signature.
3. **Readability**: The macro syntax is concise and clearly indicates where tracing is applied.
4. **Easy to Enable/Disable**: When you compile with the `trace` feature disabled, all `trace!` macros effectively disappear, leaving no runtime overhead.

### `activate_trace!`

Activates tracing for either the default tag or a specified tag.

Usage:

```rust
use nom_tracer::activate_trace;

// Activate tracing for the default tag
activate_trace!();

// Activate tracing for a custom tag
activate_trace!(my_custom_tag);
```

### `deactivate_trace!`

Deactivates tracing for either the default tag or a specified tag.

Usage:

```rust
use nom_tracer::deactivate_trace;

// Deactivate tracing for the default tag
deactivate_trace!();

// Deactivate tracing for a custom tag
deactivate_trace!(my_custom_tag);
```

### `reset_trace!`

Resets the trace for either the default tag or a specified tag, clearing all recorded events.

Usage:

```rust
use nom_tracer::reset_trace;

// Reset trace for the default tag
reset_trace!();

// Reset trace for a custom tag
reset_trace!(my_custom_tag);
```

### `get_trace!`

The `get_trace!` macro provides a convenient way to retrieve traces for either the default tag or a specified tag.

```rust
use nom_tracer::{trace, get_trace};
use nom::bytes::complete::tag;

let _ = trace!(tag("hello"))("hello world");
let default_trace = get_trace!(); // Gets trace for default tag

let _ = trace!(my_tag, tag("hello"))("hello world");
let my_tag_trace = get_trace!(my_tag); // Gets trace for "my_tag"

println!("Default trace:\n{}", default_trace);
println!("My tag trace:\n{}", my_tag_trace);
```

### `print_trace!`

The `print_trace!` macro provides a convenient way to print traces for either the default tag or a specified tag.

```rust
use nom_tracer::{trace, print_trace};
use nom::bytes::complete::tag;

let _ = trace!(tag("hello"))("hello world");
print_trace!(); // Prints trace for default tag

let _ = trace!(my_tag, tag("hello"))("hello world");
print_trace!(my_tag); // Prints trace for "my_tag"
```

## Core Tracing Functions

While the `trace!` macro is convenient for most use cases, `nom-tracer` also provides direct function calls for more advanced scenarios. These functions offer finer control over the tracing process and can be useful in situations where you need to dynamically determine tracing parameters or integrate with existing code structures.

### `tr`: Basic Tracing

The `tr` function is the simplest way to add tracing to a parser. It uses the default tag and no context.

```rust
pub fn tr<I, O, E, F>(name: &'static str, parser: F) -> impl FnMut(I) -> IResult<I, O, E>
```

Example usage:

```rust
use nom_tracer::tr;
use nom::bytes::complete::tag;
use nom::IResult;

fn parse_hello(input: &str) -> IResult<&str, &str> {
    tr("parse_hello", tag("hello"))(input)
}
```

### `tr_ctx`: Tracing with Context

`tr_ctx` allows you to specify a context string along with the parser name.

```rust
pub fn tr_ctx<I, O, E, F>(
    name: &'static str,
    context: &'static str,
    parser: F
) -> impl FnMut(I) -> IResult<I, O, E>
```

Example usage:

```rust
use nom_tracer::tr_ctx;
use nom::bytes::complete::tag;
use nom::IResult;

fn parse_greeting(input: &str) -> IResult<&str, &str> {
    tr_ctx("parse_greeting", "Parsing a formal greeting", tag("Hello, "))(input)
}
```

### `tr_tag`: Tracing with Custom Tags

`tr_tag` allows you to specify a custom tag for organizing traces.

```rust
pub fn tr_tag<I, O, E, F>(
    tag: &'static str,
    name: &'static str,
    parser: F
) -> impl FnMut(I) -> IResult<I, O, E>
```

Example usage:

```rust
use nom_tracer::tr_tag;
use nom::character::complete::digit1;
use nom::IResult;

fn parse_year(input: &str) -> IResult<&str, &str> {
    tr_tag("date_parser", "parse_year", digit1)(input)
}
```

### `tr_tag_ctx`: Tracing with Custom Tags and Context

`tr_tag_ctx` is the most flexible function, allowing you to specify both a custom tag and a context.

```rust
pub fn tr_tag_ctx<I, O, E, F>(
    tag: &'static str,
    context: Option<&'static str>,
    name: &'static str,
    parser: F
) -> impl FnMut(I) -> IResult<I, O, E>
```

Example usage:

```rust
use nom_tracer::tr_tag_ctx;
use nom::sequence::tuple;
use nom::character::complete::{alpha1, char, digit1};
use nom::IResult;

fn parse_person(input: &str) -> IResult<&str, (&str, char, &str)> {
    tr_tag_ctx(
        "person_parser",
        Some("Parsing person: name,separator,age"),
        "parse_person",
        tuple((alpha1, char(','), digit1))
    )(input)
}
```

## Trace Retrieval functions

`nom-tracer` provides several functions for retrieving and printing trace information:

### Retrieving Traces

#### `get_trace()`

Retrieves the trace for the default tag.

```rust
use nom_tracer::{trace, get_trace};
use nom::bytes::complete::tag;

let _ = trace!(tag("hello"))("hello world");
let trace = get_trace();
println!("Default trace:\n{}", trace);
```

#### `get_trace_for_tag(tag: &'static str)`

Retrieves the trace for a specific tag.

```rust
use nom_tracer::{trace, get_trace_for_tag};
use nom::bytes::complete::tag;

let _ = trace!(my_tag, tag("hello"))("hello world");
let trace = get_trace_for_tag("my_tag");
println!("My tag trace:\n{}", trace);
```

### Printing Traces

#### `print_trace()`

Prints the entire trace for the default tag to the console.

```rust
use nom_tracer::{trace, print_trace};
use nom::bytes::complete::tag;

let _ = trace!(tag("hello"))("hello world");
print_trace();
```

#### `print_trace_for_tag(tag: &'static str)`

Prints the trace for a specific tag to the console.

```rust
use nom_tracer::{trace, print_trace_for_tag};
use nom::bytes::complete::tag;

let _ = trace!(my_tag, tag("hello"))("hello world");
print_trace_for_tag("my_tag");
```

## Using Multiple Tags

You can use different tags to organize your traces into separate groups:

```rust
use nom_tracer::{tr_tag, get_trace_for_tag};

fn parse_name(input: &str) -> IResult<&str, &str> {
    tr_tag("names", "parse_name", nom::character::complete::alpha1)(input)
}

fn parse_age(input: &str) -> IResult<&str, u32> {
    tr_tag("numbers", "parse_age", nom::character::complete::u32)(input)
}

// Later, you can retrieve traces for specific tags:
let name_traces = get_trace_for_tag("names");
let number_traces = get_trace_for_tag("numbers");
```

## Context Information

Context information in `nom-tracer` provides additional details about each parser's purpose or role. This feature is especially useful for error reporting and debugging complex parsers. When the `trace-context` feature is enabled, this context is included in both the trace output and error messages.

### Enabling the Feature

To use context information, enable the `trace-context` feature in your `Cargo.toml`:

```toml
[dependencies]
nom-tracer = { version = "0.1.0", features = ["trace-context"] }
```

### Adding Context

You can add context using either the `trace!` macro or the `tr_ctx` and `tr_tag_ctx` functions:

Using the `trace!` macro:

```rust
use nom_tracer::trace;
use nom::character::complete::alpha1;
use nom::IResult;

fn parse_username(input: &str) -> IResult<&str, &str> {
    trace!("Parsing username (alphabetic characters only)", alpha1)(input)
}
```

Using the `tr_ctx` function:

```rust
use nom_tracer::tr_ctx;
use nom::character::complete::alpha1;
use nom::IResult;

fn parse_username(input: &str) -> IResult<&str, &str> {
    tr_ctx("parse_username", "Parsing username (alphabetic characters only)", alpha1)(input)
}
```

### Example with Error Handling

Here's an example that demonstrates how context information enhances error messages:

```rust
use nom_tracer::trace;
use nom::character::complete::{alpha1, digit1};
use nom::sequence::tuple;
use nom::IResult;

fn parse_user_id(input: &str) -> IResult<&str, (&str, &str)> {
    trace!(
        "Parsing user ID (format: <username>-<number>)",
        tuple((
            trace!("Parsing username part", alpha1),
            trace!("Parsing separator", tag("-")),
            trace!("Parsing numeric part", digit1)
        ))
    )(input)
}

fn main() {
    let result = parse_user_id("john123");  // Missing hyphen
    match result {
        Ok((remainder, (username, id))) => {
            println!("Parsed successfully. Username: {}, ID: {}", username, id);
        }
        Err(e) => {
            println!("Parsing failed: {:?}", e);
            // The error message will include the context "Parsing separator"
        }
    }
}
```

In this example, if the input doesn't match the expected format, the error message will include the context of the parser that failed (in this case, "Parsing separator"), making it clear which part of the input caused the failure.

### Considerations

1. **Verbosity**: While context information is valuable, overly verbose contexts can clutter your code and trace output. Aim for concise yet informative context messages.

2. **Consistency**: Try to maintain a consistent style and level of detail in your context messages across your parsing code for better readability.

By leveraging context information, you can significantly improve the debuggability and maintainability of your nom parsers, especially in larger and more complex parsing scenarios.

## Real-time Printing with `trace-print`

The `trace-print` feature allows you to see trace events as they happen, providing immediate feedback during parser execution. This can be particularly useful for debugging complex parsers or those that might cause stack overflows.

**Note**: The trace-print feature works in conjunction with the activation state of tags. Only trace events for activated tags will be printed in real-time. This means you can still control which parts of your parser generate output by activating or deactivating specific tags, even when using real-time printing.

### Enabling the Feature

To use real-time printing, you need to enable the `trace-print` feature in your `Cargo.toml`:

```toml
[dependencies]
nom-tracer = { version = "0.1.0", features = ["trace-print"] }
```

### Example Usage

Here's an example of how you might use the `trace-print` feature:

```rust
use nom_tracer::trace;
use nom::multi::many1;
use nom::character::complete::alpha1;
use nom::IResult;

fn parse_words(input: &str) -> IResult<&str, Vec<&str>> {
    trace!("Parsing multiple words", many1(trace!(alpha1)))(input)
}

fn main() {
    let input = "hello world parser";
    let result = parse_words(input);
    println!("Final result: {:?}", result);
}
```

When you run this code with the `trace-print` feature enabled, you'll see trace events printed to the console in real-time as the parser executes, even before the final result is printed.

### Considerations

1. **Output Volume**: Real-time printing can generate a lot of console output, especially for complex parsers or large inputs. Be prepared for potentially verbose output.

2. **Interleaved Output**: If you're also printing other information to the console, it may become interleaved with the trace output. Consider using different output streams or formatting to distinguish between trace events and other output.

## Cargo Features

- `trace`: Enable tracing (default)
- `trace-color`: Enable colorized output
- `trace-print`: Print trace events in real-time (unbuffered)
- `trace-context`: Add context information to error messages

To enable a feature, add it to your `Cargo.toml`:

```toml
[dependencies]
nom-tracer = { version = "0.1.0", features = ["trace-color", "trace-context"] }
```

For production builds, you can disable all tracing features to ensure zero overhead:

```toml
[dependencies]
nom-tracer = { version = "0.1.0", default-features = false }
```

### Trace: `trace`

The `trace` feature is the core functionality of `nom-tracer`. When enabled, it allows you to wrap your parsers with tracing functions that record the execution flow of your parsing operations.

### Trace Color: `trace-color`

The `trace-color` feature enhances the readability of your trace output by adding color coding. This is particularly useful when dealing with complex parsers or large trace outputs.

### Trace Context: `trace-context`

The `trace-context` feature allows you to add contextual information to your parsers. This context is included in the trace output and, more importantly, in error messages. This feature is particularly useful for complex parsers where you need more information about where and why a parsing operation failed.

### Real-time Tracing: `trace-print`

The `trace-print` feature is particularly useful for debugging complex parsers, especially those that might cause stack overflows. When enabled, this feature prints trace events to the console in real-time, without buffering.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.
