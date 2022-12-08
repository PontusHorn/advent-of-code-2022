use crate::input::read_lines;
use std::{error, fmt, io, ops::RangeInclusive, result};

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

type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
enum ParseError {
    MissingCommaInPair(String),
    MissingDashInRange(String),
}

impl error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::MissingCommaInPair(pair) => {
                write!(f, "Missing comma in pair: {pair}")
            }
            ParseError::MissingDashInRange(range) => {
                write!(f, "Missing dash in range: {range}")
            }
        }
    }
}

type Range = RangeInclusive<u32>;
type RangePair = (Range, Range);

fn get_range_pairs(line: &str) -> Result<RangePair> {
    let (part1, part2) = line
        .split_once(',')
        .ok_or::<ParseError>(ParseError::MissingCommaInPair(line.into()).into())?;
    let range1 = get_range(part1)?;
    let range2 = get_range(part2)?;
    Ok((range1, range2))
}

fn get_range(part: &str) -> Result<Range> {
    let (start_raw, end_raw) = part
        .split_once('-')
        .ok_or(ParseError::MissingDashInRange(part.into()))?;
    let start: u32 = start_raw.parse()?;
    let end: u32 = end_raw.parse()?;
    Ok(start..=end)
}

fn fully_overlap((range1, range2): &RangePair) -> bool {
    contains_range(range1, range2) || contains_range(range2, range1)
}

fn contains_range(range1: &Range, range2: &Range) -> bool {
    range1.contains(&range2.start()) && range1.contains(&range2.end())
}
