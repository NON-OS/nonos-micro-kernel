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

//! The SCL divider against the bus timing the I2C specification requires
//! (UM10204 table 10), which is arithmetic and needs no window at all.
//!
//! A bus clocked out of spec does not fail cleanly. Some transfers NAK and
//! others complete, so the touchpad reads as intermittently broken hardware
//! and the divider is the last place anyone looks. The times are checked as
//! the DesignWare databook defines them: SCL high lasts HCNT + SPKLEN + 7
//! input clocks, SCL low lasts LCNT + 1.

use crate::init::scl::{fast, fs_spklen, standard, SclCounts, HIGH_OVERHEAD, LOW_OVERHEAD};

const CLOCKS: [u32; 3] = [133_000_000, 100_000_000, 120_000_000];

/// (high, low) in nanoseconds, rounded down so a minimum is never met by
/// rounding.
fn times_ns(clk: u32, c: SclCounts) -> (u64, u64) {
    let ns = |cycles: u32| cycles as u64 * 1_000_000_000 / clk as u64;
    (ns(c.hcnt + fs_spklen(clk) + HIGH_OVERHEAD), ns(c.lcnt + LOW_OVERHEAD))
}

#[test]
fn fast_mode_meets_its_minimum_high_and_low_times_and_stays_under_400_khz() {
    for clk in CLOCKS {
        let (high, low) = times_ns(clk, fast(clk));
        assert!(high >= 600, "{clk} Hz: fast-mode tHIGH {high} ns under 600");
        assert!(low >= 1_300, "{clk} Hz: fast-mode tLOW {low} ns under 1300");
        assert!(high + low >= 2_500, "{clk} Hz: period {} ns is over 400 kHz", high + low);
    }
}

#[test]
fn standard_mode_meets_its_minimum_high_and_low_times_and_stays_under_100_khz() {
    for clk in CLOCKS {
        let (high, low) = times_ns(clk, standard(clk));
        assert!(high >= 4_000, "{clk} Hz: standard tHIGH {high} ns under 4000");
        assert!(low >= 4_700, "{clk} Hz: standard tLOW {low} ns under 4700");
        assert!(high + low >= 10_000, "{clk} Hz: period {} ns is over 100 kHz", high + low);
    }
}
