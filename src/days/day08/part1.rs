use std::fs;

use super::{tree_grid::TreeGrid, treehouse_error::TreehouseError};

pub fn run(file_path: &str) -> Result<usize, TreehouseError> {
    let input = fs::read_to_string(file_path)?;
    let grid: TreeGrid = input.as_str().try_into()?;
    let visible_count = grid.count_visible();
    Ok(visible_count)
}
