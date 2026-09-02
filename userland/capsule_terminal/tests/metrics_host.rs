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
//
// Host-side harness for the telemetry rail's arithmetic. Compiled with the
// host toolchain:
//   rustc --edition 2021 --test tests/metrics_host.rs -o /tmp/metrics_host && /tmp/metrics_host
//
// Only the allocation-free, syscall-free half of `src/rail` is pulled in:
// `sample.rs` reaches for `nonos_libc` and cannot link on the host, while the
// derivations and the sparkline window are pure and are what this asserts.

#[path = "../src/rail"]
mod rail {
    pub mod derive;
    pub mod disk;
    pub mod mem;
    pub mod metrics;
    pub mod net;
    pub mod ring;
    pub mod value;
}

use rail::derive::{cpu_pct, mem_pct};
use rail::metrics::{Proc, Sample, MAX_PROCS};
use rail::ring::{SparkRing, SPARK_SAMPLES};

#[test]
fn cpu_percent_is_the_tick_share() {
    assert_eq!(cpu_pct(25, 100), 25);
    assert_eq!(cpu_pct(1, 3), 33);
    assert_eq!(cpu_pct(0, 100), 0);
    assert_eq!(cpu_pct(100, 100), 100);
}

#[test]
fn a_zero_total_delta_reads_as_idle() {
    assert_eq!(cpu_pct(0, 0), 0);
    assert_eq!(cpu_pct(9_000, 0), 0);
}

#[test]
fn a_run_delta_over_the_total_clamps() {
    assert_eq!(cpu_pct(400, 100), 100);
    assert_eq!(cpu_pct(u64::MAX, 1), 100);
}

#[test]
fn memory_share_guards_an_empty_live_set() {
    assert_eq!(mem_pct(512, 2048), 25);
    assert_eq!(mem_pct(4096, 0), 0);
    assert_eq!(mem_pct(4096, 1024), 100);
}

fn chronological(r: &SparkRing) -> Vec<u8> {
    let d = r.slice();
    let n = d.len();
    (0..n).map(|i| d[(r.start() + i) % n]).collect()
}

#[test]
fn a_partial_window_reads_back_in_order() {
    let mut r = SparkRing::new();
    for v in [3u8, 9, 27] {
        r.push(v);
    }
    assert_eq!(r.len(), 3);
    assert_eq!(chronological(&r), vec![3, 9, 27]);
}

#[test]
fn a_wrapped_window_reads_back_in_order() {
    let mut r = SparkRing::new();
    for i in 0..(SPARK_SAMPLES + 5) {
        r.push((i % 101) as u8);
    }
    assert_eq!(r.len(), SPARK_SAMPLES);
    let got = chronological(&r);
    let first = (SPARK_SAMPLES + 5 - SPARK_SAMPLES) % 101;
    assert_eq!(got[0] as usize, first);
    assert_eq!(got[SPARK_SAMPLES - 1] as usize, (SPARK_SAMPLES + 4) % 101);
    for w in got.windows(2) {
        assert_eq!(w[1], w[0] + 1);
    }
}

#[test]
fn pushed_values_are_clamped_to_a_percentage() {
    let mut r = SparkRing::new();
    r.push(240);
    assert_eq!(chronological(&r), vec![100]);
}

#[test]
fn an_empty_sample_has_no_live_processes() {
    let s = Sample::EMPTY;
    assert_eq!(s.live().len(), 0);
    assert_eq!(Proc::EMPTY.name_str(), "");
    assert!(MAX_PROCS >= 40);
}

#[test]
fn a_name_reads_back_to_its_declared_length() {
    let mut p = Proc::EMPTY;
    p.name[..5].copy_from_slice(b"shell");
    p.name_len = 5;
    assert_eq!(p.name_str(), "shell");
}
