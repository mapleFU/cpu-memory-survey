use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use cache::constexpr;

/// Clearing all cache in L3 Cache.
fn clear_cache(c: &mut [u8; constexpr::L3_CACHE_USZ]) {
    for i in c.iter() {
        black_box(i);
    }
}

fn bench_with_sz(v: &Vec<u8>) {
    for i in v {
        black_box(i);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    // using clear array to clear all L3Cache
    let mut clear_array = [0; constexpr::L3_CACHE_USZ];
    let size_group = [4, 16, 64, 256];
    for size in size_group.iter() {
        let mut v: Vec<u8> = Vec::with_capacity(*size);
        v.resize(*size, 0);
        c.bench_with_input(BenchmarkId::new("testing_cache_line", size), &v, |b, s| {
            b.iter(|| bench_with_sz(s));
        });
        clear_cache(&mut clear_array);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
