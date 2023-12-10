use std::collections::VecDeque;

use crate::utilities::{day::Day, error::AocResult, files::read_lines_from_file};

pub struct DayEleven;

impl Day<u64, u64> for DayEleven {
    fn part_1(input_file_name: &str) -> AocResult<u64> {
        let mut lines = read_lines_from_file(input_file_name)?.peekable();
        let mut monkeys: Vec<Monkey> = vec![];

        while lines.peek().is_some() {
            monkeys.push(parse_monkey(&mut lines));
        }

        Ok(process(&mut monkeys, 20, &|val| val / 3))
    }

    fn part_2(input_file_name: &str) -> AocResult<u64> {
        let mut lines = read_lines_from_file(input_file_name)?.peekable();
        let mut monkeys: Vec<Monkey> = vec![];

        while lines.peek().is_some() {
            monkeys.push(parse_monkey(&mut lines));
        }

        let mut lcm = 1;

        for monkey in &monkeys {
            lcm *= monkey.test;
        }

        Ok(process(&mut monkeys, 10000, &|val| val % lcm))
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: String,
    test: u64,
    if_true: usize,
    if_false: usize,
    activity: u64,
}

impl Monkey {
    fn new() -> Self {
        Self {
            items: VecDeque::new(),
            operation: String::from(""),
            test: 0,
            if_true: 0,
            if_false: 0,
            activity: 0,
        }
    }

    fn inspect_items(&mut self, adjust: &dyn Fn(u64) -> u64) -> Vec<(u64, usize)> {
        let mut result: Vec<(u64, usize)> = vec![];

        self.activity += self.items.len() as u64;

        while let Some(item) = self.items.pop_front() {
            let res = adjust(self.execute_operation(item));

            if res % self.test == 0 {
                result.push((res, self.if_true));
            } else {
                result.push((res, self.if_false));
            }
        }

        result
    }

    fn execute_operation(&self, old: u64) -> u64 {
        let op: Vec<&str> = self.operation.split(' ').collect();
        let right = if op[1] == "old" {
            old
        } else {
            op[1].parse::<u64>().unwrap()
        };

        match op[0] {
            "*" => old * right,
            "+" => old + right,
            _ => panic!("Invalid operation"),
        }
    }
}

fn process(monkeys: &mut Vec<Monkey>, rounds: usize, adjust: &dyn Fn(u64) -> u64) -> u64 {
    for _ in 0..rounds {
        for current in 0..monkeys.len() {
            let moves = monkeys[current].inspect_items(adjust);

            for item_move in moves {
                monkeys[item_move.1].items.push_back(item_move.0);
            }
        }
    }

    monkeys.sort_by(|a, b| a.activity.cmp(&b.activity));

    monkeys[monkeys.len() - 1].activity * monkeys[monkeys.len() - 2].activity
}

fn parse_monkey(iter: &mut impl Iterator<Item = String>) -> Monkey {
    let mut monkey: Monkey = Monkey::new();

    iter.next();

    let items_str = &iter.next().unwrap()[18..];

    let mut items_vec: VecDeque<u64> = items_str
        .split(", ")
        .map(|item| item.parse::<u64>().unwrap())
        .collect();

    monkey.items.append(&mut items_vec);
    monkey.operation = String::from(&iter.next().unwrap()[23..]);
    monkey.test = (iter.next().unwrap()[21..]).parse::<u64>().unwrap();
    monkey.if_true = (iter.next().unwrap()[29..]).parse::<usize>().unwrap();
    monkey.if_false = (iter.next().unwrap()[30..]).parse::<usize>().unwrap();

    iter.next();

    monkey
}
