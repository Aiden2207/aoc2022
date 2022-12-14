use aoc2022::day14::*;
use aoc2022::read_data;
fn main() {
    let _test = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    let s = read_data("day14.txt");
    println!("day14:\n{}", part2(&s));
}
