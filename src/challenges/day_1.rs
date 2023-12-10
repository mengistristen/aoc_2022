use crate::utilities::day::Day;
use crate::utilities::error::AocResult;
use crate::utilities::files::read_lines_from_file;

pub struct DayOne;

impl Day<u32, u32> for DayOne {
    fn part_1(input_file_name: &str) -> AocResult<u32> {
        track_highest_calories(input_file_name, 1)
    }

    fn part_2(input_file_name: &str) -> AocResult<u32> {
        track_highest_calories(input_file_name, 3)
    }
}

fn track_highest_calories(file_name: &str, leaderboard_size: usize) -> AocResult<u32> {
    let mut leaderboard = vec![0; leaderboard_size];
    let mut value = 0;
    let mut lines = read_lines_from_file(file_name)?;
    let mut line;

    loop {
        line = lines.next();

        if let Some(data) = line { 
            if data.is_empty() {
                adjust_leaderboard(&mut leaderboard, value);
                value = 0;
            } else {
                value += data.parse::<u32>()?;
            }
        } else {
            adjust_leaderboard(&mut leaderboard, value);
            break;
        }     
    }

    Ok(leaderboard.into_iter().sum())
}

fn adjust_leaderboard(leaderboard: &mut [u32], value: u32) {
    let mut current = value;

    for item in leaderboard.iter_mut() {
        if current > *item {
            current += *item;
            *item = current - *item;
            current -= *item;
        }
    }
}
