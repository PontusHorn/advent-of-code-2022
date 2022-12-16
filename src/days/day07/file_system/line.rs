use std::{error, fmt};

pub struct Line(pub LineContent);

impl TryFrom<&str> for Line {
    type Error = ParseLineError;
    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let content = parse_line_content(line)?;
        Ok(Self(content))
    }
}

fn parse_line_content(line: &str) -> Result<LineContent, ParseLineError> {
    if let Some(change_directory_line) = parse_change_directory_line(line) {
        return Ok(change_directory_line);
    }

    if line == "$ ls" {
        return Ok(LineContent::ListCommand);
    }

    if let Some(directory_line) = parse_directory_line(line) {
        return Ok(directory_line);
    }

    if let Some(file_line) = parse_file_line(line) {
        return Ok(file_line);
    }

    Err(ParseLineError::UnknownLineFormat(String::from(line)))
}

fn parse_change_directory_line(line: &str) -> Option<LineContent> {
    if !line.starts_with("$ cd ") {
        return None;
    }

    let dir_name = line.get(5..)?;
    Some(LineContent::ChangeDirectoryCommand(String::from(dir_name)))
}

fn parse_directory_line(line: &str) -> Option<LineContent> {
    if !line.starts_with("dir ") {
        return None;
    }

    let dir_name = line.get(4..)?;
    Some(LineContent::Directory(String::from(dir_name)))
}

fn parse_file_line(line: &str) -> Option<LineContent> {
    let (size, file_name) = line.split_once(" ")?;
    let size = size.parse::<u32>().ok()?;

    Some(LineContent::File(size, String::from(file_name)))
}

#[derive(Debug, Clone)]
pub enum ParseLineError {
    UnknownLineFormat(String),
}

impl error::Error for ParseLineError {}
impl fmt::Display for ParseLineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseLineError::UnknownLineFormat(line) => {
                write!(f, "Unknown line format: {line}")
            }
        }
    }
}

pub enum LineContent {
    ChangeDirectoryCommand(String),
    ListCommand,
    File(u32, String),
    Directory(String),
}
