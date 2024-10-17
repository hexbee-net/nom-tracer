// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

use {
    nom::{bytes::complete::tag, character::complete::alpha1, sequence::tuple, IResult},
    nom_tracer::{activate_trace, deactivate_trace, print_trace, reset_trace, trace},
};

fn parse_greeting(input: &str) -> IResult<&str, (&str, &str)> {
    trace!(
        greeting_parser,
        "Parsing a greeting (format: 'hello' + name)",
        tuple((
            trace!(hello_parser, "Parsing 'hello'", tag("hello")),
            trace!(name_parser, "Parsing name", alpha1)
        ))
    )(input)
}

fn main() {
    println!("1. Parsing with all traces active:");
    activate_trace!(greeting_parser);
    activate_trace!(hello_parser);
    activate_trace!(name_parser);

    let result = parse_greeting("helloworld");
    println!("Parse result: {:?}", result);
    print_trace!(greeting_parser);

    println!("\n2. Parsing with 'name_parser' trace deactivated:");
    deactivate_trace!(name_parser);
    reset_trace!(greeting_parser);

    let result = parse_greeting("helloworld");
    println!("Parse result: {:?}", result);
    print_trace!(greeting_parser);

    println!("\n3. Parsing with all traces deactivated:");
    deactivate_trace!(greeting_parser);
    deactivate_trace!(hello_parser);
    reset_trace!(greeting_parser);

    let result = parse_greeting("helloworld");
    println!("Parse result: {:?}", result);
    print_trace!(greeting_parser);

    println!("\n4. Reactivating 'greeting_parser' trace:");
    activate_trace!(greeting_parser);
    reset_trace!(greeting_parser);

    let result = parse_greeting("helloworld");
    println!("Parse result: {:?}", result);
    print_trace!(greeting_parser);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        super::main();
    }
}
