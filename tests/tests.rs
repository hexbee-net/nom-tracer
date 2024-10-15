// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

mod functions;
mod macros;

#[allow(dead_code)]
fn debug_print<I: AsRef<str>>(s: I) {
    use {
        std::io::Write,
        termcolor::{ColorChoice, StandardStream},
    };
    let mut handle = StandardStream::stdout(ColorChoice::Always);
    write!(handle, "{}", s.as_ref()).unwrap();
}
