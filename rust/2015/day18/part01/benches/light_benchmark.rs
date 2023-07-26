use criterion::{criterion_group, criterion_main, Criterion};
use part01::{parse_light_file, step_lights};

fn light_benchmark(c: &mut Criterion) {
    let light_array = parse_light_file();
    c.bench_function("light_benchmark", |bencher| {
        bencher.iter(|| {
            let mut light_array = light_array.clone();
            let mut working_light_array = light_array.clone();
            for _ in 0..100 {
                step_lights(&mut light_array, &mut working_light_array);
            }
        })
    });
}

criterion_group!(benches, light_benchmark);
criterion_main!(benches);
