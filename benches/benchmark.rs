use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use sdcx::Parser;
use std::fs;

#[cfg(target_os = "linux")]
mod perf;

fn criterion_benchmark(c: &mut Criterion) {
    let text = fs::read_to_string("testcase/scan.sdc").unwrap();

    let mut group = c.benchmark_group("throughput");
    group.throughput(Throughput::Bytes(text.len() as u64));
    group.bench_function("parse", |b| {
        b.iter_with_large_drop(|| Parser::parse(black_box(&text), &""))
    });
    group.bench_function("validate", |b| {
        b.iter_with_large_drop(|| {
            let sdc = Parser::parse(black_box(&text), &"").unwrap();
            sdc.validate(None);
        })
    });
    group.finish();
}

#[cfg(target_os = "linux")]
criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(perf::FlamegraphProfiler::new(100));
    targets = criterion_benchmark
}

#[cfg(not(target_os = "linux"))]
criterion_group! {
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark
}

criterion_main!(benches);
