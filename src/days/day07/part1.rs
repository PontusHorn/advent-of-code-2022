use super::file_system::FileSystem;
use crate::input::read_lines;
use std::{error, result};

pub fn run(file_path: &str) -> Result<u32> {
    let lines = read_lines(file_path)?;
    let fs: FileSystem = lines.try_into()?;
    let size = fs
        .get_directories()
        .map(|d| fs.get_directory_size(d.path()))
        .filter(|size| *size <= 100_000)
        .sum();
    Ok(size)
}

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
