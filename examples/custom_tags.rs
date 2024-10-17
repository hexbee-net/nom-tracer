// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

use {
    nom::{
        bytes::complete::tag,
        character::complete::{alpha1, digit1},
        sequence::tuple,
        IResult,
    },
    nom_tracer::{get_trace, print_trace, trace},
};

fn parse_user_id(input: &str) -> IResult<&str, (&str, &str, &str)> {
    trace!(
        user_parser,
        "Parsing user ID (format: name-number)",
        tuple((
            trace!(name_parser, "Parsing name", alpha1),
            trace!(separator_parser, tag("-")),
            trace!(id_parser, "Parsing ID number", digit1)
        ))
    )(input)
}

fn main() {
    let result = parse_user_id("john-123");
    println!("Parse result: {:?}", result);

    println!("User parser trace:");
    print_trace!(user_parser);

    println!("\nName parser trace:");
    println!("{}", get_trace!(name_parser));

    println!("\nID parser trace:");
    println!("{}", get_trace!(id_parser));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        super::main();
    }
}
