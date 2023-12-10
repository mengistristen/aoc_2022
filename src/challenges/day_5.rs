use std::collections::VecDeque;

use crate::utilities::{day::Day, error::AocResult, files::read_lines_from_file};

pub struct DayFive;

impl Day<String, String> for DayFive {
    fn part_1(input_file_name: &str) -> AocResult<String> {
        perform_rearrange(input_file_name, move_crates)
    }

    fn part_2(input_file_name: &str) -> AocResult<String> {
        perform_rearrange(input_file_name, move_multiple_crates)
    }
}

fn perform_rearrange(
    input_file_name: &str,
    move_fn: fn(&mut Stacks, usize, usize, usize),
) -> AocResult<String> {
    let mut lines = read_lines_from_file(input_file_name)?;
    let mut line = lines.next().unwrap();
    let num_stacks = line.len() / 4 + 1;
    let mut stacks = Stacks::new(num_stacks);

    while line.contains('[') {
        let chars = line.as_bytes();

        for stack_id in 0..num_stacks {
            let index = stack_id * 4 + 1;

            if chars[index] as char != ' ' {
                stacks.stack_crate(stack_id, chars[index] as char);
            }
        }

        line = lines.next().unwrap();
    }

    lines.next();

    for line in lines {
        let values: Vec<&str> = line.split(' ').collect();

        move_fn(
            &mut stacks,
            values[1].parse::<usize>()?,
            values[3].parse::<usize>()?,
            values[5].parse::<usize>()?,
        );
    }

    Ok(stacks.top_values())
}

fn move_crates(stacks: &mut Stacks, num: usize, from: usize, to: usize) {
    for _ in 0..num {
        let crate_id = stacks.unstack_crate(from - 1);

        stacks.stack_crate_front(to - 1, crate_id);
    }
}

fn move_multiple_crates(stacks: &mut Stacks, num: usize, from: usize, to: usize) {
    let mut crates: VecDeque<char> = VecDeque::new();

    for _ in 0..num {
        crates.push_front(stacks.unstack_crate(from - 1));
    }

    for crate_id in crates {
        stacks.stack_crate_front(to - 1, crate_id);
    }
}

struct Stacks {
    stacks: Vec<VecDeque<char>>,
}

impl Stacks {
    fn new(size: usize) -> Self {
        Self {
            stacks: vec![VecDeque::new(); size],
        }
    }

    fn stack_crate(&mut self, stack: usize, crate_id: char) {
        self.stacks[stack].push_back(crate_id);
    }

    fn stack_crate_front(&mut self, stack: usize, crate_id: char) {
        self.stacks[stack].push_front(crate_id);
    }

    fn unstack_crate(&mut self, stack: usize) -> char {
        self.stacks[stack].pop_front().unwrap()
    }

    fn top_values(&mut self) -> String {
        let mut result = String::new();

        for stack_id in 0..self.stacks.len() {
            result.push(self.unstack_crate(stack_id));
        }

        result
    }
}
