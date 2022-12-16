#[derive(Debug, Clone)]
pub struct File {
    _path: String,
    size: u32,
}

impl File {
    pub fn new(path: &str, size: u32) -> Self {
        Self {
            _path: String::from(path),
            size,
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}
