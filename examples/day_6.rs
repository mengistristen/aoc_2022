use aoc_2022::{
    challenges::day_6::DaySix,
    utilities::{day::Day, runner::AocRunner},
};

fn main() {
    AocRunner::run(
        DaySix::part_1("./examples/input/day_6_example.txt"),
        "Example input: ",
    );
    AocRunner::run(
        DaySix::part_1("./examples/input/day_6_part_1.txt"),
        "Start of packet: ",
    );
    AocRunner::run(
        DaySix::part_2("./examples/input/day_6_part_1.txt"),
        "Start of message: ",
    );
}
