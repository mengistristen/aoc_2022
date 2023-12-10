use crate::utilities::{day::Day, error::AocResult, files::read_lines_from_file};

pub struct DayNine;

impl Day<usize, usize> for DayNine {
    fn part_1(input_file_name: &str) -> AocResult<usize> {
        let lines = read_lines_from_file(input_file_name)?;
        let mut head = Position::new(0, 0);
        let mut tail = Position::new(0, 0);
        let mut visits: Vec<Position> = vec![];

        for line in lines {
            let input: Vec<&str> = line.split(' ').collect();

            for _ in 0..input[1].parse::<u32>().unwrap() {
                match input[0] {
                    "R" => head.0 += 1,
                    "L" => head.0 -= 1,
                    "U" => head.1 += 1,
                    "D" => head.1 -= 1,
                    _ => panic!("this shouldn't happen"),
                }

                tail.trail(&head);

                if !visits.contains(&tail) {
                    visits.push(tail.clone());
                }
            }
        }

        Ok(visits.len())
    }

    fn part_2(input_file_name: &str) -> AocResult<usize> {
        let lines = read_lines_from_file(input_file_name)?;
        let mut segments: Vec<Position> = vec![Position::new(0, 0); 10];
        let mut visits: Vec<Position> = vec![];

        for line in lines {
            let input: Vec<&str> = line.split(' ').collect();

            for _ in 0..input[1].parse::<u32>().unwrap() {
                for index in 0..segments.len() {
                    if index == 0 {
                        match input[0] {
                            "R" => segments[0].0 += 1,
                            "L" => segments[0].0 -= 1,
                            "U" => segments[0].1 += 1,
                            "D" => segments[0].1 -= 1,
                            _ => panic!("this shouldn't happen"),
                        }
                    } else {
                        let temp = segments[index - 1].clone();

                        segments[index].trail(&temp);
                    }
                }

                if !visits.contains(&segments[segments.len() - 1]) {
                    visits.push(segments[segments.len() - 1].clone());
                }

                //visualize(&segments);
            }
        }

        Ok(visits.len())
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Position(i32, i32);

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    fn trail(&mut self, other: &Self) {
        let difference = Self::new(other.0 - self.0, other.1 - self.1);

        match difference {
            Position(2, 2) => {
                self.0 += 1;
                self.1 += 1;
            }
            Position(2, -2) => {
                self.0 += 1;
                self.1 -= 1;
            }
            Position(-2, 2) => {
                self.0 -= 1;
                self.1 += 1;
            }
            Position(-2, -2) => {
                self.0 -= 1;
                self.1 -= 1;
            }
            Position(x, y) => {
                if x == 2 {
                    self.0 += 1;
                    self.1 += y;
                } else if x == -2 {
                    self.0 -= 1;
                    self.1 += y;
                } else if y == 2 {
                    self.1 += 1;
                    self.0 += x;
                } else if y == -2 {
                    self.1 -= 1;
                    self.0 += x;
                }
            }
        }
    }
}
