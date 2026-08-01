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

//! Row compositing, against a per-pixel reference.
//!
//! The real loop takes opaque pixels in runs so the common case is a memcpy.
//! That is only worth having if it is indistinguishable from the obvious
//! per-pixel version, and a mistake here corrupts every window on screen, so
//! the two are compared pixel for pixel over shapes chosen to exercise every
//! run boundary.

use crate::compositor::row::{blend, composite_row};

/// The obvious implementation: one pixel at a time, no runs.
fn reference(src: &[u32], dst: &mut [u32]) {
    for col in 0..core::cmp::min(src.len(), dst.len()) {
        let px = src[col];
        let a = px >> 24;
        if a == 0 {
            continue;
        }
        if a == 0xFF {
            dst[col] = px;
            continue;
        }
        dst[col] = blend(px, dst[col], a);
    }
}

// Deterministic, so a failure is reproducible.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

#[test]
fn opaque_runs_match_the_per_pixel_reference() {
    let mut rng = Rng(0x243F_6A88_85A3_08D3);
    let mut pixels = 0u64;
    for trial in 0..20_000 {
        let w = (rng.next() % 64) as usize + 1;
        let mut src = vec![0u32; w];
        for p in src.iter_mut() {
            let r = rng.next();
            // Weighted toward opaque and transparent so runs of every length
            // form, with partial alpha still well represented.
            let a = match r % 10 {
                0..=5 => 0xFF,
                6..=7 => 0x00,
                _ => (r >> 8) as u32 & 0xFF,
            };
            *p = (a << 24) | ((r >> 16) as u32 & 0x00FF_FFFF);
        }
        let base: Vec<u32> = (0..w).map(|_| (rng.next() >> 8) as u32 | 0xFF00_0000).collect();
        let mut got = base.clone();
        let mut want = base.clone();
        composite_row(&src, &mut got);
        reference(&src, &mut want);
        assert_eq!(got, want, "trial {trial}, width {w}");
        pixels += w as u64;
    }
    assert!(pixels > 500_000, "coverage collapsed to {pixels} pixels");
}

#[test]
fn the_run_boundaries_are_handled() {
    // The shapes a random sweep can under-represent: a run at each end, a
    // single opaque pixel, and alternating pixels so every run is length one.
    let opaque = 0xFF00_00FFu32;
    let clear = 0x0000_0000u32;
    let half = 0x8012_3456u32;
    let cases: [&[u32]; 6] = [
        &[opaque],
        &[clear],
        &[opaque, clear, opaque, clear],
        &[clear, opaque, opaque, clear],
        &[half, opaque, half],
        &[opaque, opaque, half, clear, opaque],
    ];
    for (i, src) in cases.iter().enumerate() {
        let base: Vec<u32> = (0..src.len()).map(|n| 0xFF10_0000 + n as u32).collect();
        let mut got = base.clone();
        let mut want = base.clone();
        composite_row(src, &mut got);
        reference(src, &mut want);
        assert_eq!(got, want, "case {i}");
    }
}

#[test]
fn a_short_destination_is_not_overrun() {
    // Clipping can hand a destination shorter than the source; the row must
    // stop at whichever ends first rather than write past it.
    let src = [0xFF00_00FFu32; 8];
    let mut dst = [0u32; 3];
    composite_row(&src, &mut dst);
    assert_eq!(dst, [0xFF00_00FF; 3]);
}

#[test]
fn a_fully_transparent_row_leaves_the_frame_alone() {
    let src = [0x0000_0000u32; 16];
    let before: Vec<u32> = (0..16).map(|n| 0xFF00_0000 + n as u32).collect();
    let mut dst = before.clone();
    composite_row(&src, &mut dst);
    assert_eq!(dst, before);
}
