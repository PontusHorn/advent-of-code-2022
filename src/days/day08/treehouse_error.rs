use std::{error, fmt, io};

use super::tree_grid::TreeGridError;

#[derive(Debug)]
pub enum TreehouseError {
    IoError(io::Error),
    TreeGridError(TreeGridError),
}

impl error::Error for TreehouseError {}
impl fmt::Display for TreehouseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TreehouseError::IoError(err) => err.fmt(f),
            TreehouseError::TreeGridError(err) => err.fmt(f),
        }
    }
}

impl From<io::Error> for TreehouseError {
    fn from(err: io::Error) -> Self {
        TreehouseError::IoError(err)
    }
}

impl From<TreeGridError> for TreehouseError {
    fn from(err: TreeGridError) -> Self {
        TreehouseError::TreeGridError(err.into())
    }
}
