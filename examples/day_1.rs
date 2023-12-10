use aoc_2022::challenges::day_1::DayOne;
use aoc_2022::utilities::day::Day;
use aoc_2022::utilities::runner::AocRunner;

fn main() {
    AocRunner::run(
        DayOne::part_1("./examples/input/day_1_part_1.txt"),
        "The most calories held by an elf is ",
    );
    AocRunner::run(
        DayOne::part_2("./examples/input/day_1_part_1.txt"),
        "The total calories held by the top 3 elves is ",
    );
}
