use super::shared::{get_common_item, get_item_priority, Items, Result, RucksackError};
use crate::input::read_lines;

#[allow(dead_code)]
pub fn run(file_path: &str) -> Result<u32> {
    read_lines(file_path)?
        .map(|line| -> Result<u32> {
            let rucksack = line?;
            let compartments = get_compartments(&rucksack)?;
            let misplaced_item = get_common_item(&compartments)?;
            get_item_priority(misplaced_item)
        })
        .collect::<Result<Vec<u32>>>()
        .map(|priorities| priorities.into_iter().sum())
}

fn get_compartments(rucksack: &str) -> Result<Vec<Items>> {
    let item_count = rucksack.len();
    if item_count % 2 != 0 {
        return Err(RucksackError::OddItemCount(rucksack.to_string(), item_count).into());
    }

    let (compartment1, compartment2) = rucksack.split_at(item_count / 2);
    Ok([
        compartment1.chars().collect(),
        compartment2.chars().collect(),
    ]
    .to_vec())
}
