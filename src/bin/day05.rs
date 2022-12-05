use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1, multispace1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_input(input: &str) -> IResult<&str, (Vec<Vec<char>>, Vec<Move>)> {
    let (input, raw_stacks) = terminated(
        separated_list1(
            alt((newline, char(' '))),
            alt((
                map(delimited(tag("["), anychar, tag("]")), Some),
                map(tag("   "), |_| None),
            )),
        ),
        newline,
    )(input)?;

    let (input, stack_names) = terminated(
        delimited(tag(" "), separated_list1(tag("   "), digit1), tag(" ")),
        multispace1,
    )(input)?;

    let num_stacks = stack_names.len();
    let mut stacks = vec![Vec::new(); num_stacks];
    assert_eq!(raw_stacks.len() % num_stacks, 0);
    let tallest_stack_height = raw_stacks.len() / num_stacks;
    for y in (0..tallest_stack_height).rev() {
        for (x, stack) in stacks.iter_mut().enumerate() {
            if let Some(c) = raw_stacks[y * num_stacks + x] {
                stack.push(c);
            }
        }
    }

    let (input, moves) = separated_list1(
        newline,
        map(
            tuple((
                map_res(preceded(tag("move "), digit1), |s: &str| s.parse::<usize>()),
                map_res(preceded(tag(" from "), digit1), |s: &str| {
                    s.parse::<usize>()
                }),
                map_res(preceded(tag(" to "), digit1), |s: &str| s.parse::<usize>()),
            )),
            |(count, from, to)| Move { count, from, to },
        ),
    )(input)?;

    Ok((input, (stacks, moves)))
}

fn move_crates(stacks: &[Vec<char>], moves: &[Move], preserve_order: bool) -> String {
    let mut stacks = stacks.to_vec();
    for m in moves {
        // Appease the borrow checker
        let mut from = std::mem::take(&mut stacks[m.from - 1]);
        let mut to = std::mem::take(&mut stacks[m.to - 1]);

        let moved = from.drain((from.len() - m.count)..);
        if preserve_order {
            to.extend(moved);
        } else {
            to.extend(moved.rev())
        }

        stacks[m.from - 1] = from;
        stacks[m.to - 1] = to;
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input05.txt")?;

    let (stacks, moves) = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let result_a = move_crates(&stacks, &moves, false);
    println!("Day 5, part A: {}", result_a);

    let result_b = move_crates(&stacks, &moves, true);
    println!("Day 5, part B: {}", result_b);

    Ok(())
}
