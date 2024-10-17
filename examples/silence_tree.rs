// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "trace-silencing")]
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::map,
    multi::many1,
    sequence::tuple,
    IResult,
};
#[cfg(feature = "trace-silencing")]
use nom_tracer::{activate_trace, print_trace, silence_tree, trace};

#[cfg(feature = "trace-silencing")]
fn parse_item(input: &str) -> IResult<&str, (&str, &str)> {
    trace!(
        "Parsing item",
        map(tuple((alpha1, tag(":"), digit1)), |(name, _, quantity)| (
            name, quantity
        ))
    )(input)
}

#[cfg(feature = "trace-silencing")]
#[allow(clippy::type_complexity)]
fn parse_shopping_list(input: &str) -> IResult<&str, Vec<((&str, &str), &str)>> {
    trace!(
        "Parsing shopping list",
        many1(tuple((
            silence_tree!("Parsing list item (silenced)", parse_item),
            trace!("Parsing item separator", tag(","))
        )))
    )(input)
}

#[cfg(feature = "trace-silencing")]
fn main() {
    activate_trace!();

    let input = "apple:3,banana:2,orange:5";
    let result = parse_shopping_list(input);
    println!("Parse result: {:?}", result);

    print_trace!();
}

#[cfg(not(feature = "trace-silencing"))]
fn main() {
    println!("This example requires the 'trace-silencing' feature to be enabled.");
    println!("Please add the following to your Cargo.toml:");
    println!("nom-tracer = {{ version = \"1.0\", features = [\"trace-silencing\"] }}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_main() {
        super::main();
    }
}
