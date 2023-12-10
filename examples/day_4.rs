use aoc_2022::{
    challenges::day_4::DayFour,
    utilities::{day::Day, runner::AocRunner},
};

fn main() {
    AocRunner::run(
        DayFour::part_1("./examples/input/day_4_example.txt"),
        "Example output: ",
    );
    AocRunner::run(
        DayFour::part_1("./examples/input/day_4_part_1.txt"),
        "Number of contained assignments: ",
    );
    AocRunner::run(
        DayFour::part_2("./examples/input/day_4_example.txt"),
        "Example output: ",
    );
    AocRunner::run(
        DayFour::part_2("./examples/input/day_4_part_1.txt"),
        "Number of overlapping assignments: ",
    );
}
