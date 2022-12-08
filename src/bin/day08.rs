use anyhow::{anyhow, Context, Result};
use itertools::{iproduct, Itertools};

#[derive(Debug)]
struct Trees {
    width: usize,
    length: usize,
    trees: Vec<u32>,
}

impl Trees {
    fn tree_idx(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width, "x: {}", x);
        assert!(y < self.length, "y: {}", y);
        y * self.width + x
    }

    fn height_at(&self, x: usize, y: usize) -> u32 {
        self.trees[self.tree_idx(x, y)]
    }

    fn to_left(&self, x: usize, y: usize) -> impl Iterator<Item = &u32> {
        self.trees[self.tree_idx(0, y)..self.tree_idx(x, y)]
            .iter()
            .rev()
    }

    fn to_right(&self, x: usize, y: usize) -> impl Iterator<Item = &u32> {
        self.trees[self.tree_idx(x, y)..=self.tree_idx(self.width - 1, y)]
            .iter()
            .skip(1)
    }

    fn above(&self, x: usize, y: usize) -> impl Iterator<Item = &u32> {
        self.trees[x..self.tree_idx(x, y)]
            .iter()
            .step_by(self.width)
            .rev()
    }

    fn below(&self, x: usize, y: usize) -> impl Iterator<Item = &u32> {
        self.trees[self.tree_idx(x, y)..]
            .iter()
            .step_by(self.width)
            .skip(1)
    }
}

fn parse_input(input: &str) -> Result<Trees> {
    let width = input.lines().next().context("Empty input")?.len();
    let length = input.lines().count();

    let trees = input
        .trim_end()
        .lines()
        .enumerate()
        .map(|(i, l)| {
            if l.len() == width {
                Ok(l.chars())
            } else {
                Err(anyhow!(
                    "Input row {} has {} chars (expected {})",
                    i + 1,
                    l.len(),
                    width
                ))
            }
        })
        .flatten_ok()
        .map(|c| {
            c.and_then(|c| {
                c.to_digit(10)
                    .with_context(|| format!("Invalid height character: {:?}", c))
            })
        })
        .collect::<Result<Vec<u32>>>()?;

    Ok(Trees {
        width,
        length,
        trees,
    })
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input08.txt")?;

    let trees = parse_input(&input).context("Error parsing input")?;

    let result_a = iproduct!(0..trees.width, 0..trees.length)
        .filter(|(x, y)| {
            let height = trees.height_at(*x, *y);
            trees.to_left(*x, *y).all(|h| *h < height)
                || trees.to_right(*x, *y).all(|h| *h < height)
                || trees.above(*x, *y).all(|h| *h < height)
                || trees.below(*x, *y).all(|h| *h < height)
        })
        .count();
    println!("Day 8, part A: {}", result_a);

    let result_b = iproduct!(0..trees.width, 0..trees.length)
        .map(|(x, y)| {
            let height = trees.height_at(x, y);
            let l = match trees.to_left(x, y).position(|h| *h >= height) {
                Some(d) => d + 1,
                None => trees.to_left(x, y).count(),
            };
            let r = match trees.to_right(x, y).position(|h| *h >= height) {
                Some(d) => d + 1,
                None => trees.to_right(x, y).count(),
            };
            let a = match trees.above(x, y).position(|h| *h >= height) {
                Some(d) => d + 1,
                None => trees.above(x, y).count(),
            };
            let b = match trees.below(x, y).position(|h| *h >= height) {
                Some(d) => d + 1,
                None => trees.below(x, y).count(),
            };

            l * r * a * b
        })
        .max()
        .unwrap();

    println!("Day 8, part B: {}", result_b);

    Ok(())
}
