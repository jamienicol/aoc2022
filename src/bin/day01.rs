use anyhow::{anyhow, Result};
use nom::{
    character::complete::{digit1, newline},
    combinator::map_res,
    multi::{fold_many1, separated_list1},
    sequence::terminated,
    IResult,
};

fn parse_input(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(
        newline,
        fold_many1(
            terminated(map_res(digit1, |s: &str| s.parse::<u32>()), newline),
            || 0,
            |acc: u32, item| acc + item,
        ),
    )(input)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input01.txt")?;
    let mut elves = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    elves.sort();

    let result_a = elves.last().unwrap();
    println!("Day 1, part A: {}", result_a);

    let result_b = elves.iter().rev().take(3).sum::<u32>();
    println!("Day 1, part B: {}", result_b);

    Ok(())
}
