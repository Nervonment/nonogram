// This solver is written on 2024/10/10 so it's named solver_1010.

use crate::{
    problem::Problem,
    solver::{Solution, Solver},
};

type Line = u64;

#[derive(Clone)]
struct Domain(Vec<Line>);

impl Domain {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn insert(&mut self, line: Line) {
        self.0.push(line);
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

enum VarType {
    Column,
    Row,
}

fn enumerate_domain(
    line_info: &Vec<i32>,
    num_idx: usize,
    line_width: usize,
    start_pos: usize,
    line: Line,
    listed: &mut Domain,
) {
    if num_idx >= line_info.len() {
        listed.insert(line);
        return;
    }

    let bar_len = line_info[num_idx];
    let bar = (1 << bar_len) - 1;
    for place_pos in start_pos..line_width {
        if place_pos + bar_len as usize > line_width {
            break;
        }

        let new_line = line | (bar << place_pos);
        enumerate_domain(
            line_info,
            num_idx + 1,
            line_width,
            place_pos + bar_len as usize + 1,
            new_line,
            listed,
        );
    }
}

#[test]
fn test_enumerate_domain() {
    let mut domain = Domain::new();
    let line_info = vec![5, 3, 1];
    let line_width = 15;
    enumerate_domain(&line_info, 0, line_width, 0, 0, &mut domain);
    for line in domain.0 {
        for i in 0..line_width {
            print!("{}", if line & (1 << i) != 0 { "🟩" } else { "⬛" });
        }
        println!();
    }
}

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
