use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

const SAND_SOURCE: Position = Position { x: 500, y: 0 };

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Clone)]
struct Map {
    left: isize,
    top: isize,
    right: isize,
    bottom: isize,
    tiles: Vec<bool>,
}

impl Map {
    fn new(rocks: &[Vec<Position>]) -> Result<Self> {
        // Find the edges of our map so we can allocate as small a vector as
        // possible for the tiles.
        let mut left = SAND_SOURCE.x;
        let mut top = SAND_SOURCE.y;
        let mut right = SAND_SOURCE.x;
        let mut bottom = SAND_SOURCE.y;

        for pos in rocks.iter().flatten() {
            left = left.min(pos.x);
            top = top.min(pos.y);
            right = right.max(pos.x);
            bottom = bottom.max(pos.y);
        }
        assert!(right >= left);
        assert!(bottom >= top);

        let mut map = Self {
            left,
            top,
            bottom,
            right,
            tiles: Vec::new(),
        };
        map.tiles = vec![false; (map.width() * map.height()) as usize];

        for path in rocks {
            for (start, end) in path.iter().tuple_windows() {
                if start.x != end.x && start.y != end.y {
                    return Err(anyhow!(
                        "Paths must be horizontal or vertical. Got start={:?}, end={:?}",
                        start,
                        end
                    ));
                }

                let mut cur = *start;
                while cur != *end {
                    *map.tile_mut(cur).unwrap() = true;
                    cur.x += (end.x - cur.x).signum();
                    cur.y += (end.y - cur.y).signum();
                }
                *map.tile_mut(*end).unwrap() = true;
            }
        }

        Ok(map)
    }

    fn width(&self) -> isize {
        self.right + 1 - self.left
    }

    fn height(&self) -> isize {
        self.bottom + 1 - self.top
    }

    fn tile_idx(&self, pos: Position) -> Option<usize> {
        (pos.x >= self.left && pos.y >= self.top && pos.x <= self.right && pos.y <= self.bottom)
            .then_some(((pos.y - self.top) * self.width() + pos.x - self.left) as usize)
    }

    fn tile(&self, pos: Position) -> Option<&bool> {
        let idx = self.tile_idx(pos);
        idx.map(|idx| &self.tiles[idx])
    }

    fn tile_mut(&mut self, pos: Position) -> Option<&mut bool> {
        let idx = self.tile_idx(pos);
        idx.map(|idx| &mut self.tiles[idx])
    }
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    map_res(digit1, |s: &str| s.parse::<isize>())(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<Position>>> {
    separated_list1(
        newline,
        separated_list1(
            tag(" -> "),
            map(
                separated_pair(parse_isize, tag(","), parse_isize),
                |(x, y)| Position { x, y },
            ),
        ),
    )(input)
}

fn next_positions(pos: Position) -> impl IntoIterator<Item = Position> {
    [
        Position {
            x: pos.x,
            y: pos.y + 1,
        },
        Position {
            x: pos.x - 1,
            y: pos.y + 1,
        },
        Position {
            x: pos.x + 1,
            y: pos.y + 1,
        },
    ]
}

fn drop_sand(map: &mut Map) -> bool {
    if *map.tile(SAND_SOURCE).unwrap() {
        // If the source tile is occupied then no more sand can fall.
        return false;
    }

    let mut pos = SAND_SOURCE;
    while let Some(new_pos) = next_positions(pos)
        .into_iter()
        .find(|new_pos| map.tile(*new_pos).map_or(true, |occupied| !occupied))
    {
        pos = new_pos;
        if map.tile(new_pos).is_none() {
            // We are off the edge or bottom of map. All sand from now on will
            // fall into the abyss.
            return false;
        }
    }

    // The sand has settled.
    *map.tile_mut(pos).unwrap() = true;
    true
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input14.txt")?;

    let mut rocks = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let mut map_a = Map::new(&rocks)?;
    let result_a = std::iter::repeat(())
        .take_while(|_| drop_sand(&mut map_a))
        .count();
    println!("Day 14, part A: {}", result_a);

    // Add an "infinite" floor 2 tiles below the first map's bottom. In practice
    // we only need it to extend to either side by the new map's height,
    // excluding the floor.
    rocks.push(vec![
        Position {
            x: SAND_SOURCE.x - map_a.height() - 1,
            y: map_a.bottom + 2,
        },
        Position {
            x: SAND_SOURCE.x + map_a.height() + 1,
            y: map_a.bottom + 2,
        },
    ]);
    let mut map_b = Map::new(&rocks)?;
    let result_b = std::iter::repeat(())
        .take_while(|_| drop_sand(&mut map_b))
        .count();
    println!("Day 14, part B: {}", result_b);

    Ok(())
}
