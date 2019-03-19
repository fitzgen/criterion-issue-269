use criterion::{
    black_box, criterion_group, criterion_main, Benchmark, Criterion, ParameterizedBenchmark,
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench(
        "a",
        Benchmark::new("b", |b| {
            b.iter(|| {
                black_box(0);
            })
        }),
    );

    c.bench(
        "a",
        ParameterizedBenchmark::new(
            "c",
            |b, _n| {
                b.iter(|| {
                    black_box(1);
                })
            },
            vec![1, 2],
        ),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
