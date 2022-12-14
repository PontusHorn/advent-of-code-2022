use super::shared::{find_first_sequence_of_unique_chars, Result};
use std::{error, fmt, fs};

#[allow(dead_code)]
pub fn run(file_path: &str) -> Result<usize> {
    let data = fs::read_to_string(file_path)?;
    let start_of_packet = find_start_of_packet(&data).ok_or(DecodeError::MissingStartOfPacket)?;
    Ok(start_of_packet)
}

fn find_start_of_packet(data: &str) -> Option<usize> {
    find_first_sequence_of_unique_chars(data, &4)
}

#[derive(Debug, Clone)]
pub enum DecodeError {
    MissingStartOfPacket,
}

impl error::Error for DecodeError {}
impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecodeError::MissingStartOfPacket => {
                write!(f, "Missing start of packet in input data")
            }
        }
    }
}
