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

Then, wrap your parsers with the `tr` function:

```rust
use nom_tracer::tr;
use nom::bytes::complete::tag;

fn parse_hello(input: &str) -> IResult<&str, &str> {
    tr("parse_hello", tag("hello"))(input)
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


## Advanced Usage

`nom-tracer` provides several functions for more advanced tracing scenarios. Here's an overview of the main tracing functions:

### `tr_ctx`: Tracing with Context

```rust
pub fn tr_ctx<I, O, E, F>(
    name: &'static str,
    context: &'static str,
    parser: F
) -> impl FnMut(I) -> IResult<I, O, E>
```

This function wraps a parser with tracing, using the default tag and providing a context string. The context is useful for adding more detailed information about the parser's purpose or role.

Example:
```rust
use nom_tracer::tr_ctx;

fn parse_greeting(input: &str) -> IResult<&str, &str> {
    tr_ctx("parse_greeting", "Greeting parser", tag("hello"))(input)
}
```

### `tr_tag`: Tracing with Custom Tags

```rust
pub fn tr_tag<I, O, E, F>(
    tag: &'static str,
    name: &'static str,
    parser: F
) -> impl FnMut(I) -> IResult<I, O, E>
```

This function allows you to specify a custom tag for the trace, which is useful for organizing traces into different categories or groups.

Example:
```rust
use nom_tracer::tr_tag;

fn parse_number(input: &str) -> IResult<&str, &str> {
    tr_tag("numeric", "parse_number", nom::character::complete::digit1)(input)
}
```

### `tr_tag_ctx`: Tracing with Custom Tags and Context

```rust
pub fn tr_tag_ctx<I, O, E, F>(
    tag: &'static str,
    context: Option<&'static str>,
    name: &'static str,
    parser: F
) -> impl FnMut(I) -> IResult<I, O, E>
```

This is the most flexible tracing function, allowing you to specify both a custom tag and an optional context.

Example:
```rust
use nom_tracer::tr_tag_ctx;

fn parse_complex(input: &str) -> IResult<&str, &str> {
    tr_tag_ctx("complex", Some("Complex parser section"), "parse_complex", 
        // Your complex parser implementation here
    )(input)
}
```

### Using Multiple Tags

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

### Context Information

When using the `trace-context` feature, you can add context information to your parsers. This is especially useful for error reporting and debugging complex parsers.

The context is included in the trace output and, when the `trace-context` feature is enabled, it's also added to error messages, making it easier to pinpoint where and why a parser failed.

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

### Trace

The `trace` feature is the core functionality of `nom-tracer`. When enabled, it allows you to wrap your parsers with tracing functions that record the execution flow of your parsing operations.

To use the `trace` feature, add it to your `Cargo.toml`:

```toml
[dependencies]
nom-tracer = { version = "0.1.0", features = ["trace"] }
```

Benefits of `trace`:

1. **Debugging**: Easily identify where and why your parsers fail.
2. **Performance Analysis**: Understand the execution path of your parsers to optimize them.
3. **Documentation**: The trace output can serve as a form of dynamic documentation of your parser's behavior.

Example usage:

```rust
use nom_tracer::tr;
use nom::bytes::complete::tag;
use nom::IResult;

fn parse_hello(input: &str) -> IResult<&str, &str> {
    tr("parse_hello", tag("hello"))(input)
}

let result = parse_hello("hello world");
println!("Trace:\n{}", nom_tracer::get_trace());
```

When you run this code, you'll get a trace output showing the execution of your parser:

```
parse_hello("hello world")
parse_hello -> Ok("hello")
```

The `trace` feature is enabled by default. If you want to disable it in production builds for zero overhead, you can use:

```toml
[dependencies]
nom-tracer = { version = "0.1.0", default-features = false }
```

This will compile out all tracing code, ensuring your parsers run at full speed in production.

### Trace Color

The `trace-color` feature enhances the readability of your trace output by adding color coding. This is particularly useful when dealing with complex parsers or large trace outputs.

To use the `trace-color` feature, add it to your `Cargo.toml`:

```toml
[dependencies]
nom-tracer = { version = "0.1.0", features = ["trace-color"] }
```

Benefits of `trace-color`:

1. **Improved Readability**: Different colors for different types of events make it easier to scan and understand the trace output.
2. **Quick Identification**: Easily spot successes, errors, and other important events in your trace.
3. **Hierarchy Visualization**: Indentation and colors help visualize the hierarchical structure of your parsers.

Example usage:

```rust
use nom_tracer::tr;
use nom::sequence::tuple;
use nom::character::complete::{alpha1, digit1};
use nom::IResult;

fn parse_alpha_num(input: &str) -> IResult<&str, (&str, &str)> {
    tr("alpha_num", tuple((
        tr("alpha", alpha1),
        tr("digit", digit1)
    )))(input)
}

let result = parse_alpha_num("abc123xyz");
println!("Trace:\n{}", nom_tracer::get_trace());
```

When you run this code with `trace-color` enabled, you'll get a colorized output similar to:

```
alpha_num("abc123xyz")
| alpha("abc123xyz")
| alpha -> Ok("abc")
| digit("123xyz")
| digit -> Ok("123")
alpha_num -> Ok(("abc", "123"))
```

In the actual output, different colors will be used for:
- Parser names (e.g., "alpha_num", "alpha", "digit")
- Input strings
- Successful results (in green)
- Errors (in red)
- Incomplete results (in yellow)

Note that the colors may not display correctly in all terminals or when redirecting output to a file. In such cases, you might want to disable this feature.

### Trace Context

The `trace-context` feature allows you to add contextual information to your parsers. This context is included in the trace output and, more importantly, in error messages. This feature is particularly useful for complex parsers where you need more information about where and why a parsing operation failed.

To use the `trace-context` feature, add it to your `Cargo.toml`:

```toml
[dependencies]
nom-tracer = { version = "0.1.0", features = ["trace-context"] }
```

Benefits of `trace-context`:

1. **Rich Error Messages**: Errors include the context of where they occurred, making debugging easier.
2. **Improved Trace Output**: Trace events include the context, providing more detailed information about the parser's execution.
3. **Self-Documenting Code**: The context can serve as inline documentation of your parser's structure and purpose.

Example usage:

```rust
use nom_tracer::tr_ctx;
use nom::bytes::complete::tag;
use nom::IResult;
use nom::error::{VerboseError, VerboseErrorKind};

type VerboseResult<I, O> = IResult<I, O, VerboseError<I>>;

fn parse_greeting(input: &str) -> VerboseResult<&str, &str> {
    tr_ctx("parse_greeting", "Parsing a formal greeting", tag("Hello, "))(input)
}

let result = parse_greeting("Hi, world!");
println!("Result: {:?}", result);
println!("Trace:\n{}", nom_tracer::get_trace());
```

When you run this code, if there's an error, you'll get a more informative error message:

```
Result: Err(Error(VerboseError { errors: [("Hi, world!", Nom(Tag)), ("Hi, world!", Context("Parsing a formal greeting"))] }))

Trace:
parse_greeting[Parsing a formal greeting]("Hi, world!")
parse_greeting -> Error(VerboseError { errors: [("Hi, world!", Nom(Tag)), ("Hi, world!", Context("Parsing a formal greeting"))] })
```

In this example, the error message includes both the specific parsing error (failed to match the tag "Hello, ") and the context ("Parsing a formal greeting"). This additional information can be invaluable when debugging complex parsers.

The `trace-context` feature works best when combined with nom's `VerboseError` type, which can accumulate multiple error contexts. However, it will also work with other error types, adding the context information where possible.

### Real-time Tracing with `trace-print`

The `trace-print` feature is particularly useful for debugging complex parsers, especially those that might cause stack overflows. When enabled, this feature prints trace events to the console in real-time, without buffering.

To use `trace-print`, add it to your `Cargo.toml`:

```toml
[dependencies]
nom-tracer = { version = "0.1.0", features = ["trace-print"] }
```

Benefits of `trace-print`:

1. **Immediate Feedback**: See trace events as they happen, helping you pinpoint where issues occur.
2. **Stack Overflow Debugging**: If your parser is causing a stack overflow, you'll see the trace up to the point of the overflow, which can be crucial for identifying the problem.
3. **Performance Insights**: Observe in real-time how your parser progresses through the input, which can help identify performance bottlenecks.

Example usage:

```rust
use nom_tracer::tr;
use nom::multi::many1;
use nom::character::complete::alpha1;

fn parse_many_alpha(input: &str) -> IResult<&str, Vec<&str>> {
    tr("many_alpha", many1(tr("alpha", alpha1)))(input)
}

// This will print trace events in real-time
let result = parse_many_alpha("abcdefghijklmnopqrstuvwxyz");
```

When running this code with the `trace-print` feature enabled, you'll see trace events printed to the console as the parser executes, even if it encounters an issue like a stack overflow.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.
