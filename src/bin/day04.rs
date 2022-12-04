use std::ops::RangeInclusive;

use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    map(
        separated_pair(
            map_res(digit1, |s: &str| s.parse::<u32>()),
            tag("-"),
            map_res(digit1, |s: &str| s.parse::<u32>()),
        ),
        |pair| pair.0..=pair.1,
    )(input)
}

#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    separated_list1(newline, separated_pair(parse_range, tag(","), parse_range))(input)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input04.txt")?;
    let pairs = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let result_a = pairs
        .iter()
        .filter(|pair| {
            (pair.0.start() <= pair.1.start() && pair.0.end() >= pair.1.end())
                || (pair.1.start() <= pair.0.start() && pair.1.end() >= pair.0.end())
        })
        .count();
    println!("Day 4, part A: {}", result_a);

    let result_b = pairs
        .iter()
        .filter(|pair| pair.0.start() <= pair.1.end() && pair.1.start() <= pair.0.end())
        .count();
    println!("Day 4, part B: {}", result_b);

    Ok(())
}
