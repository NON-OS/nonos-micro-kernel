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

//! SCL counts for the DesignWare I2C master, from the bus timing the I2C
//! specification requires (UM10204 table 10). The core holds SCL high for
//! (HCNT + SPKLEN + 7) input clocks and low for (LCNT + 1). Splitting the
//! period in half met the frequency ceiling but left fast-mode tLOW at
//! 1.25 us against a 1.3 us minimum, which a strict device reads as a glitch.

#[derive(Clone, Copy)]
pub struct SclCounts {
    pub hcnt: u32,
    pub lcnt: u32,
}

/*
 * Low times are the specification minimums plus the 300 ns fall time, since
 * tLOW is measured from the end of the edge. High times are what brings each
 * period under the ceiling with no help from rise time: 97 kHz and 385 kHz.
 */
const SS_LOW_NS: u64 = 4_700 + 300;
const SS_HIGH_NS: u64 = 5_000 + 300;
const FS_LOW_NS: u64 = 1_300 + 300;
const FS_HIGH_NS: u64 = 700 + 300;
pub const HIGH_OVERHEAD: u32 = 7;
pub const LOW_OVERHEAD: u32 = 1;
// Floors: below these the core does not clock at all.
const HCNT_MIN: u32 = 6;
const LCNT_MIN: u32 = 8;

fn cycles(clk_hz: u32, ns: u64) -> u32 {
    (clk_hz as u64 * ns).div_ceil(1_000_000_000) as u32
}

fn counts(clk_hz: u32, low_ns: u64, high_ns: u64) -> SclCounts {
    let spklen = fs_spklen(clk_hz);
    SclCounts {
        hcnt: cycles(clk_hz, high_ns).saturating_sub(spklen + HIGH_OVERHEAD).max(HCNT_MIN),
        lcnt: cycles(clk_hz, low_ns).saturating_sub(LOW_OVERHEAD).max(LCNT_MIN),
    }
}

pub fn standard(clk_hz: u32) -> SclCounts {
    counts(clk_hz, SS_LOW_NS, SS_HIGH_NS)
}

pub fn fast(clk_hz: u32) -> SclCounts {
    counts(clk_hz, FS_LOW_NS, FS_HIGH_NS)
}

// Fast-mode spike-suppression length in input-clock cycles (~100ns). The
// DesignWare core needs this programmed or short glitches corrupt reads.
pub fn fs_spklen(clk_hz: u32) -> u32 {
    (clk_hz / 10_000_000).max(1)
}

pub fn sda_hold(clk_hz: u32) -> u32 {
    const HOLD_NS: u64 = 300;
    let ticks = (clk_hz as u64).saturating_mul(HOLD_NS) / 1_000_000_000;
    ticks.clamp(1, u16::MAX as u64) as u32
}
