use super::error::AocResult;

pub trait Day<T, U> {
    fn part_1(input_file_name: &str) -> AocResult<T>;
    fn part_2(input_file_name: &str) -> AocResult<U>;
}
