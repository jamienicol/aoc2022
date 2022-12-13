use advent_of_code_ocr::parse_string_to_letters;
use anyhow::{anyhow, Context, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, newline, space1},
    combinator::{map, map_res, opt, recognize},
    multi::separated_list1,
    sequence::{pair, separated_pair},
    IResult,
};

#[derive(Debug, Clone)]
enum Instr {
    Noop,
    Addx(isize),
}

impl Instr {
    fn cycles(&self) -> usize {
        match self {
            Instr::Noop => 1,
            Instr::Addx(_) => 2,
        }
    }
}

#[derive(Debug, Clone)]
struct Cpu {
    cycle_count: usize,
    x: isize,
    program: Vec<Instr>,
    pc: usize,
    instr_remaining_cycles: usize,
}

impl Cpu {
    fn new(program: Vec<Instr>) -> Self {
        Cpu {
            cycle_count: 0,
            x: 1,
            program,
            pc: 0,
            instr_remaining_cycles: 0,
        }
    }

    fn tick(&mut self) -> Option<CpuState> {
        self.program.get(self.pc).map(|instr| {
            self.cycle_count += 1;

            if self.instr_remaining_cycles == 0 {
                self.instr_remaining_cycles = instr.cycles() - 1;
            } else {
                self.instr_remaining_cycles -= 1;
            }

            let state = CpuState {
                cycle: self.cycle_count,
                x: self.x,
                signal_strength: self.cycle_count as isize * self.x,
            };

            if self.instr_remaining_cycles == 0 {
                match instr {
                    Instr::Noop => {}
                    Instr::Addx(val) => self.x += val,
                }
                self.pc += 1;
            }

            state
        })
    }

    fn iter(self) -> CpuIter {
        CpuIter { cpu: self }
    }
}

struct CpuState {
    cycle: usize,
    x: isize,
    signal_strength: isize,
}

struct CpuIter {
    cpu: Cpu,
}

impl Iterator for CpuIter {
    type Item = CpuState;

    fn next(&mut self) -> Option<Self::Item> {
        self.cpu.tick()
    }
}

fn parse_input(input: &str) -> IResult<&str, Vec<Instr>> {
    separated_list1(
        newline,
        alt((
            map(tag("noop"), |_| Instr::Noop),
            map(
                separated_pair(
                    tag("addx"),
                    space1,
                    map_res(recognize(pair(opt(char('-')), digit1)), |s: &str| {
                        s.parse::<isize>()
                            .with_context(|| format!("Error parsing addx argument {:?}", s))
                    }),
                ),
                |(_, val)| Instr::Addx(val),
            ),
        )),
    )(input)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input10.txt")?;

    let instructions = parse_input(&input)
        .map_err(|e| anyhow!("Error parsing input: {:?}", e))?
        .1;

    let cpu = Cpu::new(instructions);

    let result_a = cpu
        .clone()
        .iter()
        .filter_map(|state| {
            if state.cycle == 20
                || state.cycle == 60
                || state.cycle == 100
                || state.cycle == 140
                || state.cycle == 180
                || state.cycle == 220
            {
                Some(state.signal_strength)
            } else {
                None
            }
        })
        .sum::<isize>();
    println!("Day 10, part A: {}", result_a);

    let mut pixels = [[false; 40]; 6];
    for state in cpu.iter() {
        let y = (state.cycle - 1) / 40;
        let x = (state.cycle - 1) % 40;

        if ((state.x - 1)..=(state.x + 1)).contains(&(x as isize)) {
            pixels[y][x] = true;
        }
    }

    let mut display = String::new();
    for row in pixels {
        for pixel in row {
            display.push(match pixel {
                true => '#',
                false => '.',
            });
        }
        display.push('\n');
    }
    let result_b = parse_string_to_letters(&display);
    print!("Day 10, part B: {}", result_b);

    Ok(())
}
