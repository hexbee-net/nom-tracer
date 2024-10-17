// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

use {
    nom::{
        bytes::complete::tag,
        character::complete::{alpha1, digit1},
        combinator::map,
        error::{VerboseError, VerboseErrorKind},
        sequence::tuple,
        Err,
        IResult,
    },
    nom_tracer::{activate_trace, print_trace, trace},
};

type ParseResult<I, O> = IResult<I, O, VerboseError<I>>;

fn parse_user_info(input: &str) -> ParseResult<&str, (&str, &str)> {
    trace!(
        "Parsing user info (format: name-age)",
        map(
            tuple((
                trace!("Parsing name", alpha1),
                trace!("Parsing separator", tag("-")),
                trace!("Parsing age", digit1)
            )),
            |(name, _, age)| (name, age)
        )
    )(input)
}

fn main() {
    activate_trace!();

    let valid_input = "john-30";
    let invalid_input = "john30";

    println!("Parsing valid input:");
    let result = parse_user_info(valid_input);
    println!("Result: {:?}", result);
    print_trace!();

    println!("\nParsing invalid input:");
    let result = parse_user_info(invalid_input);
    match result {
        Err(Err::Error(e)) | Err(Err::Failure(e)) => {
            println!("Error: {:?}", e);
            for (input, error) in e.errors.iter() {
                match error {
                    VerboseErrorKind::Char(c) => println!("Expected '{}', got '{}'", c, input),
                    VerboseErrorKind::Context(ctx) => println!("Error in {}: '{}'", ctx, input),
                    _ => println!("Other error: {:?}", error),
                }
            }
        }
        _ => println!("Unexpected result: {:?}", result),
    }
    print_trace!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        super::main();
    }
}
