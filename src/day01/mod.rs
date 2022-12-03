use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1(file_path: &str) -> io::Result<u32> {
    let mut current_elf_calories = 0;
    let mut max_elf_calories = 0;

    let lines = read_lines(file_path)?;
    for line in lines {
        let food_item = line?;
        let res = food_item.parse::<u32>();
        match res {
            Ok(calories) => {
                current_elf_calories += calories;
                if current_elf_calories > max_elf_calories {
                    max_elf_calories = current_elf_calories;
                }
            }
            Err(_) => current_elf_calories = 0,
        }
    }

    Ok(max_elf_calories)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
