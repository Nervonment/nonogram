use std::fmt::Display;

use crate::solver::Solution;

#[derive(Clone)]
pub struct Problem {
    pub col_info: Vec<Vec<i32>>,
    pub row_info: Vec<Vec<i32>>,
}

impl From<Vec<Vec<bool>>> for Problem {
    fn from(grid: Vec<Vec<bool>>) -> Self {
        if grid.is_empty() {
            return Self {
                col_info: vec![],
                row_info: vec![],
            };
        }
        let height = grid.len();
        let width = grid[0].len();
        let mut col_info = vec![vec![]; width];
        let mut row_info = vec![vec![]; height];

        for col in 0..width {
            let mut num = 0;
            for row in 0..height {
                if grid[row][col] {
                    num += 1;
                } else if num > 0 {
                    col_info[col].push(num);
                    num = 0;
                }
            }
            if num > 0 {
                col_info[col].push(num);
            }
        }
        for row in 0..height {
            let mut num = 0;
            for col in 0..width {
                if grid[row][col] {
                    num += 1;
                } else if num > 0 {
                    row_info[row].push(num);
                    num = 0;
                }
            }
            if num > 0 {
                row_info[row].push(num);
            }
        }

        Self { col_info, row_info }
    }
}

impl Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let empty_solution = Solution {
            problem: self.clone(),
            grid: vec![vec![false; self.col_info.len()]; self.row_info.len()],
        };
        write!(f, "{}", empty_solution)?;
        Ok(())
    }
}
