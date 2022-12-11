use std::{error, result};

pub fn find_first_sequence_of_unique_chars(string: &str, character_count: &usize) -> Option<usize> {
    // Make sure we don't go past the last index
    let last_i = string.len() - (character_count - 1);
    for (i, _) in string.chars().enumerate().take_while(|(i, _)| *i < last_i) {
        let current_sequence = &string[i..(i + character_count)];
        if are_all_characters_unique(current_sequence) {
            return Some(i + character_count);
        }
    }
    None
}

fn are_all_characters_unique(data: &str) -> bool {
    data.chars()
        // If all characters but one are unique, the last one must be too - so skip one
        .skip(1)
        .all(|c| data.matches(c).count() == 1)
}

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
