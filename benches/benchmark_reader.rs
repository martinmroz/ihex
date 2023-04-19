use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ihex::Record;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Record::from_record_string", |b| {
        b.iter(|| {
            Record::from_record_string(black_box(":0B0010006164647265737320676170A7")).unwrap()
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
