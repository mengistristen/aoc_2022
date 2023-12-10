use aoc_2022::{
    challenges::day_10::DayTen,
    utilities::{day::Day, runner::AocRunner},
};

fn main() {
    AocRunner::run(
        DayTen::part_1("./examples/input/day_10_example.txt"),
        "Example output: ",
    );
    AocRunner::run(
        DayTen::part_1("./examples/input/day_10_part_1.txt"),
        "Total signal strength: ",
    );
    AocRunner::run(DayTen::part_2("./examples/input/day_10_part_1.txt"), "\n");
}
