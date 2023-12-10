use crate::utilities::{day::Day, error::AocResult, files::read_lines_from_file};

pub struct DaySix;

impl Day<usize, usize> for DaySix {
    fn part_1(input_file_name: &str) -> AocResult<usize> {
        let mut lines = read_lines_from_file(input_file_name)?;
        let line = lines.next().unwrap();

        Ok(find_marker(line.as_str(), 4))
    }

    fn part_2(input_file_name: &str) -> AocResult<usize> {
        let mut lines = read_lines_from_file(input_file_name)?;
        let line = lines.next().unwrap();

        Ok(find_marker(line.as_str(), 14))
    }
}

fn find_marker(input: &str, buffer_size: usize) -> usize {
    let mut buffer: Vec<char> = vec!['\0'; buffer_size];
    let mut loc: usize = 0;

    for (index, char) in input.chars().enumerate() {
        buffer[index % buffer_size] = char;

        if index > 2 && determine_unique(&buffer) {
            loc = index + 1;
            break;
        }
    }

    loc
}

fn determine_unique(buffer: &Vec<char>) -> bool {
    let mut combined: u32 = 0;
    let mut set_bits: usize = 0;

    for char in buffer {
        if char.is_ascii_lowercase() {
            combined |= 1 << (*char as u8 - b'a');
        }
    }

    for bit in 0..32 {
        if combined & (1 << bit) != 0 {
            set_bits += 1;
        }
    }

    set_bits == buffer.len()
}
