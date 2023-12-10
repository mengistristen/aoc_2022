use crate::utilities::{day::Day, error::AocResult, files::read_lines_from_file};

pub struct DaySeven;

impl Day<u64, u64> for DaySeven {
    fn part_1(input_file_name: &str) -> AocResult<u64> {
        let mut lines = read_lines_from_file(input_file_name)?;
        let mut size = 0;
        let mut dirs: Vec<u64> = vec![];

        parse_directory(&mut lines, &mut dirs);

        for dir in dirs {
            if dir < 100000 {
                size += dir;
            }
        }

        Ok(size)
    }

    fn part_2(input_file_name: &str) -> AocResult<u64> {
        let mut lines = read_lines_from_file(input_file_name)?;
        let mut size: u64 = 0;
        let mut dirs: Vec<u64> = vec![];

        parse_directory(&mut lines, &mut dirs);

        dirs.sort();

        let used_space = dirs[dirs.len() - 1];
        let unused_space = 70000000 - used_space;

        for dir in dirs {
            if dir + unused_space > 30000000 {
                size = dir;
                break;
            }
        }

        Ok(size)
    }
}

fn parse_directory(iter: &mut impl Iterator<Item = String>, dirs: &mut Vec<u64>) -> u64 {
    let mut size = 0;

    loop {
        let line = iter.next();

        if line.is_none() {
            break;
        }

        if let Some(input) = line {
            let values: Vec<&str> = input.split(' ').collect();

            if values[0] == "$" {
                if values[1] == "cd" {
                    if values[2] == ".." {
                        break;
                    } else {
                        let nested_size = parse_directory(iter, dirs);

                        dirs.push(nested_size);

                        size += nested_size;
                    }
                } else {
                    continue;
                }
            } else if values[0] == "dir" {
                continue;
            } else {
                let file_size = values[0].parse::<u64>().unwrap();

                size += file_size;
            }
        } else {
            break;
        }
    }

    size
}
