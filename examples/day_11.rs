use aoc_2022::{
    challenges::day_11::DayEleven,
    utilities::{day::Day, runner::AocRunner},
};

fn main() {
    AocRunner::run(
        DayEleven::part_1("./examples/input/day_11_example.txt"),
        "Example output: ",
    );
    AocRunner::run(
        DayEleven::part_1("./examples/input/day_11_part_1.txt"),
        "Monkey business: ",
    );
    AocRunner::run(
        DayEleven::part_2("./examples/input/day_11_example.txt"),
        "Example output: ",
    );
    AocRunner::run(
        DayEleven::part_2("./examples/input/day_11_part_1.txt"),
        "Monkey business after 10000 rounds: ",
    );
}
