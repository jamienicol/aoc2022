use anyhow::{anyhow, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, newline, satisfy},
    combinator::{map, map_res, opt, recognize},
    multi::{fold_many1, many_m_n, separated_list1},
    sequence::{pair, preceded, terminated, tuple},
    AsChar, IResult,
};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

type ValveId = [char; 2];

#[derive(Debug, Clone)]
struct Valve {
    id: [char; 2],
    flow_rate: usize,
    tunnels: Vec<ValveId>,
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(recognize(pair(opt(char('-')), digit1)), |s: &str| {
        s.parse::<usize>()
    })(input)
}

fn parse_valve_id(input: &str) -> IResult<&str, ValveId> {
    map(
        recognize(many_m_n(2, 2, satisfy(AsChar::is_alpha))),
        |s: &str| {
            let mut chars = s.chars();
            [chars.next().unwrap(), chars.next().unwrap()]
        },
    )(input)
}

fn parse_valve(input: &str) -> IResult<&str, Valve> {
    map(
        tuple((
            preceded(tag("Valve "), parse_valve_id),
            preceded(tag(" has flow rate="), parse_usize),
            preceded(
                tuple((
                    tag("; "),
                    alt((tag("tunnel leads "), tag("tunnels lead "))),
                    tag("to valve"),
                    opt(char('s')),
                    char(' '),
                )),
                separated_list1(tag(", "), parse_valve_id),
            ),
        )),
        |(id, flow_rate, tunnels)| Valve {
            id,
            flow_rate,
            tunnels,
        },
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, HashMap<ValveId, Valve>> {
    fold_many1(
        terminated(parse_valve, opt(newline)),
        HashMap::default,
        |mut acc, valve| {
            acc.insert(valve.id, valve);
            acc
        },
    )(input)
}

fn calc_distance(
    start: &ValveId,
    end: &ValveId,
    valves: &HashMap<ValveId, Valve>,
) -> Option<usize> {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: usize,
        pos: ValveId,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut open: BinaryHeap<State> = BinaryHeap::new();
    open.push(State {
        cost: 0,
        pos: *start,
    });
    let mut costs: HashMap<ValveId, usize> = HashMap::default();
    costs.insert(*start, 0);

    while let Some(State { cost, pos }) = open.pop() {
        if pos == *end {
            return Some(cost);
        }

        if cost > costs[&pos] {
            continue;
        }

        for neighbour in valves
            .get(&pos)
            .unwrap_or_else(|| panic!("Can't find valve {}{}", pos[0], pos[1]))
            .tunnels
            .iter()
        {
            let new_cost = cost + 1;

            if new_cost < *costs.entry(*neighbour).or_insert(usize::MAX) {
                *costs.get_mut(neighbour).unwrap() = new_cost;
                open.push(State {
                    cost: new_cost,
                    pos: *neighbour,
                })
            }
        }
    }

    None
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State<const N: usize> {
    current_pos: [ValveId; N],
    visited: Vec<ValveId>,
    time: [usize; N],
    score: usize,
}

impl<const N: usize> Ord for State<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl<const N: usize> PartialOrd for State<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> State<N> {
    fn new(start: ValveId, time: usize) -> Self {
        Self {
            current_pos: [start; N],
            visited: Vec::new(),
            time: [time; N],
            score: 0,
        }
    }

    fn not_visited<'a, 'b>(
        &'a self,
        valves: &'b HashMap<ValveId, Valve>,
    ) -> impl Iterator<Item = ValveId> + Clone + '_
    where
        'b: 'a,
    {
        valves.iter().filter_map(|(id, valve)| {
            if valve.flow_rate > 0 && !self.visited.contains(id) {
                Some(*id)
            } else {
                None
            }
        })
    }

    fn next_states<'a, 'b>(
        &'a self,
        valves: &'b HashMap<ValveId, Valve>,
        distances: &'a HashMap<(ValveId, ValveId), usize>,
    ) -> impl Iterator<Item = (Self, usize)> + '_
    where
        'b: 'a,
    {
        (0..N)
            .cartesian_product(self.not_visited(valves))
            .filter_map(|(i, next)| {
                let distance = distances[&(self.current_pos[i], next)];
                if self.time[i] > distance {
                    let mut new_state = self.clone();
                    new_state.time[i] -= distance + 1;
                    new_state.current_pos[i] = next;
                    new_state.visited.push(next);
                    new_state.visited.sort_unstable();
                    let score_increase = valves[&next].flow_rate * new_state.time[i];
                    Some((new_state, score_increase))
                } else {
                    None
                }
            })
    }

    fn potential_score(&self, valves: &HashMap<ValveId, Valve>) -> usize {
        (0..N)
            .flat_map(|i| {
                (0..=self.time[i])
                    .rev()
                    .step_by(2)
                    .zip(self.not_visited(valves).sorted_unstable_by(|a, b| b.cmp(a)))
                    .map(|(time, valve)| time * valves[&valve].flow_rate)
            })
            .sum()
    }
}

fn find_max_pressure_release<const N: usize>(
    time: usize,
    valves: &HashMap<ValveId, Valve>,
    distances: &HashMap<(ValveId, ValveId), usize>,
) -> usize {
    let initial_state = State::<N>::new(['A', 'A'], time);
    let mut open = BinaryHeap::new();
    open.push((initial_state.clone(), 0));

    let mut scores: HashMap<State<N>, usize> = HashMap::default();
    scores.insert(initial_state, 0);

    let mut paths = Vec::new();
    let mut max_score = 0;

    while let Some((state, score)) = open.pop() {
        if score < scores[&state] {
            continue;
        }
        paths.push(state.clone());
        max_score = max_score.max(score);
        for (new_state, score_increase) in state.next_states(valves, distances) {
            let new_score = score + score_increase;
            let potential = new_state.potential_score(valves);

            if new_score + potential > max_score
                && new_score > *scores.entry(new_state.clone()).or_insert(usize::MIN)
            {
                *scores.get_mut(&new_state).unwrap() = new_score;
                open.push((new_state.clone(), new_score));
            }
        }
    }

    max_score
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input16.txt")?;

    let valves = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let distances =
        valves
            .values()
            .tuple_combinations()
            .fold(HashMap::new(), |mut acc, (from, to)| {
                if let Some(cost) = calc_distance(&from.id, &to.id, &valves) {
                    acc.insert((from.id, to.id), cost);
                    acc.insert((to.id, from.id), cost);
                }
                acc
            });

    let result_a = find_max_pressure_release::<1>(30, &valves, &distances);
    println!("Day 16, part A: {}", result_a);

    let result_b = find_max_pressure_release::<2>(26, &valves, &distances);
    println!("Day 16, part B: {}", result_b);

    Ok(())
}
