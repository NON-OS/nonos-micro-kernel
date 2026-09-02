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
// Host-side harness for the sparkline's smoothing pass: the pure rotate-and-
// average step that turns the ring's raw window into the curve the rail plots.
// Compiled with the host toolchain:
//   rustc --edition 2021 --test tests/spark_host.rs -o /tmp/spark_host && /tmp/spark_host
//
// Only pure modules are pulled in; `spark.rs` itself needs a PaintBuffer and
// stays out.

#[path = "../src/paint"]
mod paint {
    pub mod spark_smooth;
}

use paint::spark_smooth::smooth;

fn run(src: &[u8], head: usize) -> Vec<u8> {
    let mut out = vec![0u8; src.len()];
    let n = smooth(src, head, &mut out);
    out.truncate(n);
    out
}

#[test]
fn an_empty_window_writes_nothing() {
    let mut out = [0u8; 4];
    assert_eq!(smooth(&[], 0, &mut out), 0);
    assert_eq!(smooth(&[5, 6, 7], 0, &mut []), 0);
}

#[test]
fn a_single_sample_survives_untouched() {
    assert_eq!(run(&[42], 0), vec![42]);
}

#[test]
fn a_flat_series_stays_flat() {
    assert_eq!(run(&[70; 8], 0), vec![70; 8]);
    assert_eq!(run(&[70; 8], 5), vec![70; 8], "rotation cannot disturb a constant");
}

#[test]
fn a_spike_is_attenuated_and_spread_over_its_neighbours() {
    let out = run(&[0, 0, 90, 0, 0], 0);
    assert!(out[2] < 90, "the peak is pulled down, got {}", out[2]);
    assert_eq!(out[2], 30);
    assert_eq!(out[1], 30);
    assert_eq!(out[3], 30);
    assert_eq!(out[0], 0);
    assert_eq!(out[4], 0);
}

#[test]
fn the_output_is_as_long_as_the_input_and_reads_oldest_first() {
    for n in 1..48usize {
        let src: Vec<u8> = (0..n as u8).collect();
        assert_eq!(run(&src, 0).len(), n);
        assert_eq!(run(&src, n / 2).len(), n);
    }
    let mut out = [0u8; 2];
    assert_eq!(smooth(&[1, 2, 3, 4], 0, &mut out), 2, "a short buffer caps the count");
    assert_eq!(run(&[0, 0, 0, 60, 60, 60], 3), vec![60, 60, 40, 20, 0, 0]);
}
