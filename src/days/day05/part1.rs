use crate::input::read_lines;
use std::{error, fmt, io, result, str::FromStr};

pub fn run(file_path: &str) -> Result<String> {
    let stacks = get_stacks(file_path)?;
    let Stacks(rearranged_stacks) = move_crates(file_path, stacks)?;
    let top_crates: Vec<&Crate> = rearranged_stacks
        .iter()
        .map(|Stack { crates, .. }| crates.last().unwrap_or(&Crate(' ')))
        .collect();
    let answer: String = top_crates
        .iter()
        .map(|Crate(crate_char)| crate_char)
        .collect();

    Ok(answer)
}

fn get_stacks(file_path: &str) -> Result<Stacks> {
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

fn move_crates(file_path: &str, Stacks(stacks): Stacks) -> Result<Stacks> {
    let instruction_lines = read_lines(file_path)?
        .skip_while(|line| line.as_ref().map_or(false, |line| !line.is_empty()))
        .skip(1);
    let mut rearranged_stacks = Stacks(Vec::new());
    for Stack { name, crates } in stacks {
        rearranged_stacks.0.push(Stack {
            name,
            crates: crates.to_vec(),
        });
    }

    for line in instruction_lines {
        let Move { count, from, to } = line?.parse::<Move>()?;
        for _ in 1..=count {
            let (new_stacks, moved_crate) = lift_crate(&rearranged_stacks, &from)?;
            rearranged_stacks = place_crate(&new_stacks, &to, moved_crate)?;
        }
    }

    Ok(rearranged_stacks)
}

fn lift_crate(stacks: &Stacks, from: &String) -> Result<(Stacks, Crate)> {
    let Stack { name, crates } = stacks.get(from)?;
    let mut remaining_crates = crates.to_vec();
    let lifted_crate = remaining_crates
        .pop()
        .ok_or(InstructionError::EmptyFromStack(from.to_string()))?;
    let remaining_stack = Stack {
        name: name.to_string(),
        crates: remaining_crates,
    };
    let mut new_stacks = Stacks(stacks.0.to_vec());
    new_stacks.replace(remaining_stack)?;
    Ok((new_stacks, lifted_crate))
}

fn place_crate(stacks: &Stacks, to: &String, moved_crate: Crate) -> Result<Stacks> {
    let Stack { name, crates } = stacks.get(to)?;
    let mut new_crates = crates.to_vec();
    new_crates.push(moved_crate);
    let new_stack = Stack {
        name: name.to_string(),
        crates: new_crates,
    };
    let mut new_stacks = Stacks(stacks.0.to_vec());
    new_stacks.replace(new_stack)?;
    Ok(new_stacks)
}

type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Clone)]
enum ParseError {
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
enum InstructionError {
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
struct Crate(pub char);

#[derive(Clone)]
struct Stack {
    pub name: String,
    pub crates: Vec<Crate>,
}

struct Stacks(pub Vec<Stack>);

impl Stacks {
    fn get(&self, name: &String) -> Result<&Stack> {
        let Self(stacks) = self;
        stacks
            .iter()
            .find(|stack| stack.name == *name)
            .ok_or(InstructionError::InvalidStack(name.to_string()).into())
    }
    fn replace(&mut self, stack: Stack) -> Result<()> {
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
struct Move {
    count: usize,
    from: String,
    to: String,
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
