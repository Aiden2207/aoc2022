use aoc2022::day9::*;
use aoc2022::read_data;
fn main() {
    let _test = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    let s = read_data("day9.txt");
    println!("day9: {:?}", part2(&s));
}
