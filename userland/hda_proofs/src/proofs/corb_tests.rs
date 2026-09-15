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

//! Describing the command and response rings to the controller.
//!
//! Both rings are named by a 64-bit physical address split across two
//! registers, and the controller fetches from them by DMA. An address landing
//! in the wrong half, or the two rings swapped, aims a bus master at memory
//! the driver did not allocate. That is not a stuck device, it is a write into
//! somebody else's page, which is why each half is asserted separately.

use nonos_devmodel::FakeBar;

use crate::constants::{
    CORBLBASE, CORBSIZE, CORBSIZE_256, CORBUBASE, RIRBLBASE, RIRBSIZE, RIRBSIZE_256, RIRBUBASE,
};
use crate::controller::corb;
use crate::model::WINDOW;
use crate::regs::Regs;

/// Two 128-byte-aligned ring addresses, deliberately unlike each other in both
/// halves so a swap or a truncation cannot read as a match.
pub const CORB_PA: u64 = 0x0000_0001_2345_6000;
pub const RIRB_PA: u64 = 0x0000_0007_ABCD_E080;

/// A window a full ring bring-up has already run against.
pub fn brought_up() -> FakeBar {
    let bar = FakeBar::new(WINDOW);
    assert!(corb::init(Regs::new(bar.base()), CORB_PA, RIRB_PA).is_ok(), "ring bring-up");
    bar
}

#[test]
fn each_ring_address_reaches_the_controller_whole_and_in_the_right_halves() {
    let bar = brought_up();
    assert_eq!(bar.wrote32(CORBLBASE as usize), CORB_PA as u32);
    assert_eq!(bar.wrote32(CORBUBASE as usize), (CORB_PA >> 32) as u32);
    assert_eq!(bar.wrote32(RIRBLBASE as usize), RIRB_PA as u32);
    assert_eq!(bar.wrote32(RIRBUBASE as usize), (RIRB_PA >> 32) as u32);
}

#[test]
fn each_ring_is_sized_to_match_the_memory_that_was_allocated_for_it() {
    /*
     * The size field is an encoding, not a count. Telling a controller it has
     * 256 entries when 16 were allocated has it wrap past the end of the
     * buffer and keep going, still by DMA.
     */
    let bar = brought_up();
    assert_eq!(bar.wrote8(CORBSIZE as usize), CORBSIZE_256);
    assert_eq!(bar.wrote8(RIRBSIZE as usize), RIRBSIZE_256);
}
