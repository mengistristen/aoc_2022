use aoc_2022::challenges::day_3::DayThree;
use aoc_2022::utilities::day::Day;
use aoc_2022::utilities::runner::AocRunner;

fn main() {
    AocRunner::run(
        DayThree::part_1("./examples/input/day_3_example.txt"),
        "Part one example: ",
    );
    AocRunner::run(
        DayThree::part_1("./examples/input/day_3_part_1.txt"),
        "Total priority: ",
    );
    AocRunner::run(
        DayThree::part_2("./examples/input/day_3_example.txt"),
        "Part two example: ",
    );
    AocRunner::run(
        DayThree::part_2("./examples/input/day_3_part_1.txt"),
        "Total priority: ",
    );
}
