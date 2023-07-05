use criterion::{black_box, criterion_group, criterion_main, Criterion};
use inflator::EasterBunnyRecursiveInflator;

fn inflating_benchmark(c: &mut Criterion) {
    let input_string = black_box("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN");

    c.bench_function("inflating_benchmark", |bencher| {
        bencher.iter(|| {
            let mut inflator = EasterBunnyRecursiveInflator::new(input_string);
            while let Some(_) = inflator.next() {}
        })
    });
}

criterion_group!(benches, inflating_benchmark);
criterion_main!(benches);
