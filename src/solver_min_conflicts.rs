// This solver is written on 2024/10/10 so it's named solver_1010.

use rand::random;

use crate::{
    csp::{enumerate_domain, Domain, Line, VarType},
    problem::Problem,
    solver::{Solution, Solver},
};

pub struct SolverMinConflicts {
    problem: Problem,
    width: usize,
    height: usize,
    col_domains: Vec<Domain>,
    row_domains: Vec<Domain>,
    col_assignments: Vec<Line>,
    row_assignments: Vec<Line>,
}

impl Solver for SolverMinConflicts {
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
        self.col_assignments = vec![0; self.width];
        self.row_assignments = vec![0; self.height];

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

        self.random_init();

        let mut fails = 0;
        while !self.is_complete() {
            let (var_type, var_idx) = self.select_var();
            // println!("{:?}, {}", var_type, var_idx);
            let domain = match var_type {
                VarType::Column => &self.col_domains[var_idx],
                VarType::Row => &self.row_domains[var_idx],
            }
            .clone();

            let mut new_value = 0;
            let mut min_conflicts = u32::MAX;
            for line_value in &domain.0 {
                let mut conflicts = 0;
                match var_type {
                    VarType::Column => {
                        for row in 0..self.height {
                            if (line_value & (1 << row) == 0)
                                != (self.row_assignments[row] & (1 << var_idx) == 0)
                            {
                                conflicts += 1;
                            }
                        }
                    }
                    VarType::Row => {
                        for col in 0..self.width {
                            if (line_value & (1 << col) == 0)
                                != (self.col_assignments[col] & (1 << var_idx) == 0)
                            {
                                conflicts += 1;
                            }
                        }
                    }
                }
                if conflicts < min_conflicts {
                    new_value = *line_value;
                    min_conflicts = conflicts;
                }
            }
            // println!(
            //     "{} <- {}",
            //     new_value,
            //     match var_type {
            //         VarType::Column => self.col_assignments[var_idx],
            //         VarType::Row => self.row_assignments[var_idx],
            //     }
            // );
            // println!("{}", min_conflicts);

            if match var_type {
                VarType::Column => self.col_assignments[var_idx] == new_value,
                VarType::Row => self.row_assignments[var_idx] == new_value,
            } {
                fails += 1;
                if fails > 1000 {
                    self.random_init();
                    fails = 0;
                    continue;
                }
            }

            match var_type {
                VarType::Column => self.col_assignments[var_idx] = new_value,
                VarType::Row => self.row_assignments[var_idx] = new_value,
            };

            // let mut grid = vec![vec![false; self.width]; self.height];
            // for row in 0..self.height {
            //     for col in 0..self.width {
            //         if self.col_assignments[col] & (1 << row) != 0 {
            //             grid[row][col] = true;
            //         }
            //     }
            // }
            // println!(
            //     "{}",
            //     Solution {
            //         problem: self.problem.clone(),
            //         grid,
            //     }
            // );
        }

        let mut grid = vec![vec![false; self.width]; self.height];
        for row in 0..self.height {
            for col in 0..self.width {
                if self.col_assignments[col] & (1 << row) != 0 {
                    grid[row][col] = true;
                }
            }
        }
        return Some(Solution {
            problem: self.problem.clone(),
            grid,
        });
    }
}

impl SolverMinConflicts {
    fn is_complete(&self) -> bool {
        for row in 0..self.height {
            for col in 0..self.width {
                if (self.col_assignments[col] & (1 << row) as u64 == 0)
                    != (self.row_assignments[row] & (1 << col) as u64 == 0)
                {
                    return false;
                }
            }
        }
        true
    }

    fn random_init(&mut self) {
        for col in 0..self.width {
            self.col_assignments[col] =
                self.col_domains[col].0[random::<usize>() % self.col_domains[col].0.len()];
        }
        for row in 0..self.height {
            self.row_assignments[row] =
                self.row_domains[row].0[random::<usize>() % self.row_domains[row].0.len()];
        }
    }

    fn select_var(&self) -> (VarType, usize) {
        let mut var_type = VarType::Column;
        let mut var_idx = 0;

        let mut max_conflicts = 0;

        for col in 0..self.width {
            let mut conflicts = 0;
            for row in 0..self.height {
                if (self.col_assignments[col] & (1 << row) == 0)
                    != (self.row_assignments[row] & (1 << col) == 0)
                {
                    conflicts += 1;
                }
            }

            if conflicts > max_conflicts && random::<bool>() {
                var_type = VarType::Column;
                var_idx = col;
                max_conflicts = conflicts;
            }
        }

        for row in 0..self.height {
            let mut conflicts = 0;
            for col in 0..self.width {
                if (self.col_assignments[col] & (1 << row) == 0)
                    != (self.row_assignments[row] & (1 << col) == 0)
                {
                    conflicts += 1;
                }
            }

            if conflicts > max_conflicts && random::<bool>() {
                var_type = VarType::Row;
                var_idx = row;
                max_conflicts = conflicts;
            }
        }

        // let var_type = if random::<bool>() {
        //     VarType::Column
        // } else {
        //     VarType::Row
        // };
        // let var_idx = match var_type {
        //     VarType::Row => random::<usize>() % self.height,
        //     VarType::Column => random::<usize>() % self.width,
        // };
        (var_type, var_idx)
    }
}
