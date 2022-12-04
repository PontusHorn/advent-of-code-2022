use std::fs::File;
use std::io::{self, BufRead};
use std::iter;
use std::path::Path;

struct ElfCalories(io::Lines<io::BufReader<File>>);
impl Iterator for ElfCalories {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let mut current = 0;
        let Self(lines) = self;
        for line in lines {
            let food_item = line.ok()?;
            let res = food_item.parse::<Self::Item>();
            match res {
                Ok(calories) => current += calories,
                Err(_) => return Some(current),
            }
        }
        None
    }
}

#[allow(dead_code)]
pub fn part1(file_path: &str) -> Option<u32> {
    let lines = read_lines(file_path).expect(&format!("Failed to read input file {file_path}"));
    let elves = ElfCalories(lines);
    elves.max()
}

#[allow(dead_code)]
pub fn part2(file_path: &str) -> Option<u32> {
    let lines = read_lines(file_path).expect(&format!("Failed to read input file {file_path}"));
    let elves = ElfCalories(lines);
    let max_3_elves = max_n(elves, 3);
    let sum = max_3_elves.iter().sum();
    Some(sum)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
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
