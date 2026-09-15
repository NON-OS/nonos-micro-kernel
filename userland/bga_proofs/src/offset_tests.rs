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

//! Where each DISPI index lands inside the register BAR.
//!
//! This is the arithmetic every other register access in the capsule is built
//! on, and it needs no device at all. Getting it wrong does not fail: it
//! writes a real resolution into a neighbouring register, so the adapter
//! takes a mode nobody asked for and the fault looks like a display problem
//! rather than an addressing one.

use crate::constants::{
    DISPI_INDEX_BPP, DISPI_INDEX_ENABLE, DISPI_INDEX_XRES, DISPI_INDEX_YRES, DISPI_IOPORT_OFFSET,
};
use crate::dispi::dispi_off;

#[test]
fn each_dispi_index_is_two_bytes_past_the_one_before_it() {
    /*
     * The DISPI block is an array of 16-bit registers based at 0x500. An
     * index scaled by anything but two overlaps its neighbour.
     */
    assert_eq!(dispi_off(0), DISPI_IOPORT_OFFSET);
    assert_eq!(dispi_off(DISPI_INDEX_XRES), 0x502);
    assert_eq!(dispi_off(DISPI_INDEX_YRES), 0x504);
    assert_eq!(dispi_off(DISPI_INDEX_BPP), 0x506);
    assert_eq!(dispi_off(DISPI_INDEX_ENABLE), 0x508);
}

#[test]
fn no_two_mode_registers_share_an_offset() {
    /*
     * The four registers a mode set touches must be distinct, in order, and
     * two bytes apart. A collision here would make the last write of the
     * sequence quietly overwrite an earlier one, which on this interface
     * means the adapter is enabled at a resolution that was never programmed.
     */
    let block = [DISPI_INDEX_XRES, DISPI_INDEX_YRES, DISPI_INDEX_BPP, DISPI_INDEX_ENABLE];
    for pair in block.windows(2) {
        assert_eq!(dispi_off(pair[1]) - dispi_off(pair[0]), 2, "indices {pair:?} overlap");
    }
}
