use std::{error, fmt};

#[derive(Debug)]
pub enum InstructionError {
    InvalidInstruction(String),
    InvalidDirection(String),
    InvalidDistance(String),
}

impl error::Error for InstructionError {}
impl fmt::Display for InstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstructionError::InvalidInstruction(instruction) => {
                write!(
                    f,
                    "Instruction should have the format 'DIRECTION DISTANCE'. Got: {instruction}"
                )
            }
            InstructionError::InvalidDirection(direction) => {
                write!(f, "Direction should be R, L, U, or D. Got: {direction}")
            }
            InstructionError::InvalidDistance(distance) => {
                write!(f, "Distance should be a positive integer. Got: {distance}")
            }
        }
    }
}
