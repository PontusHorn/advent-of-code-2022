use std::{error, fmt};

#[derive(Debug)]
pub enum InstructionError {
    InvalidInstruction(String),
    InvalidAddXValue(String),
}

impl error::Error for InstructionError {}
impl fmt::Display for InstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstructionError::InvalidInstruction(instruction) => {
                write!(
                    f,
                    "Instructions should be either 'addx n' or 'noop'. Got: {instruction}"
                )
            }
            InstructionError::InvalidAddXValue(value) => {
                write!(f, "addx should be followed by an integer. Got: {value}")
            }
        }
    }
}
