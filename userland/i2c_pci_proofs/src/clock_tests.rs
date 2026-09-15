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

//! Programming the SCL counts, and only while it is allowed.
//!
//! init/mod.rs:45-46 states it: "Writes the standard and fast SCL count pairs
//! plus the SDA hold time. Only valid while IC_ENABLE is 0; these registers
//! are read-only once enabled."
//!
//! Read-only means the writes are dropped, not refused. A controller
//! configured while enabled reports every register write as accepted, then
//! emits no SCL clock at all, so every transfer times out on a bus that
//! measures electrically perfect.

use crate::constants::{
    IC_ENABLE, IC_FS_SCL_HCNT, IC_FS_SCL_LCNT, IC_SDA_HOLD, IC_SS_SCL_HCNT, IC_SS_SCL_LCNT,
};
use crate::init::bring_up;
use crate::init::scl::{fast, sda_hold, standard};
use crate::model::{live, refusal, stuck_enabled, CLOCK_HZ};
use crate::regs::Regs;

#[test]
fn the_counts_are_not_programmed_until_the_controller_is_confirmed_disabled() {
    /*
     * A controller that never drops its enable bit is the case the ordering
     * exists for. Bring-up has to give up at the disable wait, with the count
     * registers still untouched. Programming them first and disabling after
     * would pass on a part that disables promptly and silently lose the whole
     * clock configuration on one that does not.
     */
    let bar = stuck_enabled();
    let err = refusal(bring_up(Regs::new(bar.base()), CLOCK_HZ));
    assert!(err.contains("disable timeout"), "unexpected error: {err}");

    for reg in [IC_SS_SCL_HCNT, IC_SS_SCL_LCNT, IC_FS_SCL_HCNT, IC_FS_SCL_LCNT, IC_SDA_HOLD] {
        assert_eq!(bar.wrote32(reg as usize), 0, "register {reg:#x} programmed while enabled");
    }
}

#[test]
fn a_completed_bring_up_leaves_the_controller_disabled_and_its_counts_loaded() {
    /*
     * The other half: the counts that were computed are the counts left in
     * the window, and nothing along the way turned the controller back on.
     */
    let bar = live();
    bring_up(Regs::new(bar.base()), CLOCK_HZ).expect("bring-up");

    assert_eq!(bar.wrote32(IC_ENABLE as usize), 0, "bring-up must not enable the controller");
    assert_eq!(bar.wrote32(IC_SS_SCL_HCNT as usize), standard(CLOCK_HZ).hcnt);
    assert_eq!(bar.wrote32(IC_SS_SCL_LCNT as usize), standard(CLOCK_HZ).lcnt);
    assert_eq!(bar.wrote32(IC_FS_SCL_HCNT as usize), fast(CLOCK_HZ).hcnt);
    assert_eq!(bar.wrote32(IC_FS_SCL_LCNT as usize), fast(CLOCK_HZ).lcnt);
    assert_eq!(bar.wrote32(IC_SDA_HOLD as usize), sda_hold(CLOCK_HZ));
}
