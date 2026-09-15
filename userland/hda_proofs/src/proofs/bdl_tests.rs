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

//! The buffer descriptor list, which is arithmetic and needs no window.
//!
//! The BDL is the only thing telling the controller's DMA engine where the
//! samples are and how far to go before wrapping. A gap between two entries is
//! played as whatever memory happens to sit there, and a total that disagrees
//! with the cyclic buffer length programmed into SD_CBL puts the engine and
//! the driver's refill arithmetic on different rings, which drifts into a
//! stutter that only appears after minutes of playback.

use crate::controller::bdl::{build_bdl, BDL_IOC, N_PERIODS, PERIOD_BYTES, RING_BYTES};

const RING_BASE: u64 = 0x0000_0003_4000_0000;

#[test]
fn the_periods_tile_the_ring_with_no_gap_and_no_overlap() {
    let bdl = build_bdl(RING_BASE);
    assert_eq!(bdl.len(), N_PERIODS);
    for (i, e) in bdl.iter().enumerate() {
        assert_eq!(e.addr, RING_BASE + i as u64 * PERIOD_BYTES, "period {i} is misplaced");
        assert_eq!(e.len as u64, PERIOD_BYTES, "period {i} is the wrong length");
    }
    let last = &bdl[N_PERIODS - 1];
    assert_eq!(last.addr + last.len as u64, RING_BASE + RING_BYTES, "the ring does not close");
}

#[test]
fn every_period_asks_for_an_interrupt_on_completion() {
    /*
     * The refill loop is driven entirely by the buffer completion interrupt.
     * An entry without IOC is played silently, so the driver misses a period
     * and hands the engine samples it has already consumed.
     */
    for (i, e) in build_bdl(RING_BASE).iter().enumerate() {
        assert_eq!(e.flags & BDL_IOC, BDL_IOC, "period {i} completes without an interrupt");
    }
}

#[test]
fn the_ring_length_is_exactly_what_the_cyclic_buffer_register_will_be_told() {
    /*
     * SD_CBL is programmed from this constant while the engine wraps on the
     * sum of the entries. The two have to be the same number.
     */
    assert_eq!(RING_BYTES, PERIOD_BYTES * N_PERIODS as u64);
    assert_eq!(build_bdl(RING_BASE).iter().map(|e| e.len as u64).sum::<u64>(), RING_BYTES);
}

#[test]
fn a_period_is_a_whole_number_of_forty_eight_kilohertz_stereo_frames() {
    /*
     * The stream format is 16-bit stereo, so a frame is four bytes. A period
     * that ends mid-frame swaps the channels for the rest of the ring.
     */
    assert_eq!(PERIOD_BYTES % 4, 0);
    assert_eq!(RING_BYTES % 4, 0);
}
