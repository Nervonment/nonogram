use crate::{
    problem::Problem,
    solver::{Solution, Solver},
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
        }
    }

    fn solve(&mut self) -> Option<Solution> {
        self.col_state.reserve(self.width);
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
        self.row_state.reserve(self.height);
        for row in 0..self.height {
            let rest_0s_before_1 = self.height as i32 + 1
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

        if self.search(0, 0) {
            return Some(Solution {
                problem: self.problem.clone(),
                grid: self.grid.clone(),
            });
        }

        None
    }
}

impl SolverBacktrackByCell {
    fn search(&mut self, c: usize, r: usize) -> bool {
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
                if self.search(c, r) {
                    return true;
                }
            } else {
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
                if self.search(c, r) {
                    return true;
                }
            } else {
                return true;
            }

            self.row_state[r] = tmp_row_state;
            self.col_state[c] = tmp_col_state;
        }

        false
    }

    fn next_cell(&self, c: usize, r: usize) -> Option<(usize, usize)> {
        if r + 1 < self.height {
            Some((c, r + 1))
        } else if c + 1 < self.width {
            Some((c + 1, 0))
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
