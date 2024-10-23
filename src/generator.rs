use rand::random;

use crate::{
    problem::Problem, solver::Solver, solver_backtrack_by_cell::SolverBacktrackByCell,
    solver_backtrack_inference::SolverBacktrackInference,
};

pub fn random_nonogram(width: usize, height: usize, density: f64) -> Problem {
    let mut grid = vec![vec![false; width]; height];

    for row in &mut grid {
        for cell in row {
            if random::<f64>() < density {
                *cell = true;
            }
        }
    }

    Problem::from(grid)
}

pub fn random_nonogram_with_unique_solution(
    width: usize,
    height: usize,
    mut expected_density: f64,
) -> Problem {
    let mut grid = vec![vec![true; width]; height];
    let mut rest = width * height;
    loop {
        rest -= 1;
        let (mut r, mut c) = (random::<usize>() % height, random::<usize>() % width);
        while !grid[c][r] {
            (r, c) = (random::<usize>() % height, random::<usize>() % width);
        }

        grid[c][r] = false;
        let problem = Problem::from(grid.clone());
        let density = rest as f64 / (width * height) as f64;
        if if width * height > 360 && density > 0.7 {
            SolverBacktrackByCell::new(&problem)
                .unique_solution()
                .is_unique
        } else {
            SolverBacktrackInference::new(&problem)
                .unique_solution()
                .is_unique
        } && density > expected_density
        {
            continue;
        }

        if density > expected_density {
            // 妥协
            expected_density += (1.0 - expected_density) * 0.03;
            grid = vec![vec![true; width]; height];
            rest = width * height;
            continue;
        }

        grid[c][r] = true;
        return Problem::from(grid);
    }
}
