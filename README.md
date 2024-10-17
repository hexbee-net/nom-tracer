# nom-tracer

[![Crates.io](https://img.shields.io/crates/v/nom-tracer.svg)](https://crates.io/crates/nom-tracer)
[![Documentation](https://docs.rs/nom-tracer/badge.svg)](https://docs.rs/nom-tracer)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

`nom-tracer` is a powerful and flexible tracing utility for the [nom](https://github.com/Geal/nom) parser combinator library.
It allows you to easily trace the execution of your parsers, providing invaluable insights for debugging and optimization.

## Contents

- [Features](#features)
- [Performance](#performance)
- [Quick Start](#quick-start)
- [Macros](#macros)
   - [trace!](#trace)
   - [silence_tree!](#silence_tree)
   - [activate_trace! and deactivate_trace!](#activate_trace-and-deactivate_trace)
   - [reset_trace!](#reset_trace)
   - [get_trace!](#get_trace)
   - [print_trace!](#print_trace)
   - [set_max_level!](#set_max_level)
- [Cargo Features](#cargo-features)
- [Context Information](#context-information)
- [Contributing](#contributing)
- [License](#license)

## Features

- 🔍 Trace parser execution with minimal code changes
- 🚀 **Zero overhead when disabled** - compile out all tracing code in release builds
- 🎨 Colorized output for easy reading (optional)
- 🏷️ Support for multiple trace tags to organize parser traces
- 📊 Hierarchical view of parser execution
- 🤫 Silence subtrees to reduce noise in well-tested parts of your parser
- 🔧 Configurable via Cargo features

![image](https://github.com/user-attachments/assets/b420d0fb-ae84-4351-ba93-4d21f046f55a)

## Performance

One of the key advantages of `nom-tracer` is its minimal performance impact:

- **When disabled**: The tracing code is completely compiled out, resulting in **virtually zero overhead**. Your parsers will run at full speed in production builds.
- **When enabled**: The tracing functionality is designed to be as lightweight as possible, allowing for detailed insights with minimal performance cost during development and debugging.

## Quick Start

Add `nom-tracer` to your `Cargo.toml`:

```toml
[dependencies]
nom-tracer = "0.2"
```

Then, use the `trace!` macro to wrap your parsers:

```rust
use nom_tracer::trace;
use nom::bytes::complete::tag;
use nom::IResult;

fn parse_hello(input: &str) -> IResult<&str, &str> {
    trace!(tag("hello"))(input)
}

fn parse_world(input: &str) -> IResult<&str, &str> {
    trace!(tag("world"))(input)
}

fn main() {
    let result = parse_hello("hello world");
    println!("Parse result: {:?}", result);
    println!("Trace:\n{}", nom_tracer::get_trace!());
}
```

For production builds, you can disable all tracing features to ensure zero overhead:

```toml
[dependencies]
nom-tracer = { version = "0.2", default-features = false }
```

For more detailed examples showcasing various features of `nom-tracer`, check out the [`examples`](https://github.com/hexbee-net/nom-tracer/tree/main/src) folder in the root of the repository.
These examples demonstrate real-world usage scenarios and can help you get started with more advanced tracing techniques.

## Macros

`nom-tracer` provides several macros to make tracing easier and more flexible. Here's a detailed explanation of each macro:

### trace!

The `trace!` macro is the primary way to add tracing to your nom parsers.
It wraps a parser with tracing functionality, recording its execution path and results.

You can use `trace!` in several ways:

1. `trace!(parser)`: Uses the default tag and no context.
2. `trace!(tag, parser)`: Uses a custom tag and no context.
3. `trace!("context", parser)`: Uses the default tag and a custom context.
4. `trace!(tag, "context", parser)`: Uses a custom tag and a custom context.

Here's an example that demonstrates these different usage patterns:

```rust
use nom_tracer::trace;
use nom::sequence::tuple;
use nom::character::complete::{alpha1, char, digit1};
use nom::IResult;

fn parse_person(input: &str) -> IResult<&str, (&str, char, &str)> {
    trace!(
        person_parser,
        "Parsing person: name,separator,age",
        tuple((
            trace!(name_parser, "Parsing name", alpha1),
            trace!(char(',')),
            trace!(age_parser, "Parsing age", digit1)
        ))
    )(input)
}
```

The `trace!` macro is designed to be flexible and easy to use in various scenarios.
It automatically captures the function name where it's used, reducing boilerplate code.
When compiling with the `trace` feature disabled, all `trace!` macros effectively disappear, leaving no runtime overhead.

### silence_tree!

The `silence_tree!` macro allows you to silence tracing for a subtree of parsers.
This is useful for reducing noise in well-tested or less interesting parts of your parser.

You can use `silence_tree!` in several ways:

1. `silence_tree!(parser)`: Silences the default tag.
2. `silence_tree!(tag, parser)`: Silences a specific tag.
3. `silence_tree!("context", parser)`: Silences the default tag with a context.
4. `silence_tree!(tag, "context", parser)`: Silences a specific tag with a context.

Here's an example:

```rust
use nom_tracer::{trace, silence_tree};
use nom::sequence::tuple;
use nom::character::complete::{alpha1, char, digit1};
use nom::IResult;

fn parse_person(input: &str) -> IResult<&str, (&str, char, &str)> {
    trace!(
        person_parser,
        "Parsing person",
        tuple((
            trace!(alpha1),
            trace!(char(',')),
            silence_tree!("Silenced age parsing", digit1)
        ))
    )(input)
}
```

Use `silence_tree!` for parts of your parser that are well-tested or when you want to focus on specific areas of your parser.
Silenced subtrees still execute normally but don't generate trace output. The `silence_tree!` macro is only available when the `trace-silencing` feature is enabled.

### activate_trace! and deactivate_trace!

These macros allow you to dynamically activate or deactivate tracing for either the default tag or a specified tag.

```rust
use nom_tracer::{activate_trace, deactivate_trace};

// Activate/deactivate tracing for the default tag
activate_trace!();
deactivate_trace!();

// Activate/deactivate tracing for a custom tag
activate_trace!(my_custom_tag);
deactivate_trace!(my_custom_tag);
```

These macros are useful for selectively enabling or disabling tracing in different parts of your application.
You can use them to control tracing at runtime based on conditions or user input.

### reset_trace!

This macro resets the trace for either the default tag or a specified tag, clearing all recorded events.

```rust
use nom_tracer::reset_trace;

// Reset trace for the default tag
reset_trace!();

// Reset trace for a custom tag
reset_trace!(my_custom_tag);
```

`reset_trace!` is handy when you want to clear previous trace data and start fresh. It can be particularly useful in long-running applications or test suites where you want to reset the trace between different parsing operations.

### get_trace!

The `get_trace!` macro retrieves the trace for either the default tag or a specified tag.

```rust
use nom_tracer::{trace, get_trace};
use nom::bytes::complete::tag;

fn main() {
    let _ = trace!(tag("hello"))("hello world");
    let default_trace = get_trace!(); // Gets trace for default tag

    let _ = trace!(my_tag, tag("hello"))("hello world");
    let my_tag_trace = get_trace!(my_tag); // Gets trace for "my_tag"

    println!("Default trace:\n{}", default_trace.unwrap_or_default());
    println!("My tag trace:\n{}", my_tag_trace.unwrap_or_default());
}
```

`get_trace!` returns an `Option<String>`, so you may need to handle the case where no trace is available.
It's useful for retrieving trace information for further processing or display.

### print_trace!

The `print_trace!` macro prints the trace for either the default tag or a specified tag directly to the console.

```rust
use nom_tracer::{trace, print_trace};
use nom::bytes::complete::tag;

fn main() {
    let _ = trace!(tag("hello"))("hello world");
    print_trace!(); // Prints trace for default tag

    let _ = trace!(my_tag, tag("hello"))("hello world");
    print_trace!(my_tag); // Prints trace for "my_tag"
}
```

This macro is convenient for quick debugging or when you want to immediately see the trace output. Keep in mind that it prints to stdout, so be mindful of where and when you use it, especially in production environments.

### set_max_level!

The `set_max_level!` macro allows you to set a maximum nesting level for tracing, which can be useful for detecting infinite recursion or excessively deep parser nesting.

```rust
use nom_tracer::set_max_level;

// Set maximum nesting level to 10 for the default tag
set_max_level!(Some(10));

// Set maximum nesting level to 5 for a custom tag
set_max_level!(my_custom_tag, Some(5));

// Remove the nesting level limit for the default tag
set_max_level!(None);

// Remove the nesting level limit for a custom tag
set_max_level!(my_custom_tag, None);
```

Here's an example of how you might use `set_max_level!` to catch potential infinite recursion:

```rust
use nom_tracer::{trace, set_max_level};
use nom::sequence::tuple;
use nom::character::complete::{alpha1, char};
use nom::IResult;

fn recursive_parser(input: &str) -> IResult<&str, &str> {
    trace!(
        tuple((
            alpha1,
            char(','),
            |i| recursive_parser(i)  // This could lead to infinite recursion
        ))
    )(input)
}

fn main() {
    // Set a maximum nesting level of 5
    set_max_level!(Some(5));

    // This will panic if the nesting level exceeds 5
    let _ = recursive_parser("a,b,c,d,e,f,g");
}
```

`set_max_level!` is primarily a debugging tool, useful during development to catch potential issues with recursive parsers or unexpected deep nesting. The appropriate maximum level depends on your parser's structure. Set it high enough to allow for valid deep nesting, but low enough to catch potential infinite recursion. You can set different limits for different tags, allowing for fine-grained control over various parts of your parser. This macro is only available when the `trace-max-level` feature is enabled.

## Cargo Features

- `trace`: Enable tracing (default)
- `trace-color`: Enable colorized output
- `trace-print`: Print trace events in real-time (unbuffered)
- `trace-context`: Add context information to error messages (can be used independently of `trace`)
- `trace-silencing`: Enable the `silence_tree!` macro functionality
- `trace-max-level`: Enable maximum nesting level functionality

To enable features, add them to your `Cargo.toml`:

```toml
[dependencies]
nom-tracer = { version = "0.2", features = ["trace-color", "trace-context", "trace-silencing"] }
```

Note that the `trace-context` feature can be used independently of the `trace` feature. This allows you to add context to your `nom` errors without enabling full tracing functionality.

## Context Information

The `trace-context` feature enhances both trace output and error messages with additional context. This feature can be used in conjunction with the `trace` feature or independently.

When used with `trace`, it eliminates the need for nom's `context` combinator, simplifying your parser code while still providing rich contextual information:

```rust
use nom_tracer::trace;
use nom::character::complete::alpha1;
use nom::IResult;

fn parse_username(input: &str) -> IResult<&str, &str> {
    trace!("Parsing username (alphabetic characters only)", alpha1)(input)
}
```

In this example, if the parser fails, the error message will include the context "Parsing username (alphabetic characters only)", without requiring the use of nom's `context` combinator.

The `trace-context` feature can also be used independently of full tracing. This allows you to enhance your `nom` error messages with additional context without enabling full tracing functionality. To use `trace-context` without full tracing, configure your `Cargo.toml` like this:

```toml
[dependencies]
nom-tracer = { version = "0.2", default-features = false, features = ["trace-context"] }
```

This configuration provides enhanced error messages with context information while avoiding the overhead of full tracing in production environments. It's particularly useful when you want more informative error messages but don't need the detailed execution trace that full tracing provides.

Whether used with or without full tracing, the `trace-context` feature helps in quickly identifying where and why parsing failures occur, significantly improving the debugging experience.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is license
