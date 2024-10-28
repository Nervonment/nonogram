use std::time::{Duration, Instant};

use crate::{
    problem::Problem,
    solver::{Solution, Solver, UniqueSolutionResult},
};

#[derive(Clone)]
struct LineState {
    pub rest_1s: Vec<i32>,
    pub rest_0s_before_1: i32,
    pub need_0: bool,
    pub need_1: bool,
}

pub struct SolverBacktrackByCell {
    problem: Problem,
    grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    col_state: Vec<LineState>,
    row_state: Vec<LineState>,
    solution_cnt: u32,
    solution: Option<Solution>,
    timeout: Duration,
    start: Instant,
}

impl Solver for SolverBacktrackByCell {
    fn new(problem: &Problem) -> Self {
        let width = problem.col_info.len();
        let height = problem.row_info.len();
        Self {
            problem: problem.clone(),
            grid: vec![vec![false; width]; height],
            width,
            height,
            col_state: vec![],
            row_state: vec![],
            solution_cnt: 0,
            solution: None,
            timeout: Duration::from_secs(u64::MAX),
            start: Instant::now(),
        }
    }

    fn timeout(&mut self, duration: Duration) -> &mut Self {
        self.timeout = duration;
        self
    }

    fn any_solution(&mut self) -> Option<Solution> {
        self.init();
        if self.search(0, 0, 1) {
            return self.solution.clone();
        }
        None
    }

    fn unique_solution(&mut self) -> UniqueSolutionResult {
        self.init();
        self.search(0, 0, 2);
        UniqueSolutionResult {
            solution: self.solution.clone(),
            is_unique: self.solution_cnt == 1,
        }
    }

    fn solution_cnt(&mut self) -> u32 {
        self.init();
        self.search(0, 0, u32::MAX);
        self.solution_cnt
    }
}

impl SolverBacktrackByCell {
    fn init(&mut self) {
        self.start = Instant::now();
        self.col_state.clear();
        self.col_state.reserve(self.width);
        self.grid = vec![vec![false; self.width]; self.height];
        for col in 0..self.width {
            let rest_0s_before_1 = self.height as i32 + 1
                - self.problem.col_info[col].len() as i32
                - self.problem.col_info[col].iter().sum::<i32>();
            self.col_state.push(LineState {
                rest_1s: self.problem.col_info[col].clone(),
                rest_0s_before_1,
                need_0: false,
                need_1: false,
            });
            self.col_state[col].rest_1s.reverse();
        }
        self.row_state.clear();
        self.row_state.reserve(self.height);
        for row in 0..self.height {
            let rest_0s_before_1 = self.width as i32 + 1
                - self.problem.row_info[row].len() as i32
                - self.problem.row_info[row].iter().sum::<i32>();
            self.row_state.push(LineState {
                rest_1s: self.problem.row_info[row].clone(),
                rest_0s_before_1,
                need_0: false,
                need_1: false,
            });
            self.row_state[row].rest_1s.reverse();
        }
        self.solution_cnt = 0;
        self.solution = None;
    }

    fn search(&mut self, c: usize, r: usize, solution_cnt_needed: u32) -> bool {
        if Instant::now() - self.start > self.timeout {
            return false;
        }
        let tmp_row_state = self.row_state[r].clone();
        let tmp_col_state = self.col_state[c].clone();
        if self.is_assignment_valid(c, r, true) {
            self.grid[r][c] = true;
            *self.row_state[r].rest_1s.last_mut().unwrap() -= 1;
            self.row_state[r].need_1 = true;
            if *self.row_state[r].rest_1s.last().unwrap() == 0 {
                self.row_state[r].rest_0s_before_1 += 1;
                self.row_state[r].rest_1s.pop();
                self.row_state[r].need_0 = true;
                self.row_state[r].need_1 = false;
            }
            *self.col_state[c].rest_1s.last_mut().unwrap() -= 1;
            self.col_state[c].need_1 = true;
            if *self.col_state[c].rest_1s.last().unwrap() == 0 {
                self.col_state[c].rest_0s_before_1 += 1;
                self.col_state[c].rest_1s.pop();
                self.col_state[c].need_0 = true;
                self.col_state[c].need_1 = false;
            }

            if let Some((c, r)) = self.next_cell(c, r) {
                if self.search(c, r, solution_cnt_needed)
                    && self.solution_cnt >= solution_cnt_needed
                {
                    return true;
                }
            } else {
                self.solution_cnt += 1;
                if self.solution.is_none() {
                    self.solution = Some(Solution {
                        problem: self.problem.clone(),
                        grid: self.grid.clone(),
                    });
                }
                return true;
            }

            self.row_state[r] = tmp_row_state.clone();
            self.col_state[c] = tmp_col_state.clone();
        }
        if self.is_assignment_valid(c, r, false) {
            self.grid[r][c] = false;
            self.row_state[r].rest_0s_before_1 -= 1;
            self.col_state[c].rest_0s_before_1 -= 1;
            self.row_state[r].need_0 = false;
            self.col_state[c].need_0 = false;

            if let Some((c, r)) = self.next_cell(c, r) {
                if self.search(c, r, solution_cnt_needed)
                    && self.solution_cnt >= solution_cnt_needed
                {
                    return true;
                }
            } else {
                self.solution_cnt += 1;
                if self.solution.is_none() {
                    self.solution = Some(Solution {
                        problem: self.problem.clone(),
                        grid: self.grid.clone(),
                    });
                }
                return true;
            }

            self.row_state[r] = tmp_row_state;
            self.col_state[c] = tmp_col_state;
        }

        false
    }

    fn next_cell(&self, c: usize, r: usize) -> Option<(usize, usize)> {
        if r + 1 < c {
            if r + 1 < self.height {
                Some((c, r + 1))
            } else if c + 1 < self.width {
                Some((c + 1, 0))
            } else {
                None
            }
        } else if r + 1 == c {
            if c < self.height {
                Some((0, c))
            } else if c + 1 < self.width {
                Some((c + 1, 0))
            } else {
                None
            }
        } else if r != c {
            if c + 1 < self.width {
                Some((c + 1, r))
            } else if r + 1 < self.height {
                Some((0, r + 1))
            } else {
                None
            }
        } else if c + 1 < self.width {
            Some((c + 1, 0))
        } else if r + 1 < self.height {
            Some((0, r + 1))
        } else {
            None
        }
    }

    fn is_assignment_valid(&self, c: usize, r: usize, value: bool) -> bool {
        match value {
            true => {
                return !self.col_state[c].need_0
                    && !self.row_state[r].need_0
                    && !self.col_state[c].rest_1s.is_empty()
                    && !self.row_state[r].rest_1s.is_empty();
            }
            false => {
                return self.col_state[c].rest_0s_before_1 > 0
                    && self.row_state[r].rest_0s_before_1 > 0
                    && !self.col_state[c].need_1
                    && !self.row_state[r].need_1;
            }
        }
    }
}
