use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn max(&self, other: &Self) -> Self {
        Position {
            x: i32::max(self.x, other.x),
            y: i32::max(self.y, other.y),
        }
    }

    fn min(&self, other: &Self) -> Self {
        Position {
            x: i32::min(self.x, other.x),
            y: i32::min(self.y, other.y),
        }
    }
}

#[derive(Debug)]
struct State<const N: usize> {
    knots: [Position; N],
    visited: HashSet<Position>,
    top_right_bound: Position,
    bottom_left_bound: Position,
}

impl<const N: usize> Default for State<N> {
    fn default() -> Self {
        State {
            knots: [Position::new(); N],
            visited: HashSet::from([Position::new()]),
            top_right_bound: Position::new(),
            bottom_left_bound: Position::new(),
        }
    }
}

impl<const N: usize> State<N> {
    fn head(&self) -> &Position {
        &self.knots[0]
    }

    fn tail(&self) -> &Position {
        &self.knots[N - 1]
    }

    fn update_head(&mut self, dx: i32, dy: i32) {
        self.knots[0].x += dx;
        self.knots[0].y += dy;
    }
}

struct Move(i32, i32);

fn parse_move(line: String) -> Move {
    let [direction, amount] = line.splitn(2, ' ').collect::<Vec<_>>()[..2] else { panic!("Wrong format in line!") };
    let amount = amount.parse().expect("Can parse amount");

    match direction.chars().nth(0).unwrap() {
        'R' => Move(amount, 0),
        'L' => Move(-amount, 0),
        'U' => Move(0, -amount),
        'D' => Move(0, amount),
        _ => unimplemented!("Unknown direction character!"),
    }
}

fn is_touching(head: &Position, tail: &Position) -> bool {
    if head.x.abs_diff(tail.x) > 1 {
        return false;
    }
    if head.y.abs_diff(tail.y) > 1 {
        return false;
    }
    true
}

fn update_knot(head: &Position, mut tail: Position) -> Position {
    if is_touching(head, &tail) {
        return tail;
    }

    let x_diff = head.x - tail.x;
    let step = x_diff.signum();
    tail.x += step;

    let y_diff = head.y - tail.y;
    let step = y_diff.signum();
    tail.y += step;

    tail
}

fn update_knots<const N: usize>(state: &mut State<N>) {
    for i in 1..N {
        state.knots[i] = update_knot(&state.knots[i - 1], state.knots[i]);
    }
}

fn update<const N: usize>(old: State<N>, dx: i32, dy: i32) -> State<N> {
    let mut new = old;

    //Where to go on x/y each step (on of these is always zero)
    let sx = dx.signum();
    let sy = dy.signum();

    //How many steps to move
    let num_steps = dx.abs().max(dy.abs());

    for _ in 0..num_steps {
        new.update_head(sx, sy);

        update_knots(&mut new);

        new.visited.insert(*new.tail());
    }

    //Update the bounds
    new.top_right_bound = new.top_right_bound.max(new.head()).max(new.tail());
    new.bottom_left_bound = new.bottom_left_bound.min(new.head()).min(new.tail());

    new
}

fn knot_name(index: usize) -> char {
    if index == 0 {
        return 'H';
    }
    ('0' as u8 + index as u8) as char
}

fn show_state<const N: usize>(state: &State<N>) {
    let Position { x: min_x, y: min_y } = state.bottom_left_bound;
    let Position { x: max_x, y: max_y } = state.top_right_bound;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = Position { x, y };

            let has_printed = (|| {
                for (index, knot) in state.knots.iter().enumerate() {
                    if pos == *knot {
                        print!("{}", knot_name(index));
                        return true;
                    }
                }
                false
            })();

            if has_printed {
                continue;
            }

            if state.visited.contains(&pos) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("---------");
}

fn main() {
    let file = BufReader::new(File::open("./task.txt").expect("Can open input file"));

    let moves = file.lines().flatten().map(parse_move).collect::<Vec<_>>();

    let mut state = State::<50>::default();

    for Move(dx, dy) in moves {
        state = update(state, dx, dy);
    }

    show_state(&state);
    println!("visited {} points", state.visited.len());
}
