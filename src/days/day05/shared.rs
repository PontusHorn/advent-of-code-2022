use crate::input::read_lines;
use std::{error, fmt, io, result, str::FromStr};

pub fn get_stacks(file_path: &str) -> Result<Stacks> {
    let (stack_name_line, stack_lines) = get_stack_lines(file_path)?;
    let stack_names = parse_stack_names(stack_name_line);
    let stacks = parse_stack_lines(stack_names, stack_lines)?;
    Ok(stacks)
}

fn get_stack_lines(file_path: &str) -> Result<(String, Vec<String>)> {
    let mut stack_lines = read_lines(file_path)?
        .take_while(|line| line.as_ref().map_or(false, |line| !line.is_empty()))
        .collect::<io::Result<Vec<_>>>()
        .map_err(|err| Box::new(err))?;
    let stack_name_line = stack_lines.pop().ok_or(ParseError::MissingStacks)?;
    stack_lines.reverse();
    Ok((stack_name_line, stack_lines))
}

fn parse_stack_names(stack_name_line: String) -> Vec<String> {
    stack_name_line
        .trim()
        .split_whitespace()
        .map(|name| name.to_string())
        .collect::<Vec<String>>()
}

fn parse_stack_lines(stack_names: Vec<String>, stack_lines: Vec<String>) -> Result<Stacks> {
    let mut stacks = Vec::new();

    for (index, name) in stack_names.iter().cloned().enumerate() {
        let mut crates = Vec::new();
        let crate_index = index * 4 + 1;
        for line in &stack_lines.clone() {
            let crate_char = line.chars().nth(crate_index).unwrap_or(' ');
            if crate_char != ' ' {
                crates.push(Crate(crate_char))
            }
        }
        stacks.push(Stack { name, crates });
    }

    Ok(Stacks(stacks))
}

pub fn lift_crates(stacks: &Stacks, from: &String, count: &usize) -> Result<(Stacks, Vec<Crate>)> {
    let Stack { name, crates } = stacks.get(from)?;
    let mut remaining_crates = crates.to_vec();
    let crate_count = crates.len();
    if crate_count < *count {
        return Err(Box::new(InstructionError::EmptyFromStack(from.to_string())));
    }
    let lifted_crates = remaining_crates.drain((crate_count - count)..).collect();
    let remaining_stack = Stack {
        name: name.to_string(),
        crates: remaining_crates,
    };
    let mut new_stacks = Stacks(stacks.0.to_vec());
    new_stacks.replace(remaining_stack)?;
    Ok((new_stacks, lifted_crates))
}

pub fn place_crates(stacks: &Stacks, to: &String, moved_crates: Vec<Crate>) -> Result<Stacks> {
    let Stack { name, crates } = stacks.get(to)?;
    let mut new_crates = crates.to_vec();
    new_crates.extend(moved_crates);
    let new_stack = Stack {
        name: name.to_string(),
        crates: new_crates,
    };
    let mut new_stacks = Stacks(stacks.0.to_vec());
    new_stacks.replace(new_stack)?;
    Ok(new_stacks)
}

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
pub enum ParseError {
    MissingStacks,
    InvalidMoveFormat(String),
}

impl error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::MissingStacks => {
                write!(f, "Missing stacks in input file")
            }
            ParseError::InvalidMoveFormat(line) => {
                write!(
                    f,
                    "Moves should be formatted as \"move 1 from 2 to 1\". Got: {line}"
                )
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum InstructionError {
    InvalidStack(String),
    EmptyFromStack(String),
}

impl error::Error for InstructionError {}
impl fmt::Display for InstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstructionError::InvalidStack(stack_name) => {
                write!(f, "Invalid stack name in instruction: {stack_name}")
            }
            InstructionError::EmptyFromStack(stack_name) => {
                write!(f, "Stack moved from should not be empty: {stack_name}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Crate(pub char);

#[derive(Clone)]
pub struct Stack {
    pub name: String,
    pub crates: Vec<Crate>,
}

pub struct Stacks(pub Vec<Stack>);

impl Stacks {
    pub fn get(&self, name: &String) -> Result<&Stack> {
        let Self(stacks) = self;
        stacks
            .iter()
            .find(|stack| stack.name == *name)
            .ok_or(InstructionError::InvalidStack(name.to_string()).into())
    }
    pub fn replace(&mut self, stack: Stack) -> Result<()> {
        let Self(stacks) = self;
        let (index, _) = stacks
            .iter()
            .enumerate()
            .find(|(_, s)| s.name == *stack.name)
            .ok_or(Box::new(InstructionError::InvalidStack(
                stack.name.to_string(),
            )))?;
        stacks[index] = stack;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Move {
    pub count: usize,
    pub from: String,
    pub to: String,
}

impl FromStr for Move {
    type Err = ParseError;
    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        let (count, from, to) = s
            .strip_prefix("move ")
            .and_then(|s| s.split_once(" from "))
            .and_then(|(count, s)| s.split_once(" to ").map(|(from, to)| (count, from, to)))
            .ok_or(ParseError::InvalidMoveFormat(s.to_string()))?;

        let count_num = count
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidMoveFormat(s.to_string()))?;

        Ok(Move {
            count: count_num,
            from: from.to_string(),
            to: to.to_string(),
        })
    }
}
