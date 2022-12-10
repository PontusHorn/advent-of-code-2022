use super::shared::{get_range_pairs, RangePair, Result};
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
    let overlapping_range_pairs: Vec<&RangePair> =
        range_pairs.iter().filter(|&pair| overlap(pair)).collect();

    Ok(overlapping_range_pairs.len())
}

fn overlap((range1, range2): &RangePair) -> bool {
    range1.contains(&range2.start())
        || range1.contains(&range2.end())
        || range2.contains(&range1.start())
}
