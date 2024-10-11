use crate::{
    csp::{enumerate_domain, Domain, Line, VarType},
    problem::Problem,
    solver::{Solution, Solver},
};

pub struct SolverBacktrackInference {
    problem: Problem,
    width: usize,
    height: usize,
    col_domains: Vec<Domain>,
    row_domains: Vec<Domain>,
    col_assignments: Vec<Option<Line>>,
    row_assignments: Vec<Option<Line>>,
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

        self.inference();
        self.search()
    }
}

impl SolverBacktrackInference {
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

        // println!(
        //     "Trying {:?} {} ({:?} possible values)",
        //     var_type,
        //     var_idx,
        //     domain.0.len()
        // );
        for line_value in &domain.0 {
            if self.is_assignment_consistent(&var_type, var_idx, *line_value) {
                match var_type {
                    VarType::Column => self.col_assignments[var_idx] = Some(*line_value),
                    VarType::Row => self.row_assignments[var_idx] = Some(*line_value),
                };
                let tmp_domains = (self.col_domains.clone(), self.row_domains.clone());
                match var_type {
                    VarType::Column => self.col_domains[var_idx] = Domain(vec![*line_value]),
                    VarType::Row => self.row_domains[var_idx] = Domain(vec![*line_value]),
                }

                // println!("{:?} {} <- {}", var_type, var_idx, line_value);
                // for row in 0..self.height {
                //     for col in 0..self.width {
                //         let ch;
                //         if self.col_assignments[col].is_some() {
                //             if self.row_assignments[row].is_some() {
                //                 if self.col_assignments[col].unwrap() & (1 << row) != 0
                //                     && self.row_assignments[row].unwrap() & (1 << col) != 0
                //                 {
                //                     ch = '🟪';
                //                 } else if self.col_assignments[col].unwrap() & (1 << row) != 0
                //                     && self.row_assignments[row].unwrap() & (1 << col) == 0
                //                 {
                //                     ch = '🟥';
                //                 } else if self.col_assignments[col].unwrap() & (1 << row) == 0
                //                     && self.row_assignments[row].unwrap() & (1 << col) != 0
                //                 {
                //                     ch = '🟦';
                //                 } else {
                //                     ch = '⬛';
                //                 }
                //             } else {
                //                 if self.col_assignments[col].unwrap() & (1 << row) != 0 {
                //                     ch = '🟥';
                //                 } else {
                //                     ch = '⬛';
                //                 }
                //             }
                //         } else {
                //             if self.row_assignments[row].is_some() {
                //                 if self.row_assignments[row].unwrap() & (1 << col) != 0 {
                //                     ch = '🟦';
                //                 } else {
                //                     ch = '⬛';
                //                 }
                //             } else {
                //                 ch = '⬛';
                //             }
                //         }
                //         print!("{}", ch);
                //     }
                //     println!();
                // }
                // println!();

                self.inference();

                let res = self.search();
                if res.is_some() {
                    return res;
                }

                match var_type {
                    VarType::Column => self.col_assignments[var_idx] = None,
                    VarType::Row => self.row_assignments[var_idx] = None,
                };
                self.col_domains = tmp_domains.0;
                self.row_domains = tmp_domains.1;
            } else {
                // println!("failed");
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

        // let mut col_masks_1 = vec![Line::MAX; self.width];
        // for col in 0..self.width {
        //     for value in &self.col_domains[col].0 {
        //         col_masks_1[col] &= value;
        //     }
        // }
        // let mut col_masks_0 = vec![Line::MAX; self.width];
        // for col in 0..self.width {
        //     for value in &self.col_domains[col].0 {
        //         col_masks_0[col] &= !value;
        //     }
        // }
        // for row in 0..self.height {
        //     for col in 0..self.width {
        //         if col_masks_1[col] & (1 << row) != 0 {
        //             print!("🟩");
        //         } else if col_masks_0[col] & (1 << row) != 0 {
        //             print!("❌");
        //         } else {
        //             print!("⬛");
        //         }
        //     }
        //     for num in &self.problem.row_info[row] {
        //         print!(
        //             "{}",
        //             [
        //                 "0️⃣", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣", "9️⃣", "🅰️", "🅱️", "🆎", "🆑",
        //                 "🅾️", "🆘"
        //             ][*num as usize]
        //         );
        //     }
        //     println!();
        // }
        // let mut i = 0;
        // loop {
        //     let mut finish = true;
        //     for col in 0..self.width {
        //         if self.problem.col_info[col].len() > i {
        //             finish = false;
        //             print!(
        //                 "{}",
        //                 [
        //                     "0️⃣", "1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣", "9️⃣", "🅰️", "🅱️", "🆎", "🆑",
        //                     "🅾️", "🆘"
        //                 ][self.problem.col_info[col][i] as usize]
        //             );
        //         // print!("{} ", self.problem.col_info[col][i]);
        //         } else {
        //             print!("⬛")
        //         }
        //     }
        //     println!();
        //     if finish {
        //         break;
        //     }
        //     i += 1;
        // }
        // println!();
    }
}
