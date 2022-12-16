use super::file_system::FileSystem;
use crate::input::read_lines;
use std::{error, fmt, result};

#[allow(dead_code)]
pub fn run(file_path: &str) -> Result<u32> {
    let lines = read_lines(file_path)?;
    let mut fs: FileSystem = lines.try_into()?;
    fs.set_total_space(70_000_000);
    let available_space = fs.get_available_space();
    let needed_space = 30_000_000;
    let space_to_free = needed_space - available_space;
    let smallest_dir_to_delete = fs
        .get_directories()
        .map(|d| fs.get_directory_size(d.path()))
        .filter(|size| *size >= space_to_free)
        .min()
        .ok_or(NoLargeEnoughDirectoryError)?;
    Ok(smallest_dir_to_delete)
}

type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct NoLargeEnoughDirectoryError;
impl error::Error for NoLargeEnoughDirectoryError {}
impl fmt::Display for NoLargeEnoughDirectoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No directory large enough to free up the space needed")
    }
}
