use super::shared::{get_range_pairs, Range, RangePair, Result};
use crate::input::read_lines;
use std::io;

#[allow(dead_code)]
pub fn run(file_path: &str) -> Result<usize> {
    let lines = read_lines(file_path)?
        .collect::<io::Result<Vec<_>>>()
        .map_err(|err| Box::new(err))?;
    let range_pairs: Vec<RangePair> = lines
        .iter()
        .map(|line| get_range_pairs(&line))
        .collect::<Result<_>>()?;
    let fully_overlapping_range_pairs: Vec<&RangePair> = range_pairs
        .iter()
        .filter(|&pair| fully_overlap(pair))
        .collect();

    Ok(fully_overlapping_range_pairs.len())
}

fn fully_overlap((range1, range2): &RangePair) -> bool {
    contains_range(range1, range2) || contains_range(range2, range1)
}

fn contains_range(range1: &Range, range2: &Range) -> bool {
    range1.contains(&range2.start()) && range1.contains(&range2.end())
}
