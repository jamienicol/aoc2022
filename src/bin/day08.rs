use anyhow::{anyhow, Context, Result};
use itertools::{iproduct, Itertools};
use take_until::TakeUntilExt;

#[derive(Debug)]
struct Trees {
    width: usize,
    length: usize,
    trees: Vec<u32>,
}

struct TreeIter<'a> {
    trees: &'a Trees,
    pos: (isize, isize),
    step: (isize, isize),
}

impl<'a> TreeIter<'a> {
    fn new(trees: &'a Trees, pos: (usize, usize), step: (isize, isize)) -> Self {
        assert!(pos.0 < trees.width, "invalid x: {}", pos.0);
        assert!(pos.1 < trees.length, "invalid y: {}", pos.1);
        assert!(step.0 != 0 || step.1 != 0);

        TreeIter {
            trees,
            pos: (pos.0 as isize, pos.1 as isize),
            step,
        }
    }
}

impl<'a> Iterator for TreeIter<'a> {
    type Item = &'a u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.pos.0 += self.step.0;
        self.pos.1 += self.step.1;
        if self.pos.0 >= 0
            && self.pos.1 >= 0
            && self.pos.0 < self.trees.width as isize
            && self.pos.1 < self.trees.length as isize
        {
            Some(
                &self.trees.trees[self
                    .trees
                    .tree_idx(self.pos.0 as usize, self.pos.1 as usize)],
            )
        } else {
            None
        }
    }
}

impl Trees {
    fn tree_idx(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width, "invalid x: {}", x);
        assert!(y < self.length, "invalid y: {}", y);
        y * self.width + x
    }

    fn height_at(&self, x: usize, y: usize) -> u32 {
        self.trees[self.tree_idx(x, y)]
    }

    fn to_left(&self, x: usize, y: usize) -> TreeIter {
        TreeIter::new(self, (x, y), (-1, 0))
    }

    fn to_right(&self, x: usize, y: usize) -> TreeIter {
        TreeIter::new(self, (x, y), (1, 0))
    }

    fn above(&self, x: usize, y: usize) -> TreeIter {
        TreeIter::new(self, (x, y), (0, -1))
    }

    fn below(&self, x: usize, y: usize) -> TreeIter {
        TreeIter::new(self, (x, y), (0, 1))
    }

    fn all_dirs(&self, x: usize, y: usize) -> [TreeIter; 4] {
        [
            self.above(x, y),
            self.to_left(x, y),
            self.to_right(x, y),
            self.below(x, y),
        ]
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
            trees
                .all_dirs(*x, *y)
                .iter_mut()
                .any(|dir| dir.all(|other| *other < height))
        })
        .count();
    println!("Day 8, part A: {}", result_a);

    let result_b: usize = iproduct!(0..trees.width, 0..trees.length)
        .map(|(x, y)| {
            let height = trees.height_at(x, y);
            trees
                .all_dirs(x, y)
                .iter_mut()
                .map(|dir| dir.take_until(|other| **other >= height).count())
                .product()
        })
        .max()
        .unwrap();
    println!("Day 8, part B: {}", result_b);

    Ok(())
}
