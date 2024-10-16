use criterion::{criterion_group, criterion_main, Criterion};
use nonogram::{
    generator::random_nonogram, solver::Solver, solver_backtrack::SolverBacktrack,
    solver_backtrack_inference::SolverBacktrackInference,
};

fn benchmarks(c: &mut Criterion) {
    let problem = random_nonogram(15, 15, 0.6);
    c.bench_function("Backtrack", |b| {
        b.iter(|| {
            let mut solver = SolverBacktrack::new(&problem);
            solver.solve();
        })
    });
    c.bench_function("Backtrack with Inference", |b| {
        b.iter(|| {
            let mut solver = SolverBacktrackInference::new(&problem);
            solver.solve();
        })
    });
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
