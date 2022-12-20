//Hint: This can be done much smarter by just counting cycles. No need to do this but yolo

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Instruction {
    NoOp,
    AddX(i32),
}

impl Instruction {
    fn from_line(line: String) -> Self {
        use Instruction::*;

        if line == "noop" {
            return NoOp;
        }

        assert!(line.starts_with("addx "));

        let value = &line[5..];

        let value = value.parse().expect("Can parse addx instruction");

        AddX(value)
    }
}

struct CPU {
    cycle: u32,
    occupied_until: u32,
    x: i32,
    crt_x: i32,

    current_instruction: Option<Instruction>,
    instructions: VecDeque<Instruction>,
}

#[derive(PartialEq)]
enum IsDone {
    Yes,
    No,
}

impl CPU {
    fn new(instructions: VecDeque<Instruction>) -> Self {
        Self {
            cycle: 1,
            occupied_until: 0,
            x: 1,
            crt_x: 0,
            current_instruction: None,
            instructions,
        }
    }

    fn should_yield(&self) -> bool {
        if self.cycle < 20 {
            return false;
        }

        ((self.cycle - 20) % 40) == 0
    }

    fn part1(&self) -> i32 {
        if !self.should_yield() {
            return 0;
        }

        return (self.cycle as i32) * self.x;
    }

    fn part2(&self) {
        if self.crt_x.abs_diff(self.x) <= 1 {
            print!("#");
        } else {
            print!(".");
        }
    }

    fn fetch_next_instruction(&mut self) {
        use Instruction::*;

        if self.cycle < self.occupied_until {
            return;
        }

        self.current_instruction = match self.instructions.pop_front() {
            None => None,
            Some(NoOp) => {
                self.occupied_until = self.cycle + 1;
                Some(NoOp)
            }
            Some(AddX(value)) => {
                self.occupied_until = self.cycle + 2;
                Some(AddX(value))
            }
        }
    }

    fn execute_current_instruction(&mut self) -> IsDone {
        use Instruction::*;

        if self.cycle < self.occupied_until {
            return IsDone::No;
        }

        match self.current_instruction {
            None => IsDone::Yes,
            Some(NoOp) => IsDone::No,
            Some(AddX(v)) => {
                self.x += v;
                IsDone::No
            }
        }
    }

    fn update_crt(&mut self) {
        self.crt_x += 1;
        if self.crt_x == 40 {
            print!("\n");
            self.crt_x = 0;
        }
    }

    fn run(&mut self) -> i32 {
        let mut result = 0;

        loop {
            //During cycle
            result += self.part1();
            self.part2();

            self.fetch_next_instruction();

            //End of cycle
            self.cycle += 1;
            self.update_crt();

            //After cycle
            if self.execute_current_instruction() == IsDone::Yes {
                break;
            }
        }

        result
    }
}

fn main() {
    let file = BufReader::new(File::open("./task.txt").expect("Can open input file"));
    let instructions = file.lines().flatten().map(Instruction::from_line).collect();

    let mut cpu = CPU::new(instructions);
    let answer = cpu.run();

    println!("Answer: {answer}");
}
