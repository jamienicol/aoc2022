use std::collections::HashSet;
use anyhow::Result;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect()
}

fn priority(c: char) -> u32 {
    match c {
        c if ('a'..='z').contains(&c) => c as u32 - 'a' as u32 + 1,
        c if ('A'..='Z').contains(&c) => c as u32 - 'A' as u32 + 27,
        _ => unreachable!(),
    }
}

fn part_a(rucksacks: &[Vec<char>]) -> u32 {
    rucksacks.iter().map(|rucksack| {
        let (first, second) = rucksack.split_at(rucksack.len() / 2);
        let set = first.iter().collect::<HashSet<_>>();

        let duplicate = second.iter().find(|item| set.contains(item)).unwrap();
        priority(*duplicate)
    }).sum()
}

fn part_b(rucksacks: &[Vec<char>]) -> u32 {
    rucksacks.chunks_exact(3).map(|group| {
        let sets = group.iter().map(|elf| elf.iter().cloned().collect::<HashSet<char>>()).collect::<Vec<_>>();
        let intersection = &(&sets[0] & &sets[1]) & &sets[2];

        priority(*intersection.iter().next().unwrap())
    }).sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input03.txt")?;

    let rucksacks = parse_input(&input);

    let result_a = part_a(&rucksacks);
    println!("Day 3, part A: {}", result_a);

    let result_b = part_b(&rucksacks);
    println!("Day 3, part B: {}", result_b);

    Ok(())
}
