use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use cache::constexpr;

/// time:   [340.70 us 341.18 us 341.77 us]
fn traverse_array1(sample: &[[i32; 1024]; 1024]) {
    for i in 0..1024 {
        for j in 0..1024 {
            black_box(sample[i][j]);
        }
    }
}

/// time:   [5.5622 ms 5.5658 ms 5.5702 ms]
fn traverse_array2(sample: &[[i32; 1024]; 1024]) {
    for i in 0..1024 {
        for j in 0..1024 {
            black_box(sample[j][i]);
        }
    }
}

/// This part test about "cache-friendly-code".
fn bench_array(c: &mut Criterion) {
    let mut group = c.benchmark_group("traverse-array");
    let sample_array = [[0; 1024]; 1024];
    group.bench_with_input(
        BenchmarkId::new("x-y-traverse", 1i32),
        &sample_array,
        |b, sample_array| b.iter(|| traverse_array1(&sample_array)),
    );
    group.bench_with_input(
        BenchmarkId::new("y-x-traverse", 2i32),
        &sample_array,
        |b, sample_array| b.iter(|| traverse_array2(&sample_array)),
    );
    group.finish();
}

fn show_kb() -> u64 {
    constexpr::KB
}

criterion_group!(benches, bench_array);
criterion_main!(benches);
