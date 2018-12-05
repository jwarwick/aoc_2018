#[macro_use]
extern crate criterion;
extern crate day05;

use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    let filename = "input.txt";
    let contents = util::string_from_file(&filename);
    let contents2 = contents.clone();

    c.bench_function("react_count", move |b| b.iter(|| day05::react_count(&contents)));
    c.bench_function("best_polymer_count", move |b| b.iter(|| day05::best_polymer_count(&contents2)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
