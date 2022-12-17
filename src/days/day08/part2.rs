use std::fs;

use super::{tree_grid::TreeGrid, treehouse_error::TreehouseError};

#[allow(dead_code)]
pub fn run(file_path: &str) -> Result<usize, TreehouseError> {
    let input = fs::read_to_string(file_path)?;
    let grid: TreeGrid = input.as_str().try_into()?;
    let best_scenic_score = *grid
        .get_scenic_scores()
        .iter()
        .max()
        .ok_or(TreehouseError::NoSuitableTreeError)?;
    Ok(best_scenic_score)
}
