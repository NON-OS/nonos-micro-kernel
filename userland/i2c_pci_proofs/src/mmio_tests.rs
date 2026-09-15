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

//! Whether the controller is behind the window at all.
//!
//! init/mod.rs:16-21 states the rule: "Prove MMIO actually reaches the
//! DesignWare core before trusting any other register. All-zeros means the
//! function is unpowered or clock-gated (every later read would return 0 too,
//! so disable() below would 'succeed' instantly and a dead controller would
//! bring up clean); all-ones means the BAR is not decoding."
//!
//! That is the whole value of the guard. Without it an unpowered function
//! reports a clean bring-up, the capsule binds it and announces an I2C bus,
//! and the touchpad never reports, with nothing anywhere recording that the
//! window was dead from the first read.

use crate::init::bring_up;
use crate::model::{live, refusal, stuck_at, CLOCK_HZ, DW_COMP_TYPE};
use crate::regs::Regs;

#[test]
fn a_controller_whose_window_reads_all_zero_is_refused_and_told_so() {
    let bar = stuck_at(0x00);
    let err = refusal(bring_up(Regs::new(bar.base()), CLOCK_HZ));
    assert!(err.contains("mmio dead"), "unexpected error: {err}");
}

#[test]
fn a_controller_whose_window_reads_all_ones_is_refused_and_told_so() {
    /*
     * All-ones is what a PCI read returns when nothing claims the address,
     * which is the shape of a BAR that was never programmed or a function
     * the firmware left hidden. It is not a value any live register holds,
     * so accepting it means binding a controller that is not there.
     */
    let bar = stuck_at(0xFF);
    let err = refusal(bring_up(Regs::new(bar.base()), CLOCK_HZ));
    assert!(err.contains("mmio dead"), "unexpected error: {err}");
}

#[test]
fn a_live_controller_is_accepted_and_reports_the_signature_it_read() {
    /*
     * The counterpart to the two refusals. A guard that rejects everything
     * passes both tests above and binds no hardware at all.
     */
    let bar = live();
    let state = bring_up(Regs::new(bar.base()), CLOCK_HZ).expect("bring-up");
    assert_eq!(state.comp_type, DW_COMP_TYPE);
}
