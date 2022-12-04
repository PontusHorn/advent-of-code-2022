use super::shared::ElfCalories;
use crate::input::read_lines;

#[allow(dead_code)]
pub fn run(file_path: &str) -> Option<u32> {
    let lines = read_lines(file_path).expect(&format!("Failed to read input file {file_path}"));
    let elves = ElfCalories(lines);
    elves.max()
}
