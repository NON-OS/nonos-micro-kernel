// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

extern crate alloc;
use alloc::vec::Vec;

pub struct Waveform {
    pub buckets: Vec<u8>,
}

fn abs_i32(sample: i16) -> i32 {
    (sample as i32).abs()
}

pub fn from_samples(all: &[i16], n_buckets: usize) -> Waveform {
    let mut buckets = Vec::new();
    buckets.resize(n_buckets, 0u8);

    if n_buckets == 0 || all.is_empty() {
        return Waveform { buckets };
    }

    let mut global_peak: i32 = 0;
    for &s in all {
        let a = abs_i32(s);
        if a > global_peak {
            global_peak = a;
        }
    }
    if global_peak == 0 {
        return Waveform { buckets };
    }

    let window_len = all.len() / n_buckets;
    if window_len == 0 {
        return Waveform { buckets };
    }

    for i in 0..n_buckets {
        let start = i * window_len;
        let end = start + window_len;
        let mut peak: i32 = 0;
        for &s in &all[start..end] {
            let a = abs_i32(s);
            if a > peak {
                peak = a;
            }
        }
        buckets[i] = (peak * 255 / global_peak) as u8;
    }

    Waveform { buckets }
}
