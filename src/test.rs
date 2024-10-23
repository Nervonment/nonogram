use rand::random;

use crate::{
    generator::{random_nonogram, random_nonogram_with_unique_solution},
    problem::Problem,
    solver::Solver,
    solver_backtrack::SolverBacktrack,
    solver_backtrack_by_cell::SolverBacktrackByCell,
    solver_backtrack_inference::SolverBacktrackInference,
};

fn have_solution<T: Solver>() {
    for _ in 0..100 {
        let problem = random_nonogram(random::<usize>() % 10 + 1, random::<usize>() % 10 + 1, 0.6);
        let mut solver = T::new(&problem);
        solver.any_solution();
        let solution = solver.any_solution();
        assert!(
            solution.is_some() && solution.as_ref().unwrap().is_correct(),
            "{}",
            solution.unwrap()
        );
    }
}

fn have_two_solutions<T: Solver>() {
    let problem = Problem::from(vec![
        vec![true, false, true, false, true, false, true, false],
        vec![false, true, false, true, false, true, false, true],
        vec![true, false, true, false, true, false, true, false],
        vec![false, true, false, true, false, true, false, true],
        vec![true, false, true, false, true, false, true, false],
        vec![false, true, false, true, false, true, false, true],
        vec![true, false, true, false, true, false, true, false],
        vec![false, true, false, true, false, true, false, true],
    ]);
    let mut solver = T::new(&problem);
    let result = solver.unique_solution();
    assert!(!result.is_unique);
    assert_eq!(solver.solution_cnt(), 2);
}

fn solution_cnt<T1: Solver, T2: Solver>() {
    for _ in 0..100 {
        let problem = random_nonogram(random::<usize>() % 10 + 1, random::<usize>() % 10 + 1, 0.6);
        let mut solver1 = T1::new(&problem);
        let mut solver2 = T2::new(&problem);
        assert_eq!(
            solver1.solution_cnt(),
            solver2.solution_cnt(),
            "Problem: {}",
            problem
        );
    }
}

#[test]
fn solvers_test() {
    have_solution::<SolverBacktrack>();
    have_two_solutions::<SolverBacktrack>();
    have_solution::<SolverBacktrackInference>();
    have_two_solutions::<SolverBacktrackInference>();
    have_solution::<SolverBacktrackByCell>();
    have_two_solutions::<SolverBacktrackByCell>();
    solution_cnt::<SolverBacktrack, SolverBacktrackInference>();
    solution_cnt::<SolverBacktrackInference, SolverBacktrackByCell>();
}

#[test]

fn generator_test() {
    for _ in 0..100 {
        let problem = random_nonogram_with_unique_solution(20, 20, 0.6);
        let mut solver = SolverBacktrackInference::new(&problem);
        let result = solver.unique_solution();
        assert!(result.is_unique);
    }
}
