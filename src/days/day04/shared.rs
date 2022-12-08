use std::{error, fmt, ops::RangeInclusive, result};

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

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

pub type Range = RangeInclusive<u32>;
pub type RangePair = (Range, Range);

pub fn get_range_pairs(line: &str) -> Result<RangePair> {
    let (part1, part2) = line
        .split_once(',')
        .ok_or::<ParseError>(ParseError::MissingCommaInPair(line.into()).into())?;
    let range1 = get_range(part1)?;
    let range2 = get_range(part2)?;
    Ok((range1, range2))
}

pub fn get_range(part: &str) -> Result<Range> {
    let (start_raw, end_raw) = part
        .split_once('-')
        .ok_or(ParseError::MissingDashInRange(part.into()))?;
    let start: u32 = start_raw.parse()?;
    let end: u32 = end_raw.parse()?;
    Ok(start..=end)
}
