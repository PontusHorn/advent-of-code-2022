use super::shared::{get_score, Move, Outcome};
use crate::input::read_lines;
use std::{error, fmt};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

pub fn run(file_path: &str) -> Result<u32> {
    read_lines(file_path)?
        .map(|line| -> Result<u32> {
            let strategy = parse_line(&line?)?;
            let own_move = get_own_move(&strategy);
            Ok(get_score(&own_move, &strategy.outcome))
        })
        .collect::<Result<Vec<u32>>>()
        .map(|scores| scores.into_iter().sum())
}

#[derive(Debug, Clone)]
enum ParseError {
    WrongColumnCount(String, usize),
    InvalidElfMove(String),
    InvalidOutcome(String),
}

impl error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::WrongColumnCount(line, count) => {
                write!(f, "Each line should have 2 columns. Got {count}: {line}")
            }
            ParseError::InvalidElfMove(elf_move) => {
                write!(f, "Elf move should be A, B, or C. Got {elf_move}")
            }
            ParseError::InvalidOutcome(outcome) => {
                write!(f, "Outcome should be X, Y, or Z. Got {outcome}")
            }
        }
    }
}

#[derive(Debug)]
struct Strategy {
    pub elf_move: Move,
    pub outcome: Outcome,
}

fn parse_line(line: &str) -> Result<Strategy> {
    let parts: Vec<&str> = line.split(" ").collect();
    let column_count = parts.len();
    if column_count != 2 {
        return Err(ParseError::WrongColumnCount(line.to_string(), column_count).into());
    }
    let elf_move = match parts[0] {
        "A" => Move::Rock,
        "B" => Move::Paper,
        "C" => Move::Scissors,
        _ => return Err(ParseError::InvalidElfMove(parts[0].to_string()).into()),
    };
    let outcome = match parts[1] {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => return Err(ParseError::InvalidOutcome(parts[1].to_string()).into()),
    };
    Ok(Strategy { elf_move, outcome })
}

fn get_own_move(strategy: &Strategy) -> Move {
    match strategy {
        Strategy {
            elf_move: Move::Rock,
            outcome: Outcome::Win,
        } => Move::Paper,
        Strategy {
            elf_move: Move::Rock,
            outcome: Outcome::Loss,
        } => Move::Scissors,
        Strategy {
            elf_move: Move::Rock,
            outcome: Outcome::Draw,
        } => Move::Rock,
        Strategy {
            elf_move: Move::Paper,
            outcome: Outcome::Win,
        } => Move::Scissors,
        Strategy {
            elf_move: Move::Paper,
            outcome: Outcome::Loss,
        } => Move::Rock,
        Strategy {
            elf_move: Move::Paper,
            outcome: Outcome::Draw,
        } => Move::Paper,
        Strategy {
            elf_move: Move::Scissors,
            outcome: Outcome::Win,
        } => Move::Rock,
        Strategy {
            elf_move: Move::Scissors,
            outcome: Outcome::Loss,
        } => Move::Paper,
        Strategy {
            elf_move: Move::Scissors,
            outcome: Outcome::Draw,
        } => Move::Scissors,
    }
}
