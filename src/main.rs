use nonogram::{
    generator::{random_nonogram, random_nonogram_with_unique_solution},
    solver::Solver,
    solver_backtrack_inference::SolverBacktrackInference,
};

fn main() {
    let problem = random_nonogram(5, 5, 0.2);
    println!("Problem:");
    println!("{}", problem);

    let mut solver = SolverBacktrackInference::new(&problem);
    let solution = solver.any_solution();
    let solution_cnt = solver.solution_cnt();
    assert!(solution.is_some());
    let solution = solution.unwrap();
    assert!(solution.is_correct());
    println!("Solution:");
    println!("{}", solution);
    println!("Solution count: {}", solution_cnt);

    let w = 25;
    let h = 25;
    let d = 0.5;
    let problem = random_nonogram_with_unique_solution(w, h, d);
    println!("Problem:");
    println!("{}", problem);

    let mut solver = SolverBacktrackInference::new(&problem);
    let result = solver.unique_solution();
    assert!(result.is_unique);
    let solution = result.solution.unwrap();
    assert!(solution.is_correct());
    println!("Solution:");
    println!("{}", solution);

    let mut filled = 0;
    for row in &solution.grid {
        for v in row {
            filled += *v as usize;
        }
    }
    println!("Expected density: {}", d);
    println!("Actual density: {}", filled as f64 / (w * h) as f64);
}
