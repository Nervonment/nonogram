use rand::random;

use crate::{
    generator::random_nonogram, solver::Solver,
    solver_backtrack_inference::SolverBacktrackInference,
};

#[test]
fn solver_test() {
    for _ in 0..100 {
        let problem = random_nonogram(random::<usize>() % 20 + 1, random::<usize>() % 20 + 1, 0.6);
        let mut solver = SolverBacktrackInference::new(&problem);
        let solution = solver.solve();
        assert!(
            solution.is_some() && solution.as_ref().unwrap().is_correct(),
            "{}",
            solution.unwrap()
        );
    }
}
