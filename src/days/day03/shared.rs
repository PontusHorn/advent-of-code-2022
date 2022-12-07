use std::{error, fmt};

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub type Items = Vec<char>;

#[derive(Debug, Clone)]
pub enum RucksackError {
    OddItemCount(String, usize),
    NoCommonItem(Vec<Items>),
    InvalidItem(char),
    MissingItems,
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
            RucksackError::NoCommonItem(items_lists) => {
                write!(
                    f,
                    "Items lists should have one shared item. Got {items_lists:?}"
                )
            }
            RucksackError::InvalidItem(item) => {
                write!(
                    f,
                    "Items should be English letters (a-z or A-Z). Got {item}"
                )
            }
            RucksackError::MissingItems => {
                write!(f, "Items should be present")
            }
        }
    }
}

pub fn get_common_item(items_lists: &Vec<Items>) -> Result<&char> {
    let (first_items, remaining_items_lists) = items_lists
        .split_first()
        .ok_or(RucksackError::MissingItems)?;

    first_items
        .iter()
        .find(|item| {
            remaining_items_lists
                .iter()
                .all(|items| items.contains(*item))
        })
        .ok_or(RucksackError::NoCommonItem(items_lists.to_vec()).into())
}

pub fn get_item_priority(item: &char) -> Result<u32> {
    if item.is_ascii_lowercase() {
        // ASCII a-z range starts at 96
        return Ok(u32::from(*item) - 96);
    } else if item.is_ascii_uppercase() {
        // ASCII A-Z range starts at 38
        return Ok(u32::from(*item) - 38);
    }
    Err(RucksackError::InvalidItem(*item).into())
}
