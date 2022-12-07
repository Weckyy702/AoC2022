use std::{
    collections::VecDeque,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

type Crate = char;
type Stack = VecDeque<Crate>;

#[derive(Debug)]
struct Step {
    quantity: usize,
    from: usize,
    to: usize,
}

fn apply_step(stacks: &mut Vec<Stack>, step: &Step) {
    let mut from = stacks[step.from].clone();
    let mut to = stacks[step.to].clone();

    (0..step.quantity)
        .map(|_| from.pop_front().unwrap())
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .for_each(|&elem| {
            to.push_front(elem)
        });

    stacks[step.from] = from;
    stacks[step.to] = to;
}

fn apply_steps(mut stacks: Vec<Stack>, steps: Vec<Step>) -> String {
    steps.iter().for_each(|step| {
        println!("{stacks:?}");
        apply_step(&mut stacks, step);
    });
    stacks
        .iter_mut()
        .map(|stack| stack.pop_front().unwrap_or(' '))
        .collect::<String>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = BufReader::new(File::open("task.txt")?);
    let lines = file.lines().flatten().collect::<Vec<_>>();

    let drawing = lines
        .iter()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let num_columns = drawing
        .last()
        .unwrap()
        .chars()
        .filter(|c| c.is_numeric())
        .count();

    let num_rows = drawing.len() - 1;

    let num_chars_per_row = 4 * num_columns - 1;

    let steps = lines
        .iter()
        .skip(num_rows + 2)
        .map(|line| {
            let [quantity, from, to] = line
                .split_ascii_whitespace()
                .enumerate()
                .flat_map(|(idx, chunk)| match idx {
                    1 | 3 | 5 => chunk.parse::<usize>().ok(),
                    _ => None,
                })
                .collect::<Vec<_>>()[..3] else {
                    unreachable!()
                };
            Step {
                quantity,
                from: from - 1,
                to: to - 1,
            }
        })
        .collect::<Vec<_>>();

    println!(
        "Parsing {}x{} Image ({} chars per row)",
        num_columns, num_rows, num_chars_per_row
    );

    drawing.iter().for_each(|line| {
        assert!(line.len() == num_chars_per_row);
    });

    let stacks = drawing
        .iter()
        .take(num_rows)
        .map(|line| {
            line.chars()
                .enumerate()
                .filter(|(idx, _)| idx % 4 == 1)
                .map(|(_, c)| if c != ' ' { Some(c) } else { None })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let stacks = (0..num_columns)
        .map(|i| {
            let crates = (0..num_rows)
                .flat_map(|j| *stacks.get(j).unwrap().get(i).unwrap())
                .collect();
            crates
        })
        .collect();

    let result = apply_steps(stacks, steps);

    println!("{result}");

    Ok(())
}
