// Copyright (c) Hexbee
// SPDX-License-Identifier: Apache-2.0

use {
    nom::{
        bytes::complete::tag,
        character::complete::{alpha1, digit1},
        combinator::map,
        multi::many1,
        sequence::tuple,
        IResult,
    },
    nom_tracer::{activate_trace, print_trace, trace},
};

fn parse_item(input: &str) -> IResult<&str, (&str, &str)> {
    trace!(
        "Parsing item (format: name:quantity)",
        map(
            tuple((
                trace!("Parsing item name", alpha1),
                trace!("Parsing item separator", tag(":")),
                trace!("Parsing item quantity", digit1)
            )),
            |(name, _, quantity)| (name, quantity)
        )
    )(input)
}

#[allow(clippy::type_complexity)]
fn parse_shopping_list(input: &str) -> IResult<&str, Vec<((&str, &str), &str)>> {
    trace!(
        "Parsing shopping list",
        many1(tuple((
            trace!("Parsing list item", parse_item),
            trace!("Parsing item separator", tag(","))
        )))
    )(input)
}

fn main() {
    activate_trace!();

    let input = "apple:3,banana:2,orange:5";
    let result = parse_shopping_list(input);
    println!("Parse result: {:?}", result);

    print_trace!();
}
