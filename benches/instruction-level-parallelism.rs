/// ```
/// cargo bench --benches visiting-array
/// ```
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use std::ops::AddAssign;

const ARRAY_SIZE: usize = 1024;

#[allow(unused)]
fn visit_array_same(array: &mut [u8]) {
    for i in 0..ARRAY_SIZE - 1 {
        black_box(array[i].add_assign(1));
        black_box(array[i].add_assign(1));
    }
}

#[allow(unused)]
fn visit_array_diff(array: &mut [u8]) {
    for i in 0..ARRAY_SIZE - 1 {
        black_box(array[i].add_assign(1));
        black_box(array[i + 1].add_assign(1));
    }
}

#[allow(unused)]
fn visit_array_same_2(array: &mut [u32]) {
    for _ in 0..black_box(ARRAY_SIZE - 1) {
        black_box(array[0].add_assign(1));
        black_box(array[0].add_assign(1));
    }
}

#[allow(unused)]
fn visit_array_diff_2(array: &mut [u32]) {
    for _ in 0..black_box(ARRAY_SIZE - 1) {
        black_box(array[0].add_assign(1));
        black_box(array[1].add_assign(1));
    }
}

/// This part test about "instruction-level-parallelism".
fn bench_array(c: &mut Criterion) {
    let mut test_array1 = Vec::with_capacity(ARRAY_SIZE);
    test_array1.resize(ARRAY_SIZE, 0);

    let mut group = c.benchmark_group("visiting-array");
    group.bench_with_input(
        BenchmarkId::new("same-traverse", 1i32),
        &test_array1,
        move |b, sample_array| {
            b.iter_batched(
                || sample_array.to_vec(),
                |mut arr| visit_array_same(&mut arr),
                BatchSize::SmallInput,
            )
        },
    );
    group.bench_with_input(
        BenchmarkId::new("diff-traverse", 1i32),
        &test_array1,
        move |b, sample_array| {
            b.iter_batched(
                || sample_array.to_vec(),
                |mut arr| visit_array_diff(&mut arr),
                BatchSize::SmallInput,
            )
        },
    );

    let mut test_array2 = Vec::with_capacity(ARRAY_SIZE);
    test_array2.resize(2, 0);

    group.bench_with_input(
        BenchmarkId::new("same-traverse-2", 1i32),
        &test_array2,
        move |b, sample_array| {
            b.iter_batched(
                || sample_array.to_vec(),
                |mut arr| visit_array_same_2(&mut arr),
                BatchSize::SmallInput,
            )
        },
    );

    group.bench_with_input(
        BenchmarkId::new("diff-traverse-2", 1i32),
        &test_array2,
        move |b, sample_array| {
            b.iter_batched(
                || sample_array.to_vec(),
                |mut arr| visit_array_diff_2(&mut arr),
                BatchSize::SmallInput,
            )
        },
    );
    group.finish();
}

criterion_group!(benches, bench_array);
criterion_main!(benches);
