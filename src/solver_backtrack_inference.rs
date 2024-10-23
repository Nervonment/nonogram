use crate::{
    csp::{enumerate_domain, Domain, Line, VarType},
    problem::Problem,
    solver::{Solution, Solver, UniqueSolutionResult},
};

pub struct SolverBacktrackInference {
    problem: Problem,
    width: usize,
    height: usize,
    col_domains: Vec<Domain>,
    row_domains: Vec<Domain>,
    col_assignments: Vec<Option<Line>>,
    row_assignments: Vec<Option<Line>>,
    solution_cnt: u32,
    solution: Option<Solution>,
}

impl Solver for SolverBacktrackInference {
    fn new(problem: &Problem) -> Self {
        Self {
            problem: problem.clone(),
            width: 0,
            height: 0,
            col_domains: vec![],
            row_domains: vec![],
            col_assignments: vec![],
            row_assignments: vec![],
            solution_cnt: 0,
            solution: None,
        }
    }

    fn any_solution(&mut self) -> Option<Solution> {
        self.init();
        if self.search(1) {
            return self.solution.clone();
        }
        None
    }

    fn unique_solution(&mut self) -> UniqueSolutionResult {
        self.init();
        self.search(2);
        UniqueSolutionResult {
            solution: self.solution.clone(),
            is_unique: self.solution_cnt == 1,
        }
    }

    fn solution_cnt(&mut self) -> u32 {
        self.init();
        self.search(u32::MAX);
        self.solution_cnt
    }
}

impl SolverBacktrackInference {
    fn init(&mut self) {
        self.width = self.problem.col_info.len();
        self.height = self.problem.row_info.len();
        self.col_domains = vec![Domain::new(); self.width];
        self.row_domains = vec![Domain::new(); self.height];
        self.col_assignments = vec![None; self.width];
        self.row_assignments = vec![None; self.height];
        self.solution_cnt = 0;
        self.solution = None;

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

        self.inference();
    }

    fn search(&mut self, solution_cnt_needed: u32) -> bool {
        if self.is_complete() {
            self.solution_cnt += 1;
            if self.solution.is_none() {
                self.solution = Some(self.to_solution());
            }
            return true;
        }

        let mut determined_cols = Vec::with_capacity(self.width);
        let mut determined_rows = Vec::with_capacity(self.height);
        for col in 0..self.width {
            if self.col_domains[col].size() == 1 {
                self.col_assignments[col] = Some(self.col_domains[col].0[0]);
                determined_cols.push(col);
            }
        }
        for row in 0..self.height {
            if self.row_domains[row].size() == 1 {
                self.row_assignments[row] = Some(self.row_domains[row].0[0]);
                determined_rows.push(row);
            }
        }

        let (var_type, var_idx) = self.select_unassigned_var();
        let domain = match var_type {
            VarType::Column => &self.col_domains[var_idx],
            VarType::Row => &self.row_domains[var_idx],
        }
        .clone();

        for line_value in &domain.0 {
            match var_type {
                VarType::Column => self.col_assignments[var_idx] = Some(*line_value),
                VarType::Row => self.row_assignments[var_idx] = Some(*line_value),
            };
            let tmp_domains = (self.col_domains.clone(), self.row_domains.clone());
            match var_type {
                VarType::Column => self.col_domains[var_idx] = Domain(vec![*line_value]),
                VarType::Row => self.row_domains[var_idx] = Domain(vec![*line_value]),
            }

            self.inference_single_var(&var_type, var_idx);

            if self.search(solution_cnt_needed) && self.solution_cnt >= solution_cnt_needed {
                return true;
            }

            match var_type {
                VarType::Column => self.col_assignments[var_idx] = None,
                VarType::Row => self.row_assignments[var_idx] = None,
            };
            self.col_domains = tmp_domains.0;
            self.row_domains = tmp_domains.1;
        }

        for col in determined_cols {
            self.col_assignments[col] = None;
        }
        for row in determined_rows {
            self.row_assignments[row] = None;
        }

        false
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

    fn inference(&mut self) {
        loop {
            let mut delete_count = 0;
            // mask: positions that must be filled
            for col in 0..self.width {
                let mut mask = Line::MAX;
                for value in &self.col_domains[col].0 {
                    mask &= value;
                }
                for row in 0..self.height {
                    let before = self.row_domains[row].0.len();
                    self.row_domains[row]
                        .0
                        .retain(|value| !(mask & (1 << row) != 0 && value & (1 << col) == 0));
                    delete_count += before - self.row_domains[row].0.len();
                }
            }
            for row in 0..self.height {
                let mut mask = Line::MAX;
                for value in &self.row_domains[row].0 {
                    mask &= value;
                }
                for col in 0..self.width {
                    let before = self.col_domains[col].0.len();
                    self.col_domains[col]
                        .0
                        .retain(|value| !(mask & (1 << col) != 0 && value & (1 << row) == 0));
                    delete_count += before - self.col_domains[col].0.len();
                }
            }

            // mask: positions that must NOT be filled
            for col in 0..self.width {
                let mut mask = Line::MAX;
                for value in &self.col_domains[col].0 {
                    mask &= !value;
                }
                for row in 0..self.height {
                    let before = self.row_domains[row].0.len();
                    self.row_domains[row]
                        .0
                        .retain(|value| !(mask & (1 << row) != 0 && value & (1 << col) != 0));
                    delete_count += before - self.row_domains[row].0.len();
                }
            }
            for row in 0..self.height {
                let mut mask = Line::MAX;
                for value in &self.row_domains[row].0 {
                    mask &= !value;
                }
                for col in 0..self.width {
                    let before = self.col_domains[col].0.len();
                    self.col_domains[col]
                        .0
                        .retain(|value| !(mask & (1 << col) != 0 && value & (1 << row) != 0));
                    delete_count += before - self.col_domains[col].0.len();
                }
            }

            if delete_count == 0 {
                break;
            }
        }
    }

    fn inference_single_var(&mut self, var_type: &VarType, var_idx: usize) {
        match var_type {
            VarType::Column => {
                let mut mask_1 = Line::MAX;
                let mut mask_0 = Line::MAX;
                for value in &self.col_domains[var_idx].0 {
                    mask_1 &= value;
                }
                for value in &self.col_domains[var_idx].0 {
                    mask_0 &= !value;
                }
                for row in 0..self.height {
                    let before = self.row_domains[row].0.len();
                    self.row_domains[row].0.retain(|value| {
                        !(mask_1 & (1 << row) != 0 && value & (1 << var_idx) == 0)
                            && !(mask_0 & (1 << row) != 0 && value & (1 << var_idx) != 0)
                    });
                    if before != self.row_domains[row].0.len() {
                        self.inference_single_var(&VarType::Row, row);
                    }
                }
            }
            VarType::Row => {
                let mut mask_1 = Line::MAX;
                let mut mask_0 = Line::MAX;
                for value in &self.row_domains[var_idx].0 {
                    mask_1 &= value;
                }
                for value in &self.row_domains[var_idx].0 {
                    mask_0 &= !value;
                }
                for col in 0..self.width {
                    let before = self.col_domains[col].0.len();
                    self.col_domains[col].0.retain(|value| {
                        !(mask_1 & (1 << col) != 0 && value & (1 << var_idx) == 0)
                            && !(mask_0 & (1 << col) != 0 && value & (1 << var_idx) != 0)
                    });
                    if before != self.col_domains[col].0.len() {
                        self.inference_single_var(&VarType::Column, col);
                    }
                }
            }
        }
    }

    fn to_solution(&self) -> Solution {
        let mut grid = vec![vec![false; self.width]; self.height];
        for row in 0..self.height {
            for col in 0..self.width {
                if self.col_assignments[col].unwrap() & (1 << row) != 0 {
                    grid[row][col] = true;
                }
            }
        }
        Solution {
            problem: self.problem.clone(),
            grid,
        }
    }
}
