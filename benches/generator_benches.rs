use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
};
use nonogram::generator::random_nonogram_with_unique_solution;

fn generator_with_density_0_5(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    let mut group = c.benchmark_group(format!("Generator, Density = 0.5"));
    group.plot_config(plot_config);
    for size in [5, 10, 15, 20, 25] {
        group.bench_with_input(BenchmarkId::new("default", size), &size, |b, &size| {
            b.iter(|| {
                random_nonogram_with_unique_solution(size, size, 0.5);
            })
        });
    }
}

criterion_group!(benches, generator_with_density_0_5);
criterion_main!(benches);
