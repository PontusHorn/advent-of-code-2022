mod directory;
mod file;
mod file_system_error;
mod line;

use crate::input::Lines;
use directory::Directory;
use file::File;
use file_system_error::FileSystemError;
use line::{Line, LineContent};
use std::collections::HashMap;

#[derive(Debug)]
pub struct FileSystem {
    directories: HashMap<String, Directory>,
    total_space: u32,
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            directories: HashMap::new(),
            total_space: u32::MAX,
        }
    }

    pub fn set_total_space(&mut self, space: u32) {
        self.total_space = space;
    }

    pub fn add_directory(&mut self, directory: Directory) {
        self.directories
            .insert(directory.path().to_string(), directory);
    }

    pub fn add_file(&mut self, path: &str, file: File) -> Result<(), FileSystemError> {
        let dir = self.get_directory_mut(path)?;
        dir.add_file(file);
        Ok(())
    }

    pub fn get_directory_mut(&mut self, path: &str) -> Result<&mut Directory, FileSystemError> {
        self.directories
            .get_mut(path)
            .ok_or(FileSystemError::UnknownDirectory(path.to_string()))
    }

    pub fn get_directories(&self) -> impl Iterator<Item = &Directory> {
        self.directories.values()
    }

    pub fn get_directory_size(&self, path: &str) -> u32 {
        self.directories
            .iter()
            .filter(|(p, _)| *p == path || p.starts_with(&format!("{}/", path)))
            .map(|(_, d)| d.size())
            .sum::<u32>()
    }

    fn get_used_size(&self) -> u32 {
        self.get_directory_size("")
    }

    pub fn get_available_space(&self) -> u32 {
        self.total_space - self.get_used_size()
    }
}

impl TryFrom<Lines> for FileSystem {
    type Error = FileSystemError;
    fn try_from(lines: Lines) -> Result<Self, Self::Error> {
        let mut fs = FileSystem::new();
        let root_path = "";
        fs.add_directory(Directory::new(root_path));

        let mut current_path = String::with_capacity(32);

        for line in lines {
            let line = line?;
            let Line(line_content) = line.as_str().try_into()?;
            match line_content {
                LineContent::ChangeDirectoryCommand(name) if name.as_str() == "/" => {
                    current_path.clear();
                }
                LineContent::ChangeDirectoryCommand(name) if name.as_str() == ".." => {
                    current_path.truncate(current_path.rfind('/').unwrap_or(0));
                }
                LineContent::ChangeDirectoryCommand(name) => {
                    current_path.push_str(&["/", &name].concat());
                }
                LineContent::ListCommand => {}
                LineContent::Directory(name) => {
                    let path = [&current_path, "/", &name].concat();
                    fs.add_directory(Directory::new(&path));
                }
                LineContent::File(size, name) => {
                    let path = [&current_path, "/", &name].concat();
                    fs.add_file(&current_path, File::new(&path, size))?;
                }
            }
        }

        Ok(fs)
    }
}
