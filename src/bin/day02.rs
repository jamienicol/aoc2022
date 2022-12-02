use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{newline, one_of},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Copy, Clone, Debug)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl Outcome {
    fn points(&self) -> u32 {
        match *self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn points(&self) -> u32 {
        match *self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

trait Turn {
    fn from_input(input: (char, char)) -> Self;
    fn my_move(&self) -> Move;
    fn outcome(&self) -> Outcome;
    fn points(&self) -> u32 {
        self.my_move().points() + self.outcome().points()
    }
}

#[derive(Copy, Clone, Debug)]
struct TurnA {
    their_move: Move,
    my_move: Move,
}

impl Turn for TurnA {
    fn from_input(input: (char, char)) -> Self {
        let their_move = match input.0 {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => unreachable!(),
        };

        let my_move = match input.1 {
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
            _ => unreachable!(),
        };

        Self {
            their_move,
            my_move,
        }
    }

    fn my_move(&self) -> Move {
        self.my_move
    }

    fn outcome(&self) -> Outcome {
        match (self.my_move, self.their_move) {
            (Move::Rock, Move::Rock) => Outcome::Draw,
            (Move::Rock, Move::Paper) => Outcome::Lose,
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (Move::Paper, Move::Paper) => Outcome::Draw,
            (Move::Paper, Move::Scissors) => Outcome::Lose,
            (Move::Scissors, Move::Rock) => Outcome::Lose,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            (Move::Scissors, Move::Scissors) => Outcome::Draw,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct TurnB {
    their_move: Move,
    outcome: Outcome,
}

impl Turn for TurnB {
    fn from_input(input: (char, char)) -> Self {
        let their_move = match input.0 {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            _ => unreachable!(),
        };

        let outcome = match input.1 {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!(),
        };

        Self {
            their_move,
            outcome,
        }
    }

    fn my_move(&self) -> Move {
        match (self.their_move, self.outcome) {
            (Move::Rock, Outcome::Lose) => Move::Scissors,
            (Move::Rock, Outcome::Win) => Move::Paper,
            (Move::Rock, Outcome::Draw) => Move::Rock,
            (Move::Paper, Outcome::Lose) => Move::Rock,
            (Move::Paper, Outcome::Win) => Move::Scissors,
            (Move::Paper, Outcome::Draw) => Move::Paper,
            (Move::Scissors, Outcome::Lose) => Move::Paper,
            (Move::Scissors, Outcome::Win) => Move::Rock,
            (Move::Scissors, Outcome::Draw) => Move::Scissors,
        }
    }

    fn outcome(&self) -> Outcome {
        self.outcome
    }
}

fn parse_input<T: Turn>(input: &str) -> IResult<&str, Vec<T>> {
    separated_list1(
        newline,
        map(
            separated_pair(one_of("ABC"), tag(" "), one_of("XYZ")),
            Turn::from_input,
        ),
    )(input)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input02.txt")?;

    let turns_a: Vec<TurnA> = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;
    let result_a = turns_a.iter().map(Turn::points).sum::<u32>();

    println!("Day 2, part A: {}", result_a);

    let turns_b: Vec<TurnB> = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;
    let result_b = turns_b.iter().map(Turn::points).sum::<u32>();

    println!("Day 2, part B: {}", result_b);

    Ok(())
}
