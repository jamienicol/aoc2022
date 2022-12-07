use anyhow::{Context, Result};
use std::cell::Cell;
use std::collections::HashMap;
use std::path::PathBuf;

type FileSystem = HashMap<PathBuf, Dir>;

#[derive(Debug)]
struct Dir {
    path: PathBuf,
    subdirs: Vec<String>,
    immediate_size: u32,
    cached_recursive_size: Cell<Option<u32>>,
}

impl Dir {
    fn new(path: PathBuf) -> Self {
        Dir {
            path,
            subdirs: Vec::new(),
            immediate_size: 0,
            cached_recursive_size: Cell::new(None),
        }
    }

    fn size(&self, fs: &FileSystem) -> u32 {
        self.cached_recursive_size.get().unwrap_or_else(|| {
            let size = self.immediate_size
                + self
                    .subdirs
                    .iter()
                    .map(|name| fs.get(&self.path.join(name)).unwrap().size(fs))
                    .sum::<u32>();
            self.cached_recursive_size.set(Some(size));
            size
        })
    }
}

fn parse_input(input: &str) -> Result<FileSystem> {
    let mut current_path = PathBuf::new();
    let mut is_ls_running = false;

    let mut fs = FileSystem::new();
    fs.insert(PathBuf::from("/"), Dir::new(PathBuf::from("/")));

    for line in input.lines() {
        if line.starts_with('$') {
            is_ls_running = false;

            if let Some((_, dir)) = line.split_once("$ cd ") {
                if dir.starts_with('/') {
                    current_path = PathBuf::from(dir);
                } else if dir == ".." {
                    current_path.pop();
                } else {
                    current_path.push(dir);
                }
            } else if line == "$ ls" {
                is_ls_running = true;
            }
        } else if is_ls_running {
            let current_dir = fs
                .get_mut(&current_path)
                .with_context(|| format!("{:?} is not a known dir", current_path))?;

            let (node_type, name) = line
                .split_once(' ')
                .with_context(|| format!("Unexpected ls output: {}", line))?;

            if node_type == "dir" {
                current_dir.subdirs.push(name.to_string());
                let new_path = current_path.join(name);
                fs.entry(new_path.clone())
                    .or_insert_with(|| Dir::new(new_path));
            } else {
                let size = node_type
                    .parse::<u32>()
                    .with_context(|| format!("Expected file size, got {}", node_type))?;
                current_dir.immediate_size += size;
            };
        }
    }

    Ok(fs)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("res/input07.txt")?;

    let fs = parse_input(&input).context("Error parsing input")?;

    let result_a = fs
        .values()
        .map(|dir| dir.size(&fs))
        .filter(|size| *size <= 100000)
        .sum::<u32>();
    println!("Day 7, part A: {}", result_a);

    let required = 30000000 - (70000000 - fs[&PathBuf::from("/")].size(&fs));

    let result_b = fs
        .values()
        .map(|dir| dir.size(&fs))
        .filter(|size| *size > required)
        .min()
        .context("Cannot find any directories of required size")?;
    println!("Day 7, part B: {}", result_b);

    Ok(())
}
