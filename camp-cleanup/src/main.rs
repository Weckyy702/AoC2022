use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
};

trait RangeExt {
    fn contains_range(&self, other: &Self) -> bool;

    fn overlaps(&self, other: &Self) -> bool;
}

impl<I: PartialOrd> RangeExt for RangeInclusive<I> {
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(&other.start()) & self.contains(&other.end())
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.start())
            | self.contains(other.end())
            | other.contains(self.start())
            | other.contains(self.end())
    }
}

fn parse_to_range(str: &str) -> Option<RangeInclusive<u32>> {
    let [start, end] = str.split('-').collect::<Vec<_>>()[..2] else { return None };

    let start = start.parse().ok()?;
    let end = end.parse().ok()?;

    Some(start..=end)
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = BufReader::new(File::open("./task.txt")?);

    let pairs = file.lines().flatten().map(|line|{
        let [first, second] = &line.split(',').flat_map(parse_to_range).collect::<Vec<_>>()[..2] else { unreachable!() };

        (first.clone(), second.clone())
    }).collect::<Vec<_>>();

    let num_fully_overlapping_pairs = pairs
        .iter()
        .filter(|(a, b)| a.contains_range(&b) | b.contains_range(&a))
        .count();

    let num_overlapping_pairs = pairs.iter().filter(|(a, b)| a.overlaps(&b)).count();

    println!("{num_fully_overlapping_pairs} {num_overlapping_pairs}");

    Ok(())
}
