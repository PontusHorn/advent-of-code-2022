use super::shared::{get_common_item, get_item_priority, Items, Result};
use crate::input::read_lines;

pub fn run(file_path: &str) -> Result<u32> {
    read_lines(file_path)?
        .map(|line| Ok(line?.chars().collect::<Items>()))
        .collect::<Result<Vec<Items>>>()?
        .chunks(3)
        .map(|chunk| -> Result<u32> {
            let group_rucksacks = chunk.to_vec();
            let group_badge = get_common_item(&group_rucksacks)?;
            get_item_priority(group_badge)
        })
        .collect::<Result<Vec<u32>>>()
        .map(|priorities| priorities.into_iter().sum())
}
