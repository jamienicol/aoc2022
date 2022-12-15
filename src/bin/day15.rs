use anyhow::{anyhow, Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, newline},
    combinator::{cut, map, map_res, opt, recognize},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};
use std::{collections::HashSet, ops::RangeInclusive};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn dist(&self, other: &Position) -> isize {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

#[derive(Debug, Clone)]
struct Sensor {
    pos: Position,
    nearest_beacon: Position,
}

fn ranges_overlap(first: &RangeInclusive<isize>, second: &RangeInclusive<isize>) -> bool {
    first.start() <= second.end() && second.start() <= first.end()
}

fn merge_ranges(
    first: &RangeInclusive<isize>,
    second: &RangeInclusive<isize>,
) -> Option<RangeInclusive<isize>> {
    if ranges_overlap(first, second) {
        Some(*(first.start().min(second.start()))..=*(first.end().max(second.end())))
    } else {
        None
    }
}

#[derive(Debug)]
struct RangeSet(Vec<RangeInclusive<isize>>);

impl RangeSet {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn add(&mut self, mut new: RangeInclusive<isize>) {
        self.0.retain(|r| match merge_ranges(r, &new) {
            Some(merged) => {
                new = merged;
                false
            }
            None => true,
        });
        self.0.push(new);
        self.0.sort_by(|a, b| a.start().cmp(b.start()));
    }
}

fn parse_isize(input: &str) -> IResult<&str, isize> {
    map_res(recognize(pair(opt(char('-')), digit1)), |s: &str| {
        s.parse::<isize>()
    })(input)
}

fn parse_position(input: &str) -> IResult<&str, Position> {
    map(
        separated_pair(
            preceded(tag("x="), parse_isize),
            tag(", "),
            preceded(tag("y="), parse_isize),
        ),
        |(x, y)| Position { x, y },
    )(input)
}

fn parse_sensor(input: &str) -> IResult<&str, Sensor> {
    map(
        pair(
            preceded(tag("Sensor at "), parse_position),
            preceded(tag(": closest beacon is at "), parse_position),
        ),
        |(pos, nearest_beacon)| Sensor {
            pos,
            nearest_beacon,
        },
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<Sensor>> {
    cut(separated_list1(newline, parse_sensor))(input.trim_end())
}

fn part_a(sensors: &[Sensor]) -> isize {
    const ROW: isize = 2000000;

    let mut beacons = HashSet::new();
    for sensor in sensors {
        if sensor.nearest_beacon.y == ROW {
            beacons.insert(sensor.nearest_beacon.x);
        }
    }

    let mut not_beacons = RangeSet::new();

    for sensor in sensors {
        let beacon_dist = sensor.pos.dist(&sensor.nearest_beacon);
        let vertical_dist = (ROW - sensor.pos.y).abs();
        if beacon_dist - vertical_dist >= 0 {
            let first = sensor.pos.x - (beacon_dist - vertical_dist);
            let last = sensor.pos.x + (beacon_dist - vertical_dist);

            not_beacons.add(first..=last);
        }
    }

    not_beacons
        .0
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum::<isize>()
        - beacons.len() as isize
}

fn part_b(sensors: &[Sensor]) -> Result<isize> {
    const SEARCH_AREA: isize = 4000000;

    for y in 0..=SEARCH_AREA {
        let mut not_beacons = RangeSet::new();
        for sensor in sensors {
            let beacon_dist = sensor.pos.dist(&sensor.nearest_beacon);
            let vertical_dist = (y - sensor.pos.y).abs();
            if beacon_dist - vertical_dist >= 0 {
                let first = (sensor.pos.x - (beacon_dist - vertical_dist)).max(0);
                let last = (sensor.pos.x + (beacon_dist - vertical_dist)).min(SEARCH_AREA);

                not_beacons.add(first..=last);
            }
        }
        if not_beacons.0.len() > 1 {
            return Ok((not_beacons.0[0].end() + 1) * 4000000 + y);
        }
    }

    Err(anyhow!("Failed to find beacon"))
}

/// Alternative solution for part B
fn part_b_2(sensors: &[Sensor]) -> Result<isize> {
    const SEARCH_AREA: isize = 4000000;

    // Find all positions directly adjacent to the exclusion zone around each sensor.
    let mut adjacent_positions = sensors
        .iter()
        .flat_map(|sensor| {
            let beacon_dist = sensor.pos.dist(&sensor.nearest_beacon);
            let y_range = (sensor.pos.y - beacon_dist)..=(sensor.pos.y + beacon_dist);
            y_range.flat_map(move |y| {
                let vertical_dist = (y - sensor.pos.y).abs();
                [
                    Position {
                        x: sensor.pos.x - (beacon_dist - vertical_dist) - 1,
                        y,
                    },
                    Position {
                        x: sensor.pos.x + (beacon_dist - vertical_dist) + 1,
                        y,
                    },
                ]
            })
        })
        .filter(|pos| pos.x >= 0 && pos.y >= 0 && pos.x <= SEARCH_AREA && pos.y <= SEARCH_AREA);

    // Find which of these positions isn't in the exclusion zone of any other sensor.
    let beacon = adjacent_positions
        .find(|pos| {
            sensors
                .iter()
                .all(|sensor| sensor.pos.dist(pos) > sensor.pos.dist(&sensor.nearest_beacon))
        })
        .context("Failed to find beacon")?;

    Ok(beacon.x * 4000000 + beacon.y)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input15.txt")?;

    let sensors = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let result_a = part_a(&sensors);
    println!("Day 15, part A: {}", result_a);

    let result_b = part_b(&sensors)?;
    println!("Day 15, part B: {}", result_b);

    let result_b_2 = part_b_2(&sensors)?;
    println!("Day 15, part B: {}", result_b_2);

    Ok(())
}
