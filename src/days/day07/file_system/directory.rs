use super::file::File;

#[derive(Debug, Clone)]
pub struct Directory {
    path: String,
    files: Vec<File>,
}

impl Directory {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.into(),
            files: Vec::new(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn size(&self) -> u32 {
        self.files.iter().map(|f| f.size()).sum()
    }
}
