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

//! The enable bit around a target-address change.
//!
//! IC_TAR is one of the registers the DesignWare core holds read-only while
//! IC_ENABLE is set, and set_target.rs is built around that: it reads the
//! enable state, disables if it has to, writes the address, and puts the
//! enable bit back only if it was the one that took it away. A write to an
//! enabled controller is dropped without error, so the next transfer goes to
//! whichever device was addressed before.

use nonos_devmodel::FakeBar;

use crate::constants::{IC_ENABLE, IC_TAR};
use crate::model::{live, stuck_enabled};
use crate::regs::Regs;
use crate::transaction::control::set_target;
use crate::transaction::TransferError;

const TOUCHPAD: u8 = 0x15;

/// Enable bit set, status already quiescent, so the disable takes on the
/// first read. The restore then waits on a status this passive window never
/// raises, so the restore is observed in IC_ENABLE, not in the return value.
fn enabled_and_quiescent() -> FakeBar {
    let bar = live();
    bar.present32(IC_ENABLE as usize, 1);
    bar
}

#[test]
fn a_disabled_controller_is_not_switched_on_behind_the_callers_back() {
    /*
     * Nothing had to be disabled, so nothing may be enabled afterwards.
     * Restoring an enable bit that was never set arms a controller the caller
     * believed was quiescent, and the first stray transfer is a real one.
     */
    let bar = live();
    assert!(set_target(Regs::new(bar.base()), TOUCHPAD).is_ok(), "target change refused");
    assert_eq!(bar.wrote32(IC_ENABLE as usize), 0, "a disabled controller was enabled");
}

#[test]
fn a_controller_that_will_not_disable_never_has_its_target_address_changed() {
    /*
     * If the disable does not take, the IC_TAR write is swallowed by the core
     * and the caller goes on to address a device it never selected. Refusing
     * is the only outcome that leaves the bus in a knowable state.
     */
    let bar = stuck_enabled();
    let err = set_target(Regs::new(bar.base()), TOUCHPAD).expect_err("must refuse");
    assert!(matches!(err, TransferError::Timeout));
    assert_eq!(bar.wrote32(IC_TAR as usize), 0, "IC_TAR written to an enabled controller");
}

#[test]
fn an_enabled_controller_has_its_enable_bit_put_back_after_the_address_changes() {
    /*
     * The caller handed over a running controller and must get one back. A
     * missing restore leaves the bus dead from the next transfer on, with the
     * address change as the only thing that happened in between.
     */
    let bar = enabled_and_quiescent();
    let _ = set_target(Regs::new(bar.base()), TOUCHPAD);
    assert_eq!(bar.wrote32(IC_TAR as usize), TOUCHPAD as u32);
    assert_eq!(bar.wrote32(IC_ENABLE as usize), 1, "the enable bit was not restored");
}
