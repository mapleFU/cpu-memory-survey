use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use cache::constexpr;

/// Clearing all cache in L3 Cache.
fn clear_cache(c: &[u8; constexpr::L2_CACHE_USZ]) {
    for i in c.iter() {
        black_box(i);
    }
}

fn bench_with_sz(v: &[u8], clear_array: &[u8; constexpr::L2_CACHE_USZ]) {
    for i in v {
        black_box(i);
    }
    clear_cache(clear_array);
}

fn criterion_benchmark(c: &mut Criterion) {
    // using clear array to clear all L3Cache
    let clear_array = [0; constexpr::L2_CACHE_USZ];
    let size_group = [4, 16, 64, 256];
    for size in size_group.iter() {
        let mut v: Vec<u8> = Vec::with_capacity(*size);
        v.resize(*size, 0);
        c.bench_with_input(BenchmarkId::new("testing_cache_line", size), &v, |b, s| {
            b.iter(|| bench_with_sz(&s, &clear_array));
        });
        clear_cache(&clear_array);
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
