use nonogram::{
    generator::random_nonogram, solver::Solver,
    solver_backtrack_inference::SolverBacktrackInference,
};

fn main() {
    let problem = random_nonogram(10, 10, 0.4);
    println!("Problem:");
    println!("{}", problem);

    let mut solver = SolverBacktrackInference::new(&problem);
    let solution = solver.solve();

    if solution.is_some() {
        println!("Solution:");
        println!("{}", solution.unwrap());
    } else {
        println!("There is no solution.")
    }
}
