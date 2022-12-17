use std::{collections::HashSet, error, fmt};

use super::tree::Tree;

#[derive(Debug)]
pub struct TreeGrid {
    grid: Vec<i32>,
    col_count: usize,
}

impl TreeGrid {
    pub fn row_count(&self) -> usize {
        self.grid.len() / self.col_count
    }

    pub fn col_count(&self) -> usize {
        self.col_count
    }

    pub fn size(&self) -> (usize, usize) {
        (self.col_count(), self.row_count())
    }

    fn get(&self, col: usize, row: usize) -> (usize, i32) {
        let i = row * self.col_count + col;
        (i, self.grid[i])
    }

    fn get_trees(&self) -> Vec<Tree> {
        let (col_count, row_count) = self.size();
        let mut trees = Vec::with_capacity(self.grid.len());

        for col in 0..col_count {
            for row in 0..row_count {
                let (i, tree_height) = self.get(col, row);
                trees.push(Tree {
                    i,
                    col,
                    row,
                    height: tree_height,
                })
            }
        }

        trees
    }

    pub fn count_visible(&self) -> usize {
        let mut visible = HashSet::new();
        let (col_count, row_count) = self.size();

        for col in 0..col_count {
            let mut max_height = -1;
            for row in 0..row_count {
                let (i, tree_height) = self.get(col, row);
                if tree_height > max_height {
                    max_height = tree_height;
                    visible.insert(i);
                }
            }

            max_height = -1;
            for row in (0..row_count).rev() {
                let (i, tree_height) = self.get(col, row);
                if tree_height > max_height {
                    max_height = tree_height;
                    visible.insert(i);
                }
            }
        }

        for row in 0..row_count {
            let mut max_height = -1;
            for col in 0..col_count {
                let (i, tree_height) = self.get(col, row);
                if tree_height > max_height {
                    max_height = tree_height;
                    visible.insert(i);
                }
            }

            max_height = -1;
            for col in (0..col_count).rev() {
                let (i, tree_height) = self.get(col, row);
                if tree_height > max_height {
                    max_height = tree_height;
                    visible.insert(i);
                }
            }
        }

        visible.len()
    }

    pub fn get_scenic_scores(&self) -> Vec<usize> {
        self.get_trees()
            .iter()
            .map(|tree| self.get_scenic_score(tree))
            .collect()
    }

    fn get_scenic_score(&self, tree: &Tree) -> usize {
        self.get_scenic_score_top(&tree)
            * self.get_scenic_score_bottom(&tree)
            * self.get_scenic_score_left(&tree)
            * self.get_scenic_score_right(&tree)
    }

    fn get_scenic_score_top(&self, tree: &Tree) -> usize {
        let mut score = 0;
        for row in (0..tree.row).rev() {
            score += 1;
            let (_, other_height) = self.get(tree.col, row);
            if other_height >= tree.height {
                break;
            }
        }
        score
    }

    fn get_scenic_score_bottom(&self, tree: &Tree) -> usize {
        let row_count = self.row_count();
        let mut score = 0;
        for row in (tree.row + 1)..row_count {
            score += 1;
            let (_, other_height) = self.get(tree.col, row);
            if other_height >= tree.height {
                break;
            }
        }
        score
    }

    fn get_scenic_score_left(&self, tree: &Tree) -> usize {
        let mut score = 0;
        for col in (0..tree.col).rev() {
            score += 1;
            let (_, other_height) = self.get(col, tree.row);
            if other_height >= tree.height {
                break;
            }
        }
        score
    }

    fn get_scenic_score_right(&self, tree: &Tree) -> usize {
        let mut score = 0;
        for col in (tree.col + 1)..self.col_count {
            score += 1;
            let (_, other_height) = self.get(col, tree.row);
            if other_height >= tree.height {
                break;
            }
        }
        score
    }
}

impl TryFrom<&str> for TreeGrid {
    type Error = TreeGridError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let col_count = input
            .lines()
            .next()
            .ok_or(TreeGridError::MissingInput)?
            .len();

        let grid: Vec<i32> = input
            .chars()
            .filter(|&c| !c.is_whitespace())
            .map(|c| {
                c.to_digit(10)
                    .ok_or(TreeGridError::InvalidDigit(c))?
                    .try_into()
                    .map_err(|_| TreeGridError::InvalidDigit(c))
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { grid, col_count })
    }
}

impl fmt::Display for TreeGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut grid = self.grid.iter();
        let (col_count, row_count) = self.size();
        for row in 0..row_count {
            for _ in 0..col_count {
                write!(f, "{}", grid.next().unwrap())?;
            }
            if (row + 1) < row_count {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum TreeGridError {
    MissingInput,
    InvalidDigit(char),
}

impl error::Error for TreeGridError {}
impl fmt::Display for TreeGridError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TreeGridError::MissingInput => {
                write!(f, "Missing input for tree grid")
            }
            TreeGridError::InvalidDigit(char) => {
                write!(f, "Invalid digit in tree grid: {}", char)
            }
        }
    }
}
