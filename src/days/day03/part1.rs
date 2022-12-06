use crate::input::read_lines;
use std::{error, fmt};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[allow(dead_code)]
pub fn run(file_path: &str) -> Result<u32> {
    read_lines(file_path)?
        .map(|line| -> Result<u32> {
            let rucksack = line?;
            let (compartment1, compartment2) = get_compartments(&rucksack)?;
            let misplaced_item = get_common_item(&compartment1, &compartment2)?;
            get_item_priority(misplaced_item)
        })
        .collect::<Result<Vec<u32>>>()
        .map(|priorities| priorities.into_iter().sum())
}

#[derive(Debug, Clone)]
enum RucksackError {
    OddItemCount(String, usize),
    NoCommonItem(String, String),
    InvalidItem(char),
}

impl error::Error for RucksackError {}
impl fmt::Display for RucksackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RucksackError::OddItemCount(line, count) => {
                write!(
                    f,
                    "Each line should have an even number of characters. Got {count}: {line}"
                )
            }
            RucksackError::NoCommonItem(compartment1, compartment2) => {
                write!(
                    f,
                    "Compartments should have one shared item. Got {compartment1} and {compartment2}"
                )
            }
            RucksackError::InvalidItem(item) => {
                write!(
                    f,
                    "Items should be English letters (a-z or A-Z). Got {item}"
                )
            }
        }
    }
}

fn get_compartments(rucksack: &str) -> Result<(&str, &str)> {
    let item_count = rucksack.len();
    if item_count % 2 != 0 {
        return Err(RucksackError::OddItemCount(rucksack.to_string(), item_count).into());
    }

    Ok(rucksack.split_at(item_count / 2))
}

fn get_common_item(compartment1: &str, compartment2: &str) -> Result<char> {
    compartment1
        .chars()
        .find(|item| compartment2.contains(*item))
        .ok_or(
            RucksackError::NoCommonItem(compartment1.to_string(), compartment2.to_string()).into(),
        )
}

fn get_item_priority(item: char) -> Result<u32> {
    if item.is_ascii_lowercase() {
        // ASCII a-z range starts at 96
        return Ok(u32::from(item) - 96);
    } else if item.is_ascii_uppercase() {
        // ASCII A-Z range starts at 38
        return Ok(u32::from(item) - 38);
    }
    Err(RucksackError::InvalidItem(item).into())
}
