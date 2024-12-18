use std::{fmt::Display, time::Duration};

use crate::problem::Problem;

#[derive(Clone)]
pub struct Solution {
    pub problem: Problem,
    pub grid: Vec<Vec<bool>>,
}

impl Solution {
    pub fn is_correct(&self) -> bool {
        let problem = Problem::from(self.grid.clone());
        problem.col_info == self.problem.col_info && problem.row_info == self.problem.row_info
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (r, row) in self.grid.iter().enumerate() {
            write!(f, " ")?;
            for cell in row {
                write!(f, "{}", if *cell { "▇▇ " } else { "▔▔▏" })?;
            }
            for num in &self.problem.row_info[r] {
                write!(f, "{:3}", num)?;
            }
            writeln!(f)?;
        }
        writeln!(f)?;
        let mut i = 0;
        loop {
            let mut finish = true;
            for col in 0..self.grid[0].len() {
                if self.problem.col_info[col].len() > i {
                    finish = false;
                    write!(f, "{:3}", self.problem.col_info[col][i])?;
                } else {
                    write!(f, "   ")?;
                }
            }
            writeln!(f)?;
            if finish {
                break;
            }
            i += 1;
        }
        Ok(())
    }
}

pub trait Solver {
    fn new(problem: &Problem) -> Self;
    fn timeout(&mut self, duration: Duration) -> &mut Self;
    fn any_solution(&mut self) -> Option<Solution>;
    fn unique_solution(&mut self) -> UniqueSolutionResult;
    fn solution_cnt(&mut self) -> u32;
}

pub struct UniqueSolutionResult {
    pub solution: Option<Solution>,
    pub is_unique: bool
}