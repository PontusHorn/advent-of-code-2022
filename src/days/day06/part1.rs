use std::{error, fmt, fs, result};

pub fn run(file_path: &str) -> Result<usize> {
    let data = fs::read_to_string(file_path)?;
    let start_of_packet = find_start_of_packet(&data).ok_or(DecodeError::MissingStartOfPacket)?;
    Ok(start_of_packet)
}

fn find_start_of_packet(data: &str) -> Option<usize> {
    // Make sure we don't go past the last index
    let last_i = data.len() - 3;
    for (i, _) in data.chars().enumerate().take_while(|(i, _)| *i < last_i) {
        let current_four = &data[i..(i + 4)];
        if is_start_of_packet_marker(current_four) {
            return Some(i + 4);
        }
    }
    None
}

fn is_start_of_packet_marker(data: &str) -> bool {
    data.chars()
        // If all characters but one are unique, the last one must be too - so skip one
        .skip(1)
        .all(|c| data.matches(c).count() == 1)
}

type Result<T> = result::Result<T, Box<dyn error::Error>>;

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
