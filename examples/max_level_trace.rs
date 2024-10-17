// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "trace-max-level")]
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    multi::many0,
    sequence::delimited,
    IResult,
};
use nom::{combinator::map, sequence::pair};
#[cfg(feature = "trace-max-level")]
use nom_tracer::{activate_trace, print_trace, set_max_level, trace};

// This parser can potentially lead to deep nesting or infinite recursion
#[cfg(feature = "trace-max-level")]
fn nested_expression(input: &str) -> IResult<&str, String> {
    trace!(
        expr_parser,
        "Parsing nested expression",
        alt((
            map(
                pair(
                    digit1,
                    delimited(tag("("), |i| nested_expression(i), tag(")")),
                ),
                |(digit1, nested)| format!("{}({})", digit1, nested)
            ),
            map(digit1, |d: &str| d.to_string())
        ))
    )(input)
}

#[cfg(feature = "trace-max-level")]
fn parse_expression(input: &str) -> IResult<&str, Vec<String>> {
    trace!("Parsing full expression", many0(nested_expression))(input)
}

#[cfg(feature = "trace-max-level")]
fn main() {
    activate_trace!(expr_parser);

    // Override the default hook to avoid logging panic location info.
    std::panic::set_hook(Box::new(|_| {}));

    println!("1. Parsing with no max level set:");
    let input = "1(2(3(4)))";

    let result = parse_expression(input);
    print_trace!(expr_parser);
    println!("Result: {:?}", result);

    println!("\n2. Parsing with max level set to 3:");
    set_max_level!(expr_parser, Some(3));
    let input = "1(2(3(4)))";
    let result = std::panic::catch_unwind(|| parse_expression(input));
    print_trace!(expr_parser);
    match result {
        Ok(ok_result) => println!("Result: {:?}", ok_result),
        Err(_) => println!("Parser panicked due to exceeding max level"),
    }

    println!("\n3. Parsing a deeply nested expression:");
    set_max_level!(expr_parser, Some(6));
    let input = "1(2(3(4(5(6(7(8(9(10))))))))))";
    let result = std::panic::catch_unwind(|| parse_expression(input));
    print_trace!(expr_parser);
    match result {
        Ok(ok_result) => println!("Result: {:?}", ok_result),
        Err(_) => println!("Parser panicked due to exceeding max level"),
    }

    println!("\n4. Parsing deeply nested with max level removed:");
    set_max_level!(expr_parser, None);
    let input = "1(2(3(4(5(6(7(8(9(10))))))))))";
    let result = parse_expression(input);
    print_trace!(expr_parser);
    println!("Result: {:?}", result);
}

#[cfg(not(feature = "trace-max-level"))]
fn main() {
    println!("This example requires the 'trace-max-level' feature to be enabled.");
    println!("Please add the following to your Cargo.toml:");
    println!("nom-tracer = {{ version = \"1.0\", features = [\"trace-max-level\"] }}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        super::main();
    }
}
