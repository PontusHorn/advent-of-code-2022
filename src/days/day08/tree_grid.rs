use std::{collections::HashSet, error, fmt};

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

    fn get(&self, col: usize, row: usize) -> (usize, i32) {
        let i = row * self.col_count + col;
        (i, self.grid[i])
    }

    pub fn count_visible(&self) -> usize {
        let mut visible = HashSet::new();
        let col_count = self.col_count();
        let row_count = self.row_count();

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
        let col_count = self.col_count();
        let row_count = self.row_count();
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
