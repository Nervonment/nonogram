use std::fmt::Display;

use crate::solver::Solution;

#[derive(Clone)]
pub struct Problem {
    pub col_info: Vec<Vec<i32>>,
    pub row_info: Vec<Vec<i32>>,
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
