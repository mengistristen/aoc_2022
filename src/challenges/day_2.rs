use crate::utilities::{day::Day, error::AocResult, files::read_lines_from_file};
use std::fmt;
use std::{error::Error, fmt::Display};

pub struct DayTwo;

impl Day<u32, u32> for DayTwo {
    fn part_1(input_file_name: &str) -> AocResult<u32> {
        let lines = read_lines_from_file(input_file_name)?;
        let mut total: u32 = 0;

        for line in lines {
            let input = line.as_bytes();
            let opponent_play = input[0];
            let player_play = input[2];

            total += determine_score_by_play(
                get_opponent_value(opponent_play as char)?,
                get_player_value(player_play as char)?,
            );
        }

        Ok(total)
    }

    fn part_2(input_file_name: &str) -> AocResult<u32> {
        let lines = read_lines_from_file(input_file_name)?;
        let mut total: u32 = 0;

        for line in lines {
            let input = line.as_bytes();
            let opponent_play = input[0];
            let outcome = input[2];

            total += determine_score_by_outcome(
                get_opponent_value(opponent_play as char)?,
                get_outcome_value(outcome as char)?,
            );
        }

        Ok(total)
    }
}

#[derive(Debug)]
pub enum RockPaperScissorsError {
    InvalidPlay,
    InvalidOutcome,
}

impl Display for RockPaperScissorsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InvalidPlay => "a player played an invalid play",
                Self::InvalidOutcome => "invalid outcome was determined",
            }
        )
    }
}

impl Error for RockPaperScissorsError {}

fn get_opponent_value(choice: char) -> AocResult<u32> {
    match choice {
        'A' => Ok(1),
        'B' => Ok(2),
        'C' => Ok(3),
        _ => Err(Box::new(RockPaperScissorsError::InvalidPlay)),
    }
}

fn get_player_value(choice: char) -> AocResult<u32> {
    match choice {
        'X' => Ok(1),
        'Y' => Ok(2),
        'Z' => Ok(3),
        _ => Err(Box::new(RockPaperScissorsError::InvalidPlay)),
    }
}

fn get_outcome_value(choice: char) -> AocResult<u32> {
    match choice {
        'X' => Ok(1),
        'Y' => Ok(0),
        'Z' => Ok(2),
        _ => Err(Box::new(RockPaperScissorsError::InvalidOutcome)),
    }
}

fn determine_score_by_play(opponent_value: u32, player_value: u32) -> u32 {
    let outcome = (opponent_value + 3 - player_value) % 3;

    player_value + determine_outcome_score(outcome)
}

fn determine_score_by_outcome(opponent_value: u32, outcome: u32) -> u32 {
    let mut player_value = opponent_value + (3 - outcome);

    if player_value > 3 {
        player_value -= 3;
    }

    player_value + determine_outcome_score(outcome)
}

fn determine_outcome_score(outcome: u32) -> u32 {
    match outcome {
        0 => 3,
        1 => 0,
        2 => 6,
        _ => panic!("invalid outcome"),
    }
}
