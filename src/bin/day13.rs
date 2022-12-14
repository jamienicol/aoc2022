use std::cmp::Ordering;

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{cut, map, map_res},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, pair, terminated},
    IResult,
};

#[derive(Debug, Clone, Eq)]
enum Data {
    Number(usize),
    List(Vec<Data>),
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Data::Number(lhs), Data::Number(rhs)) => lhs.cmp(rhs),
            (Data::List(lhs), Data::List(rhs)) => lhs.cmp(rhs),
            (Data::Number(lhs), Data::List(rhs)) => vec![Data::Number(*lhs)].cmp(rhs),
            (Data::List(lhs), Data::Number(rhs)) => lhs.cmp(&vec![Data::Number(*rhs)]),
        }
    }
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<Data>> {
    delimited(tag("["), separated_list0(tag(","), parse_data), tag("]"))(input)
}

fn parse_data(input: &str) -> IResult<&str, Data> {
    alt((map(parse_usize, Data::Number), map(parse_list, Data::List)))(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Data, Data)>> {
    separated_list1(
        newline,
        cut(pair(
            terminated(parse_data, newline),
            terminated(parse_data, newline),
        )),
    )(input)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input13.txt")?;

    let pairs = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let result_a = pairs
        .iter()
        .enumerate()
        .filter_map(|(i, pair)| (pair.0 <= pair.1).then_some(i + 1))
        .sum::<usize>();
    println!("Day 13, part A: {}", result_a);

    let divider_a = parse_data("[[2]]")?.1;
    let divider_b = parse_data("[[6]]")?.1;
    let mut pairs = pairs;
    let mut all_packets = pairs
        .drain(..)
        .flat_map(|pair| [pair.0, pair.1])
        .collect::<Vec<Data>>();
    all_packets.push(divider_a.clone());
    all_packets.push(divider_b.clone());

    all_packets.sort();
    let divider_a_pos = all_packets
        .iter()
        .position(|packet| packet == &divider_a)
        .unwrap();
    let divider_b_pos = all_packets
        .iter()
        .position(|packet| packet == &divider_b)
        .unwrap();

    let result_b = (divider_a_pos + 1) * (divider_b_pos + 1);
    println!("Day 13, part B: {}", result_b);

    Ok(())
}
