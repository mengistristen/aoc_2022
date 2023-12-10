use aoc_2022::{
    challenges::day_8::DayEight,
    utilities::{day::Day, runner::AocRunner},
};

fn main() {
    AocRunner::run(
        DayEight::part_1("./examples/input/day_8_example.txt"),
        "Example output: ",
    );
    AocRunner::run(
        DayEight::part_1("./examples/input/day_8_part_1.txt"),
        "Number of visible trees: ",
    );
    AocRunner::run(
        DayEight::part_2("./examples/input/day_8_example.txt"),
        "Example output: ",
    );
    AocRunner::run(
        DayEight::part_2("./examples/input/day_8_part_1.txt"),
        "Highest scenic score: ",
    );
}
