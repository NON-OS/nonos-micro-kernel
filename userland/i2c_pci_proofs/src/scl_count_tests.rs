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

//! The count values themselves, pinned per input clock, and the floors that
//! keep a misreported clock from stopping the bus.

use crate::init::scl::{fast, fs_spklen, sda_hold, standard};

const GEMINI_LAKE: u32 = 133_000_000;
const TIGER_LAKE: u32 = 100_000_000;
const SUNRISE_POINT: u32 = 120_000_000;

#[test]
fn each_input_clock_gives_the_counts_the_timing_produces() {
    /*
     * Deriving these from the wrong input clock is the documented Gemini Lake
     * trap: 133 MHz counts fed a 100 MHz assumption clock the bus a third
     * fast, which is in spec for nothing.
     */
    assert_eq!((standard(GEMINI_LAKE).hcnt, standard(GEMINI_LAKE).lcnt), (685, 664));
    assert_eq!((fast(GEMINI_LAKE).hcnt, fast(GEMINI_LAKE).lcnt), (113, 212));
    assert_eq!((standard(TIGER_LAKE).hcnt, standard(TIGER_LAKE).lcnt), (513, 499));
    assert_eq!((fast(TIGER_LAKE).hcnt, fast(TIGER_LAKE).lcnt), (83, 159));
    assert_eq!((standard(SUNRISE_POINT).hcnt, standard(SUNRISE_POINT).lcnt), (617, 599));
    assert_eq!((fast(SUNRISE_POINT).hcnt, fast(SUNRISE_POINT).lcnt), (101, 191));
}

#[test]
fn a_clock_too_slow_to_divide_still_yields_counts_the_core_will_accept() {
    /*
     * Programming a zero count stops the clock entirely; the floors keep a
     * misreported input clock from bricking the bus.
     */
    assert_eq!((standard(1_000_000).hcnt, standard(1_000_000).lcnt), (6, 8));
}

#[test]
fn the_spike_filter_and_sda_hold_are_never_programmed_as_zero() {
    /*
     * Zero spike length disables glitch suppression, and zero SDA hold puts
     * the data edge on the clock edge, which reads back as random NAKs.
     */
    for clk in [GEMINI_LAKE, TIGER_LAKE, SUNRISE_POINT, 1_000_000] {
        assert!(fs_spklen(clk) >= 1, "{clk} Hz gave a zero spike length");
        assert!(sda_hold(clk) >= 1, "{clk} Hz gave a zero SDA hold");
    }
    assert_eq!((fs_spklen(GEMINI_LAKE), sda_hold(GEMINI_LAKE)), (13, 39));
}
