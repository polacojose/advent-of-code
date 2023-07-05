use criterion::{criterion_group, criterion_main, Criterion};
use part01::{gen_map, solve};
use std::fs;

fn solving_benchmark(c: &mut Criterion) {
    c.bench_function("solving_benchmark", |bencher| {
        let input_map = gen_map(fs::read_to_string("input.txt").unwrap());
        bencher.iter(|| {
            let _result = solve(&input_map).unwrap();
        })
    });
}

criterion_group!(benches, solving_benchmark);
criterion_main!(benches);
