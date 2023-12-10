use aoc_2022::{
    challenges::day_9::DayNine,
    utilities::{day::Day, runner::AocRunner},
};

fn main() {
    AocRunner::run(
        DayNine::part_1("./examples/input/day_9_example.txt"),
        "Example output: ",
    );
    AocRunner::run(
        DayNine::part_1("./examples/input/day_9_part_1.txt"),
        "Number of spots the tail visits: ",
    );
    AocRunner::run(
        DayNine::part_2("./examples/input/day_9_example.txt"),
        "Example output: ",
    );
    AocRunner::run(
        DayNine::part_2("./examples/input/day_9_part_1.txt"),
        "Number of spots the tail visits: ",
    );
}
