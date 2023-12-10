use aoc_2022::challenges::day_2::DayTwo;
use aoc_2022::utilities::day::Day;
use aoc_2022::utilities::runner::AocRunner;

fn main() {
    AocRunner::run(
        DayTwo::part_1("./examples/input/day_2_part_1.txt"),
        "Total score: ",
    );
    AocRunner::run(
        DayTwo::part_2("./examples/input/day_2_part_1.txt"),
        "Total score: ",
    );
}
