use crate::utilities::{day::Day, error::AocResult, files::read_lines_from_file};

pub struct DayThree;

impl Day<u64, u64> for DayThree {
    fn part_1(input_file_name: &str) -> AocResult<u64> {
        let lines = read_lines_from_file(input_file_name)?;
        let mut result: u64 = 0;

        for line in lines {
            let compartment_one = &line[0..(line.len() / 2)];
            let compartment_two = &line[(line.len() / 2)..];
            let items_one = process_compartment(compartment_one);
            let items_two = process_compartment(compartment_two);

            result += compare_compartment_items(items_one, items_two);
        }

        Ok(result)
    }

    fn part_2(input_file_name: &str) -> AocResult<u64> {
        let lines = read_lines_from_file(input_file_name)?;
        let mut elves: [u64; 3] = [0; 3];
        let mut result: u64 = 0;

        for (index, line) in lines.enumerate() {
            let items = process_compartment(line.as_str());
            let current_elf = index % 3;

            elves[current_elf] = items;

            if current_elf == 2 {
                result += f64::log2((elves[0] & elves[1] & elves[2]) as f64) as u64;
            }
        }

        Ok(result)
    }
}

fn process_compartment(compartment: &str) -> u64 {
    let mut output: u64 = 0;

    for item in compartment.chars() {
        if item.is_ascii_alphabetic() {
            output |= 1 << get_priority(item);
        }
    }

    output
}

fn compare_compartment_items(items_one: u64, items_two: u64) -> u64 {
    f64::log2((items_one & items_two) as f64) as u64
}

fn get_priority(item: char) -> u64 {
    if item.is_ascii_uppercase() {
        item as u64 - 'A' as u64 + 27
    } else {
        item as u64 - 'a' as u64 + 1
    }
}
