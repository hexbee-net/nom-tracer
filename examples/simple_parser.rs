// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

use {
    nom::{bytes::complete::tag, character::complete::alpha1, sequence::tuple, IResult},
    nom_tracer::{activate_trace, print_trace, trace},
};

fn parse_greeting(input: &str) -> IResult<&str, (&str, &str)> {
    trace!(
        "Parsing a greeting (format: 'hello' + name)",
        tuple((
            trace!("Parsing 'hello'", tag("hello")),
            trace!("Parsing name", alpha1)
        ))
    )(input)
}

fn main() {
    activate_trace!();

    let result = parse_greeting("helloworld");
    println!("Parse result: {:?}", result);

    print_trace!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        super::main();
    }
}
