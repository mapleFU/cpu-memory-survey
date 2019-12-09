#![feature(test)]

/// 我希望能够测量出 cache-line, 但是由于 cpu-prefetching, 其实效果差不多
/// 接下来砍砍能不能砍掉 prefetching 然后交给 benchmark 做
/// 好吧，可能是 AMD 做了什么优化，在我的 Intel Core i7 的 Mac 上倒是很明显。
use cache::constexpr;

use std::hint::black_box;
use std::time::{Duration, Instant};

/// Clearing all cache in L3 Cache.
fn clear_cache(c: &mut [u8; constexpr::L2_CACHE_USZ]) {
    for i in c.iter_mut() {
        black_box(*i);
    }
}

fn bench_with_sz(v: &mut [u8], mut clear_array: &mut [u8; constexpr::L2_CACHE_USZ]) -> Duration {
    let mut sum_duration = Duration::default();
    for _ in 0..1024 {
        let start = Instant::now();
        for i in black_box(v.iter_mut()) {
            black_box(*i);
        }
        let duration = start.elapsed();
        sum_duration += duration;
        clear_cache(&mut clear_array);
    }
    sum_duration
}

fn benchmark() {
    // using clear array to clear all L3Cache
    let mut clear_array = [0; constexpr::L2_CACHE_USZ];
    let mut size_group: Vec<usize> = Vec::new();
    for sz in 60..=68 {
        size_group.push(sz);
    }
    let mut avg_sz = Vec::new();
    for size in size_group.iter() {
        let mut v: Vec<u8> = Vec::with_capacity(*size);
        v.resize(*size, 0);

        avg_sz.push((size, bench_with_sz(&mut v, &mut clear_array)));
    }

    for (sz, time) in avg_sz {
        println!(
            "in size {}, time is {}, average timing is {}",
            sz,
            time.as_nanos(),
            time.as_nanos() as usize / *sz
        );
    }
}

fn main() {
    benchmark();
}
