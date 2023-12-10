use aoc_2022::{
    challenges::day_7::DaySeven,
    utilities::{day::Day, runner::AocRunner},
};

fn main() {
    AocRunner::run(
        DaySeven::part_1("./examples/input/day_7_example.txt"),
        "Example output: ",
    );
    AocRunner::run(
        DaySeven::part_1("./examples/input/day_7_part_1.txt"),
        "Sum of dirs < 100000: ",
    );
    AocRunner::run(
        DaySeven::part_2("./examples/input/day_7_example.txt"),
        "Example output: ",
    );
    AocRunner::run(
        DaySeven::part_2("./examples/input/day_7_part_1.txt"),
        "Size of dir to reach 30000000: ",
    );
}
