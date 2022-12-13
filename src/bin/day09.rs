use anyhow::{anyhow, Result};
use nom::{
    character::complete::{digit1, newline, one_of, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Motion {
    dir: Direction,
    dist: isize,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Motion>> {
    separated_list1(
        newline,
        map(
            separated_pair(
                map(one_of("UDLR"), |c| match c {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => unreachable!(),
                }),
                space1,
                map_res(digit1, |c: &str| c.parse::<isize>()),
            ),
            |(dir, dist)| Motion { dir, dist },
        ),
    )(input)
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn step(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn is_touching(&self, other: &Position) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }

    fn move_towards(&mut self, other: &Position) {
        self.x += (other.x - self.x).signum();
        self.y += (other.y - self.y).signum();
    }
}

fn run(rope: &mut [Position], motions: &[Motion]) -> usize {
    let mut tail_positions: HashSet<Position> = HashSet::default();
    tail_positions.insert(*rope.last().unwrap());

    for motion in motions {
        for _step in 0..motion.dist {
            rope[0].step(motion.dir);
            for i in 1..rope.len() {
                let head = rope[i - 1];
                if !rope[i].is_touching(&head) {
                    rope[i].move_towards(&head);
                }
            }
            tail_positions.insert(*rope.last().unwrap());
        }
    }

    tail_positions.len()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input09.txt")?;

    let motions = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let result_a = run(&mut [Position { x: 0, y: 0 }; 2], &motions);
    println!("Day 9, part A: {}", result_a);

    let result_b = run(&mut [Position { x: 0, y: 0 }; 10], &motions);
    println!("Day 9, part B: {}", result_b);

    Ok(())
}
