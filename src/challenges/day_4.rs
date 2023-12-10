use crate::utilities::{day::Day, error::AocResult, files::read_lines_from_file};

pub struct DayFour;

impl Day<u32, u32> for DayFour {
    fn part_1(input_file_name: &str) -> AocResult<u32> {
        let lines = read_lines_from_file(input_file_name)?;
        let mut result: u32 = 0;

        for line in lines {
            let assignments = parse_assignments(line.as_str())?;

            if assignments.0.contains(&assignments.1) || assignments.1.contains(&assignments.0) {
                result += 1;
            }
        }

        Ok(result)
    }

    fn part_2(input_file_name: &str) -> AocResult<u32> {
        let lines = read_lines_from_file(input_file_name)?;
        let mut result: u32 = 0;

        for line in lines {
            let assignments = parse_assignments(line.as_str())?;

            if assignments.0.overlaps(&assignments.1) {
                result += 1;
            }
        }

        Ok(result)
    }
}

struct Assignment(u32, u32);

impl Assignment {
    fn from_str(input: &str) -> AocResult<Self> {
        let range: Vec<&str> = input.split('-').collect();

        Ok(Self(range[0].parse::<u32>()?, range[1].parse::<u32>()?))
    }

    fn min(&self) -> u32 {
        self.0
    }

    fn max(&self) -> u32 {
        self.1
    }

    fn contains(&self, other: &Self) -> bool {
        other.min() >= self.min() && other.max() <= self.max()
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.max() >= other.min() && self.min() <= other.max()
    }
}

fn parse_assignments(input: &str) -> AocResult<(Assignment, Assignment)> {
    let assignments: Vec<&str> = input.split(',').collect();
    Ok((
        Assignment::from_str(assignments[0])?,
        Assignment::from_str(assignments[1])?,
    ))
}
