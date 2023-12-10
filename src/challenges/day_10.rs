use std::collections::VecDeque;

use crate::utilities::{day::Day, error::AocResult, files::read_lines_from_file};

pub struct DayTen;

impl Day<i32, String> for DayTen {
    fn part_1(input_file_name: &str) -> AocResult<i32> {
        let lines = read_lines_from_file(input_file_name)?;
        let mut command_queue: VecDeque<Command> = VecDeque::new();
        let mut signal_marker: i32 = 20;
        let mut reg: i32 = 1;
        let mut result: i32 = 0;
        let mut cycle: i32 = 1;

        for line in lines {
            let input: Vec<&str> = line.split(' ').collect();

            command_queue.push_front(Command::Noop);

            if input[0] == "addx" {
                command_queue.push_front(Command::Addx(input[1].parse::<i32>().unwrap()));
            }

            result += process_signal_strength(cycle, &reg, &mut signal_marker);

            if let Some(command) = command_queue.pop_back() {
                command.process(&mut reg);
            }

            cycle += 1;
        }

        for command in command_queue.iter().rev() {
            result += process_signal_strength(cycle, &reg, &mut signal_marker);
            command.process(&mut reg);
            cycle += 1;
        }

        Ok(result)
    }

    fn part_2(input_file_name: &str) -> AocResult<String> {
        let lines = read_lines_from_file(input_file_name)?;
        let mut command_queue: VecDeque<Command> = VecDeque::new();
        let mut pos: i32 = 1;
        let mut cycle: i32 = 1;
        let mut output: String = String::new();

        for line in lines {
            let input: Vec<&str> = line.split(' ').collect();

            command_queue.push_front(Command::Noop);

            if input[0] == "addx" {
                command_queue.push_front(Command::Addx(input[1].parse::<i32>().unwrap()));
            }

            draw_pixel(cycle, &pos, &mut output);

            if cycle % 40 == 0 {
                output.push('\n');
            }

            if let Some(command) = command_queue.pop_back() {
                command.process(&mut pos);
            }

            cycle += 1;
        }

        for command in command_queue.iter().rev() {
            draw_pixel(cycle, &pos, &mut output);

            if cycle % 40 == 0 {
                output.push('\n');
            }

            command.process(&mut pos);
            cycle += 1;
        }

        Ok(output)
    }
}

enum Command {
    Noop,
    Addx(i32),
}

impl Command {
    fn process(&self, reg: &mut i32) {
        match self {
            Self::Noop => (),
            Self::Addx(val) => {
                *reg += val;
            }
        };
    }
}

fn process_signal_strength(cycle: i32, reg: &i32, signal_marker: &mut i32) -> i32 {
    let mut result = 0;

    if cycle == *signal_marker {
        result = cycle * reg;
        *signal_marker += 40;
    }

    result
}

fn draw_pixel(cycle: i32, pos: &i32, output: &mut String) {
    let difference = (cycle - 1) % 40 - pos;

    if (-1..=1).contains(&difference) {
        output.push('#');
    } else {
        output.push('_');
    }
}
