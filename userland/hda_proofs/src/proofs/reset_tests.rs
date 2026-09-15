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

//! Taking the controller out of its global reset.
//!
//! Nothing else in the capsule works until GCTL.CRST reads back set: while it
//! is clear the controller holds every other register at zero, so a bring-up
//! that carries on regardless programs a ring nobody reads and probes codecs
//! that cannot answer, and the failure surfaces much later as a silent machine
//! rather than as a refusal here.
//!
//! The timeout on this wait is not exercised, and cannot be. The driver polls
//! the same register it just wrote, so a memory window answers the wait from
//! the driver's own store and a device model on another thread has no way to
//! get between the two instructions. See `nonos_devmodel::live` for why.

use nonos_devmodel::FakeBar;

use crate::constants::{GCTL, GCTL_CRST, STATESTS};
use crate::controller::leave_reset;
use crate::model::WINDOW;
use crate::regs::Regs;

/// GCTL's unsolicited response enable, a bit the driver has no business
/// touching while it clears the reset.
const GCTL_UNSOL: u32 = 1 << 8;

#[test]
fn a_controller_found_in_reset_is_brought_out_of_it() {
    let bar = FakeBar::new(WINDOW);
    assert!(leave_reset(Regs::new(bar.base())).is_ok(), "a live controller must leave reset");
    assert_eq!(bar.wrote32(GCTL as usize) & GCTL_CRST, GCTL_CRST);
}

#[test]
fn clearing_the_reset_preserves_every_other_bit_of_the_control_register() {
    /*
     * Firmware leaves configuration in GCTL that the driver never re-derives.
     * A blind write of GCTL_CRST rather than a read-modify-write drops it, and
     * losing the unsolicited response enable means the codec's jack detection
     * stops reporting for the rest of the boot.
     */
    let bar = FakeBar::new(WINDOW);
    bar.present32(GCTL as usize, GCTL_UNSOL);
    assert!(leave_reset(Regs::new(bar.base())).is_ok());
    assert_eq!(bar.wrote32(GCTL as usize), GCTL_UNSOL | GCTL_CRST);
}

#[test]
fn a_controller_already_running_is_not_pushed_back_through_a_reset() {
    /*
     * Re-asserting CRST on a controller that is already out of reset throws
     * away the codec state and the ring pointers a previous stage set up.
     */
    let bar = FakeBar::new(WINDOW);
    bar.present32(GCTL as usize, GCTL_CRST | GCTL_UNSOL);
    bar.present16(STATESTS as usize, 0x0001);
    assert!(leave_reset(Regs::new(bar.base())).is_ok());
    assert_eq!(bar.wrote32(GCTL as usize), GCTL_CRST | GCTL_UNSOL);
    assert_eq!(bar.wrote16(STATESTS as usize), 0x0001, "the codec latch was cleared");
    assert_eq!((GCTL, GCTL_CRST), (0x08, 1), "the reset is bit zero of the register at 0x08");
}
