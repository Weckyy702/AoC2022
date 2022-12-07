#![feature(array_chunks)]

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

trait Priority {
    fn priority(&self) -> u32;
}

impl Priority for char {
    fn priority(&self) -> u32 {
        match self {
            'a'..='z' => *self as u32 - ('a' as u32) + 1,
            'A'..='Z' => *self as u32 - ('A' as u32) + 27,
            _ => panic!("Unexpected item type!"),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = BufReader::new(File::open("./task.txt")?);

    let lines = file.lines().flatten().collect::<Vec<_>>();

    let priority_sum = lines
        .iter()
        .map(|line| {
            let (first, second) = line.split_at(line.len() / 2);

            let Some(index) = first.find(|c| {
            second.contains(c)
        }) else { panic!() };

            first.chars().nth(index).unwrap().priority()
        })
        .sum::<u32>();

    let group_sum = lines
        .array_chunks::<3>()
        .flat_map(|[first, second, third]| {
            first
                .chars()
                .filter(|&c| second.contains(c) && third.contains(c))
                .nth(0)
        })
        .map(|c| c.priority())
        .sum::<u32>();

    println!("{priority_sum} {group_sum}");

    Ok(())
}
