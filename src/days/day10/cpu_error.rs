use super::instruction_error::InstructionError;
use std::{error, fmt, io};

#[derive(Debug)]
pub enum CpuError {
    IoError(io::Error),
    InstructionError(InstructionError),
}

impl error::Error for CpuError {}
impl fmt::Display for CpuError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CpuError::IoError(err) => err.fmt(f),
            CpuError::InstructionError(err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for CpuError {
    fn from(err: io::Error) -> Self {
        CpuError::IoError(err)
    }
}

impl From<InstructionError> for CpuError {
    fn from(err: InstructionError) -> Self {
        CpuError::InstructionError(err)
    }
}
