use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = BufReader::new(File::open("test.txt")?);

    let lines = file.lines().flatten().collect::<Vec<_>>();

    let mut elves = lines
        .split(|line| line.is_empty())
        .map(|chunk| {
            chunk
                .iter()
                .flat_map(|line| line.parse::<u32>())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    let max = elves.iter().max().unwrap().clone();

    elves.sort_unstable();

    let max_sum = elves.iter().rev().take(3).sum::<u32>();

    println!("{max} {max_sum}");

    Ok(())
}
