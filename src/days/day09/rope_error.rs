use std::{error, fmt, io};

use super::instruction_error::InstructionError;

#[derive(Debug)]
pub enum RopeError {
    IoError(io::Error),
    InstructionError(InstructionError),
}

impl error::Error for RopeError {}
impl fmt::Display for RopeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RopeError::IoError(err) => err.fmt(f),
            RopeError::InstructionError(err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for RopeError {
    fn from(err: io::Error) -> Self {
        RopeError::IoError(err)
    }
}

impl From<InstructionError> for RopeError {
    fn from(err: InstructionError) -> Self {
        RopeError::InstructionError(err)
    }
}
