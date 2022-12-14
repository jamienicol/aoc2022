use anyhow::{anyhow, Context, Result};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Map {
    width: isize,
    length: isize,
    heights: Vec<u32>,
}

impl Map {
    fn height_at(&self, pos: Position) -> Option<u32> {
        (pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.length)
            .then(|| self.heights[(pos.y * self.width + pos.x) as usize])
    }

    fn neighbours(&self, pos: Position) -> impl Iterator<Item = Position> + '_ {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(move |(dx, dy)| Position {
                x: pos.x + dx,
                y: pos.y + dy,
            })
            .filter(move |neighbour_pos| {
                match (self.height_at(pos), self.height_at(*neighbour_pos)) {
                    (Some(height), Some(neighbour_height)) => neighbour_height <= height + 1,
                    _ => false,
                }
            })
    }

    fn lowest_points(&self) -> impl Iterator<Item = Position> + '_ {
        self.heights.iter().enumerate().filter_map(|(i, height)| {
            (*height == 0).then_some(Position {
                x: i as isize % self.width,
                y: i as isize / self.width,
            })
        })
    }
}

fn parse_input(input: &str) -> Result<(Map, Position, Position)> {
    let width = input.lines().next().context("Empty input")?.len();
    let length = input.lines().count();

    let mut start = None;
    let mut end = None;
    let mut heights = vec![0; width * length];

    for (y, line) in input.trim_end().lines().enumerate() {
        if line.chars().count() != width {
            return Err(anyhow!(
                "Input row {} has {} chars (expected {})",
                y + 1,
                line.chars().count(),
                width
            ));
        }

        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    if start.is_some() {
                        return Err(anyhow!("Input has multiple start positions"));
                    }
                    start = Some(Position {
                        x: x as isize,
                        y: y as isize,
                    });
                    heights[y * width + x] = 0;
                }
                'E' => {
                    if end.is_some() {
                        return Err(anyhow!("Input has multiple end positions"));
                    }
                    end = Some(Position {
                        x: x as isize,
                        y: y as isize,
                    });
                    heights[y * width + x] = 'z' as u32 - 'a' as u32;
                }
                c if ('a'..='z').contains(&c) => {
                    heights[y * width + x] = c as u32 - 'a' as u32;
                }
                c => return Err(anyhow!("Unexpected char {:?}", c)),
            }
        }
    }

    Ok((
        Map {
            width: width as isize,
            length: length as isize,
            heights,
        },
        start.context("No start position found")?,
        end.context("No end position found")?,
    ))
}

fn a_star(start: Position, end: Position, map: &Map) -> Option<isize> {
    #[derive(Debug, Clone, Copy)]
    struct Cost {
        g: isize,
        h: isize,
    }

    fn h(pos: Position, end: Position) -> isize {
        (end.x - pos.x).abs() + (end.y - pos.y).abs()
    }

    let mut open: HashMap<Position, Cost> = HashMap::default();
    let mut closed: HashSet<Position> = HashSet::default();
    open.insert(
        start,
        Cost {
            g: 0,
            h: h(start, end),
        },
    );

    while let Some((current_pos, current_cost)) = open
        .iter()
        .min_by_key(|(_pos, cost)| cost.g + cost.h)
        .map(|(pos, cost)| (*pos, *cost))
    {
        open.remove(&current_pos);
        closed.insert(current_pos);

        if current_pos == end {
            assert_eq!(current_cost.h, 0);
            return Some(current_cost.g);
        }

        // Calculate the cost for each neighbouring cell and add to open list.
        for neighbour in map
            .neighbours(current_pos)
            .filter(|neighbour| !closed.contains(neighbour))
        {
            let g = current_cost.g + 1;
            let h = h(neighbour, end);
            open.entry(neighbour)
                .and_modify(|existing| {
                    assert_eq!(h, existing.h);
                    // If we've found a shorter route to an already discovered cell, update its cost.
                    existing.g = g.min(existing.g);
                })
                .or_insert(Cost { g, h });
        }
    }

    None
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input12.txt")?;

    let (map, start, end) = parse_input(&input).context("Error parsing input")?;
    let result_a = a_star(start, end, &map).context("Failed to find path")?;
    println!("Day 12, part A: {}", result_a);

    let result_b = map
        .lowest_points()
        .filter_map(|start| a_star(start, end, &map))
        .min()
        .context("Failed to find path")?;
    println!("Day 12, part B: {}", result_b);

    Ok(())
}
