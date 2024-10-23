use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
};
use nonogram::{
    generator::random_nonogram, solver::Solver, solver_backtrack::SolverBacktrack,
    solver_backtrack_by_cell::SolverBacktrackByCell,
    solver_backtrack_inference::SolverBacktrackInference,
};

fn with_size(c: &mut Criterion, size: usize) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group(format!("Size = {}", size));
    group.plot_config(plot_config);
    for density in [0.6, 0.633, 0.667, 0.7, 0.733, 0.767, 0.8] {
        if size < 20 {
            group.bench_with_input(
                BenchmarkId::new("Backtrack", density),
                &density,
                |b, &density| {
                    b.iter(|| {
                        let problem = random_nonogram(size, size, density);
                        let mut solver = SolverBacktrack::new(&problem);
                        solver.any_solution();
                    })
                },
            );
        }
        group.bench_with_input(
            BenchmarkId::new("Backtrack with Inference", density),
            &density,
            |b, &density| {
                b.iter(|| {
                    let problem = random_nonogram(size, size, density);
                    let mut solver = SolverBacktrackInference::new(&problem);
                    solver.any_solution();
                })
            },
        );
        group.bench_with_input(
            BenchmarkId::new("Backtrack by Cell", density),
            &density,
            |b, &density| {
                b.iter(|| {
                    let problem = random_nonogram(size, size, density);
                    let mut solver = SolverBacktrackByCell::new(&problem);
                    solver.any_solution();
                })
            },
        );
    }
}

fn size_10(c: &mut Criterion) {
    with_size(c, 10);
}

fn size_20(c: &mut Criterion) {
    with_size(c, 20);
}

fn size_25(c: &mut Criterion) {
    with_size(c, 25);
}

fn size_30(c: &mut Criterion) {
    with_size(c, 30);
}

fn size_35(c: &mut Criterion) {
    with_size(c, 35);
}

criterion_group!(benches, size_10, size_20, size_25, size_30, size_35);
criterion_main!(benches);
