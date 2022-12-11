use std::path::{Path, PathBuf};
pub mod day1;
pub mod day10;
pub mod day11;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub fn read_data(p: impl AsRef<Path>) -> String {
    let path = PathBuf::from("data").join(p);
    std::fs::read_to_string(path).unwrap()
}
