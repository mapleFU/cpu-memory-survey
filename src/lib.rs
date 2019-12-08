// Copyright 2016 TiKV Project Authors. Licensed under Apache-2.0.

// The code is paste from tikv. I use this to show a readable code.
#[allow(unused)]
pub mod constexpr {
    const UNIT: u64 = 1;
    const DATA_MAGNITUDE: u64 = 1024;
    pub const KB: u64 = UNIT * DATA_MAGNITUDE;
    pub const MB: u64 = KB * DATA_MAGNITUDE;
    pub const GB: u64 = MB * DATA_MAGNITUDE;

    // Make sure it will not overflow.
    const TB: u64 = (GB as u64) * (DATA_MAGNITUDE as u64);
    const PB: u64 = (TB as u64) * (DATA_MAGNITUDE as u64);

    const TIME_MAGNITUDE_1: u64 = 1000;
    const TIME_MAGNITUDE_2: u64 = 60;
    const TIME_MAGNITUDE_3: u64 = 24;
    const MS: u64 = UNIT;
    const SECOND: u64 = MS * TIME_MAGNITUDE_1;
    const MINUTE: u64 = SECOND * TIME_MAGNITUDE_2;
    const HOUR: u64 = MINUTE * TIME_MAGNITUDE_2;
    const DAY: u64 = HOUR * TIME_MAGNITUDE_3;

    const L1D_CACHE: u64 = KB * 256;
    const L1I_CACHE: u64 = KB * 256;
    const L2_CACHE: u64 = MB * 4;
    const L3_CACHE: u64 = MB * 32;
}
