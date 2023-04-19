use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ihex::{create_object_file_representation, Record};

pub fn criterion_benchmark(c: &mut Criterion) {
    let records = &[
        Record::Data {
            offset: 0x0010,
            value: vec![
                0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x67, 0x61, 0x70,
            ],
        },
        Record::ExtendedSegmentAddress(0x1200),
        Record::StartSegmentAddress {
            cs: 0x0000,
            ip: 0x3800,
        },
        Record::ExtendedLinearAddress(0xFFFF),
        Record::StartLinearAddress(0x000000CD),
        Record::EndOfFile,
    ];

    c.bench_function("create_object_file_representation", |b| {
        b.iter(|| {
            create_object_file_representation(black_box(records)).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
