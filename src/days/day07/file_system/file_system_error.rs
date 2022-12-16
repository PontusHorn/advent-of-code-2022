use super::line::ParseLineError;
use std::{error, fmt, io};

#[derive(Debug)]
pub enum FileSystemError {
    ReadLineError(io::Error),
    ParseLineError(ParseLineError),
    UnknownDirectory(String),
}

impl error::Error for FileSystemError {}
impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileSystemError::ReadLineError(err) => err.fmt(f),
            FileSystemError::ParseLineError(err) => err.fmt(f),
            FileSystemError::UnknownDirectory(path) => {
                write!(f, "Unknown directory: {}", path)
            }
        }
    }
}

impl From<io::Error> for FileSystemError {
    fn from(err: io::Error) -> Self {
        FileSystemError::ReadLineError(err)
    }
}

impl From<ParseLineError> for FileSystemError {
    fn from(err: ParseLineError) -> Self {
        FileSystemError::ParseLineError(err)
    }
}
