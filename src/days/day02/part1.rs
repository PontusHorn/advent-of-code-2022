use super::shared::{get_score, Move, Outcome};
use crate::input::read_lines;
use std::{error, fmt};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[allow(dead_code)]
pub fn run(file_path: &str) -> Result<u32> {
    read_lines(file_path)?
        .map(|line| -> Result<u32> {
            let moves = parse_line(&line?)?;
            let outcome = get_outcome(&moves);
            Ok(get_score(&moves.own, &outcome))
        })
        .collect::<Result<Vec<u32>>>()
        .map(|scores| scores.into_iter().sum())
}

#[derive(Debug, Clone)]
enum ParseError {
    WrongColumnCount(String, usize),
    InvalidElfMove(String),
    InvalidOwnMove(String),
}

impl error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::WrongColumnCount(line, count) => {
                write!(f, "Each line should have 2 columns. Got {count}: {line}")
            }
            ParseError::InvalidElfMove(elf) => {
                write!(f, "Elf move should be A, B, or C. Got {elf}")
            }
            ParseError::InvalidOwnMove(own) => {
                write!(f, "Own move should be X, Y, or Z. Got {own}")
            }
        }
    }
}

#[derive(Debug)]
struct Moves {
    pub elf: Move,
    pub own: Move,
}

fn parse_line(line: &str) -> Result<Moves> {
    let parts: Vec<&str> = line.split(" ").collect();
    let column_count = parts.len();
    if column_count != 2 {
        return Err(ParseError::WrongColumnCount(line.to_string(), column_count).into());
    }
    let elf = match parts[0] {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => return Err(ParseError::InvalidElfMove(parts[0].to_string()).into()),
    };
    let own = match parts[1] {
        "X" => Move::Rock,
        "Y" => Move::Paper,
        "Z" => Move::Scissors,
        _ => return Err(ParseError::InvalidOwnMove(parts[1].to_string()).into()),
    };
    Ok(Moves { elf, own })
}

fn get_outcome(moves: &Moves) -> Outcome {
    match moves {
        Moves {
            elf: Move::Rock,
            own: Move::Paper,
        } => Outcome::Win,
        Moves {
            elf: Move::Rock,
            own: Move::Scissors,
        } => Outcome::Loss,
        Moves {
            elf: Move::Rock,
            own: Move::Rock,
        } => Outcome::Draw,
        Moves {
            elf: Move::Paper,
            own: Move::Scissors,
        } => Outcome::Win,
        Moves {
            elf: Move::Paper,
            own: Move::Rock,
        } => Outcome::Loss,
        Moves {
            elf: Move::Paper,
            own: Move::Paper,
        } => Outcome::Draw,
        Moves {
            elf: Move::Scissors,
            own: Move::Rock,
        } => Outcome::Win,
        Moves {
            elf: Move::Scissors,
            own: Move::Paper,
        } => Outcome::Loss,
        Moves {
            elf: Move::Scissors,
            own: Move::Scissors,
        } => Outcome::Draw,
    }
}
