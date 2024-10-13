use crate::{
    generator::random_nonogram, solver::Solver,
    solver_backtrack_inference::SolverBacktrackInference,
};

#[test]
fn solver_test() {
    for _ in 0..20 {
        for size in 1..15 {
            let problem = random_nonogram(size, size + 1, 0.4);
            let mut solver = SolverBacktrackInference::new(&problem);
            let solution = solver.solve();
            assert!(
                solution.is_some() && solution.as_ref().unwrap().is_correct(),
                "{}",
                solution.unwrap()
            );
        }
    }
}
