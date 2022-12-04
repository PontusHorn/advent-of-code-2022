use super::shared::ElfCalories;
use crate::input::read_lines;
use std::iter;

#[allow(dead_code)]
pub fn run(file_path: &str) -> Option<u32> {
    let lines = read_lines(file_path).expect(&format!("Failed to read input file {file_path}"));
    let elves = ElfCalories(lines);
    let max_3_elves = max_n(elves, 3);
    let sum = max_3_elves.iter().sum();
    Some(sum)
}

fn max_n<'a, Values>(values: Values, n: usize) -> Vec<u32>
where
    Values: Iterator<Item = u32>,
{
    let mut max_values = iter::repeat(0_u32).take(n).collect::<Vec<u32>>();

    for value in values {
        if value > max_values[0] {
            max_values[0] = value;
            max_values.sort();
        }
    }

    max_values
}
