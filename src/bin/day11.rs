use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{cut, map, map_res},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
enum Operand {
    Old,
    Literal(usize),
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(Operand),
    Mul(Operand),
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    op: Operation,
    test_divisor: usize,
    true_target: usize,
    false_target: usize,
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_monkey_header(input: &str) -> IResult<&str, usize> {
    delimited(tag("Monkey "), parse_usize, tag(":\n"))(input)
}

fn parse_starting_items(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        tag("  Starting items: "),
        separated_list1(tag(", "), parse_usize),
        newline,
    )(input)
}

fn parse_operand(input: &str) -> IResult<&str, Operand> {
    alt((
        map(tag("old"), |_| Operand::Old),
        map(parse_usize, Operand::Literal),
    ))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    delimited(
        tag("  Operation: new = old "),
        alt((
            map(preceded(tag("+ "), parse_operand), |operand| {
                Operation::Add(operand)
            }),
            map(preceded(tag("* "), parse_operand), |operand| {
                Operation::Mul(operand)
            }),
        )),
        newline,
    )(input)
}

fn parse_test_divisor(input: &str) -> IResult<&str, usize> {
    delimited(tag("  Test: divisible by "), parse_usize, newline)(input)
}

fn parse_true_target(input: &str) -> IResult<&str, usize> {
    delimited(tag("    If true: throw to monkey "), parse_usize, newline)(input)
}

fn parse_false_target(input: &str) -> IResult<&str, usize> {
    delimited(tag("    If false: throw to monkey "), parse_usize, newline)(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(
        many1(newline),
        cut(map(
            tuple((
                parse_monkey_header,
                parse_starting_items,
                parse_operation,
                parse_test_divisor,
                parse_true_target,
                parse_false_target,
            )),
            |(_num, items, op, test_divisor, true_target, false_target)| Monkey {
                items,
                op,
                test_divisor,
                true_target,
                false_target,
            },
        )),
    )(input)
}

fn run(mut monkeys: Vec<Monkey>, num_iterations: usize, really_worried: bool) -> usize {
    let mut items_inspected = vec![0; monkeys.len()];

    let common_divisor = monkeys.iter().map(|m| m.test_divisor).product::<usize>();

    for _round in 0..num_iterations {
        for i in 0..monkeys.len() {
            // Work around the borrow checker. Remember to give the items
            // back to the monkeys when done.
            let mut items = std::mem::take(&mut monkeys[i].items);
            let op = monkeys[i].op;
            let test_divisor = monkeys[i].test_divisor;
            let true_target = monkeys[i].true_target;
            let false_target = monkeys[i].false_target;
            let mut true_items = std::mem::take(&mut monkeys[true_target].items);
            let mut false_items = std::mem::take(&mut monkeys[false_target].items);

            items_inspected[i] += items.len();

            items.drain(..).for_each(|mut item| {
                match op {
                    Operation::Add(Operand::Literal(val)) => {
                        item += val;
                    }
                    Operation::Add(Operand::Old) => {
                        item += item;
                    }
                    Operation::Mul(Operand::Literal(val)) => {
                        item *= val;
                    }
                    Operation::Mul(Operand::Old) => {
                        item *= item;
                    }
                };

                if !really_worried {
                    item /= 3;
                }

                item %= common_divisor;

                if item % test_divisor == 0 {
                    true_items.push(item);
                } else {
                    false_items.push(item);
                }
            });

            monkeys[true_target].items = true_items;
            monkeys[false_target].items = false_items;
        }
    }

    items_inspected.iter().sorted().rev().take(2).product()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input11.txt")?;

    let monkeys = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let result_a = run(monkeys.clone(), 20, false);
    println!("Day 11, part A: {}", result_a);

    let result_b = run(monkeys, 10000, true);
    println!("Day 11, part B: {}", result_b);

    Ok(())
}
