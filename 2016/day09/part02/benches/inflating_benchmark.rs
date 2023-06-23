use criterion::{black_box, criterion_group, criterion_main, Criterion};
use inflator::EasterBunnyRecursiveInflator;

fn inflating_benchmark(c: &mut Criterion) {
    let input_string = black_box("(27x12)(20x12)(13x14)(7x10)(1x12)A");

    c.bench_function("inflating_benchmark", |bencher| {
        bencher.iter(|| {
            let mut inflator = EasterBunnyRecursiveInflator::new(input_string);
            while let Some(_) = inflator.next() {}
        })
    });
}

criterion_group!(benches, inflating_benchmark);
criterion_main!(benches);
