#![feature(map_first_last)]
use std::collections::BTreeSet;
use util;

#[derive(Clone)]
struct Line {
    op: String,
    arg: i32,
}

#[derive(Clone)]
struct Program {
    lines: Vec<Line>,
    visited: BTreeSet<i32>,
    i: i32,
    acc: i32,
}

struct Result {
    completed: bool,
    acc: i32,
}

impl Program {
    const NOP: &'static str = "nop";
    const JMP: &'static str = "jmp";
    const ACC: &'static str = "acc";
    
    fn new(input: &str) -> Program {
        Program {
            lines: input.lines()
                .map(|l| {
                    let mut parts = l.split(' ');
                    Line {
                        op: parts.next().unwrap().to_string(),
                        arg: parts.next().unwrap().parse().unwrap()
                    }
                })
                .collect(),
            visited: BTreeSet::new(),
            i: 0,
            acc: 0,
        }
    }

    fn step(&mut self) {
        self.visited.insert(self.i);

        let line = &self.lines[self.i as usize];

        match line.op.as_ref() {
            Program::NOP => self.i += 1,
            Program::JMP => self.i += line.arg,
            Program::ACC => {
                self.acc += line.arg;
                self.i += 1;
            },
            _ => panic!("Invalid op")
        }
    }

    fn lines(&self) -> &Vec<Line> {
        &self.lines
    }

    fn set_op(&mut self, i: i32, op: String) {
        self.lines[i as usize].op = op;
    }

    fn run(&mut self) -> Result {
        loop {
            self.step();

            if self.visited.contains(&self.i) {
                return Result { completed: false, acc: self.acc };
            }

            if self.i >= self.lines.len() as i32 {
                return Result { completed: true, acc: self.acc };
            }
        }
    }
}

fn part1(program: &mut Program) -> i32 {
    program.run().acc
}

fn part2(program: &Program) -> i32 {
    for (i, line) in program.lines().iter().enumerate() {
        let mut copy: Program;
        match line.op.as_ref() {
            Program::JMP => {
                let mut new = program.clone();
                new.set_op(i as i32, Program::NOP.to_string());
                copy = new;
            },
            Program::NOP => {
                let mut new = program.clone();
                new.set_op(i as i32, Program::JMP.to_string());
                copy = new;
            },
            _ => continue,
        }
        
        let result = copy.run();
        if result.completed {
            return result.acc;
        }
    }
    
    0
}

fn main() {
    let input = util::file_to_string("input/8");

    println!("Part 1: {}", part1(&mut Program::new(&input)));
    println!("Part 2: {}", part2(&Program::new(&input)));
}