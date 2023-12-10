use aoc_2022::{
    challenges::day_5::DayFive,
    utilities::{day::Day, runner::AocRunner},
};

fn main() {
    AocRunner::run(
        DayFive::part_1("./examples/input/day_5_example.txt"),
        "Example output: ",
    );
    AocRunner::run(
        DayFive::part_1("./examples/input/day_5_part_1.txt"),
        "Top crates: ",
    );
    AocRunner::run(
        DayFive::part_2("./examples/input/day_5_example.txt"),
        "Example output: ",
    );
    AocRunner::run(
        DayFive::part_2("./examples/input/day_5_part_1.txt"),
        "Top crates with CrateMover 9001: ",
    );
}
