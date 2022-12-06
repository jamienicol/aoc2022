use anyhow::{Context, Result};
use itertools::Itertools;

fn find_start_marker(input: &[char], marker_length: usize) -> Result<usize> {
    let position = input
        .windows(marker_length)
        .position(|chars| chars.iter().duplicates().next().is_none())
        .context(format!(
            "Cannot find {} unique consecutive characters",
            marker_length
        ))?;

    Ok(position + marker_length)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input06.txt")?;
    let input_chars = input.trim_end().chars().collect::<Vec<char>>();

    let result_a =
        find_start_marker(&input_chars, 4).context("Cannot find start-of-packet marker")?;
    println!("Day 6, part A: {}", result_a);

    let result_b =
        find_start_marker(&input_chars, 14).context("Cannot find start-of-message marker")?;
    println!("Day 6, part B: {}", result_b);

    Ok(())
}
