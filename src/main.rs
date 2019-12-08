#![feature(test)]

/// 我希望能够测量出 cache-line, 但是由于 cpu-prefetching, 其实效果差不多
/// 接下来砍砍能不能砍掉 prefetching 然后交给 benchmark 做

use cache::constexpr;

use std::time::{Duration, Instant};
use std::hint::black_box;

/// Clearing all cache in L3 Cache.
fn clear_cache(c: &mut [u8; constexpr::L2_CACHE_USZ]) {
    for i in c.iter_mut() {
        black_box(*i);
    }
}

fn bench_with_sz(v: &mut [u8], mut clear_array: &mut [u8; constexpr::L2_CACHE_USZ]) -> Duration {
    let mut sum_duration = Duration::default();
    for _ in 0..512 {
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
    let size_group: Vec<usize> = (16..96).collect();
    let mut avg_sz = Vec::new();
    for size in size_group.iter() {
        let mut v: Vec<u8> = Vec::with_capacity(*size);
        v.resize(*size, 0);

        avg_sz.push((size, bench_with_sz(&mut v, &mut clear_array)));
    }

    for (sz, time) in avg_sz {
        println!("in size {}, average timing is {}", sz, time.as_nanos());
    }
}

fn main() {
    benchmark();
}
