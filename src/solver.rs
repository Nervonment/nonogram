use std::fmt::Display;

use crate::problem::Problem;

pub struct Solution {
    pub problem: Problem,
    pub grid: Vec<Vec<bool>>,
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", if *cell { "🟩" } else { "⬛" })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub trait Solver {
    fn new(problem: &Problem) -> Self;
    fn solve(&mut self) -> Option<Solution>;
}
