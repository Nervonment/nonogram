// This solver is written on 2024/10/10 so it's named solver_1010.

use crate::{
    csp::{enumerate_domain, Domain, Line, VarType}, problem::Problem, solver::{Solution, Solver}
};

pub struct Solver1010 {
    problem: Problem,
    width: usize,
    height: usize,
    col_domains: Vec<Domain>,
    row_domains: Vec<Domain>,
    col_assignments: Vec<Option<Line>>,
    row_assignments: Vec<Option<Line>>,
}

impl Solver for Solver1010 {
    fn new(problem: &Problem) -> Self {
        Self {
            problem: problem.clone(),
            width: 0,
            height: 0,
            col_domains: vec![],
            row_domains: vec![],
            col_assignments: vec![],
            row_assignments: vec![],
        }
    }

    fn solve(&mut self) -> Option<Solution> {
        self.width = self.problem.col_info.len();
        self.height = self.problem.row_info.len();
        self.col_domains = vec![Domain::new(); self.width];
        self.row_domains = vec![Domain::new(); self.height];
        self.col_assignments = vec![None; self.width];
        self.row_assignments = vec![None; self.height];

        for col in 0..self.width {
            enumerate_domain(
                &self.problem.col_info[col],
                0,
                self.height,
                0,
                0,
                &mut self.col_domains[col],
            );
        }

        for row in 0..self.height {
            enumerate_domain(
                &self.problem.row_info[row],
                0,
                self.width,
                0,
                0,
                &mut self.row_domains[row],
            );
        }

        self.search()
    }
}

impl Solver1010 {
    fn search(&mut self) -> Option<Solution> {
        if self.is_complete() {
            let mut grid = vec![vec![false; self.width]; self.height];
            for row in 0..self.height {
                for col in 0..self.width {
                    if self.col_assignments[col].unwrap() & (1 << row) != 0 {
                        grid[row][col] = true;
                    }
                }
            }
            return Some(Solution {
                problem: self.problem.clone(),
                grid,
            });
        }

        let (var_type, var_idx) = self.select_unassigned_var();
        let domain = match var_type {
            VarType::Column => &self.col_domains[var_idx],
            VarType::Row => &self.row_domains[var_idx],
        }
        .clone();
        for line_value in &domain.0 {
            if self.is_assignment_consistent(&var_type, var_idx, *line_value) {
                match var_type {
                    VarType::Column => self.col_assignments[var_idx] = Some(*line_value),
                    VarType::Row => self.row_assignments[var_idx] = Some(*line_value),
                };
                let res = self.search();
                if res.is_some() {
                    return res;
                }
                match var_type {
                    VarType::Column => self.col_assignments[var_idx] = None,
                    VarType::Row => self.row_assignments[var_idx] = None,
                };
            }
        }

        None
    }

    fn is_complete(&self) -> bool {
        self.col_assignments.iter().all(|v| v.is_some())
            && self.row_assignments.iter().all(|v| v.is_some())
    }

    fn select_unassigned_var(&self) -> (VarType, usize) {
        let mut res = (VarType::Column, 0);
        let mut least = usize::MAX;
        for col in 0..self.width {
            if self.col_assignments[col].is_none() && self.col_domains[col].size() < least {
                least = self.col_domains[col].size();
                res = (VarType::Column, col);
            }
        }
        for row in 0..self.height {
            if self.row_assignments[row].is_none() && self.row_domains[row].size() < least {
                least = self.row_domains[row].size();
                res = (VarType::Row, row);
            }
        }
        res
    }

    fn is_assignment_consistent(&self, var_type: &VarType, var_idx: usize, value: Line) -> bool {
        match var_type {
            VarType::Row => {
                for col in 0..self.width {
                    if self.col_assignments[col].is_some()
                        && (value & (1 << col) == 0)
                            != (self.col_assignments[col].as_ref().unwrap() & (1 << var_idx) == 0)
                    {
                        return false;
                    }
                }
            }
            VarType::Column => {
                for row in 0..self.height {
                    if self.row_assignments[row].is_some()
                        && (value & (1 << row) == 0)
                            != (self.row_assignments[row].as_ref().unwrap() & (1 << var_idx) == 0)
                    {
                        return false;
                    }
                }
            }
        }
        true
    }
}
