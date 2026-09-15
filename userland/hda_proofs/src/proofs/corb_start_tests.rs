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

//! The state the ring bring-up leaves the controller in.
//!
//! Both DMA engines have to end up running and both ring pointers have to end
//! up agreeing with the controller's own idea of where it is reading. Neither
//! failure is loud: a stopped engine turns every later verb into a full
//! timeout, and a pointer left one slot out reads each response from the
//! previous command's slot, so the codec walk reports the wrong widgets and
//! the capsule configures a path that does not exist.

use crate::constants::{
    CORBCTL, CORBCTL_RUN, CORBRP, CORBRP_RST, CORBWP, RINTCNT, RINTCNT_ONE, RIRBCTL, RIRBCTL_DMAEN,
    RIRBCTL_RINTCTL, RIRBWP, RIRBWP_RST,
};
use crate::proofs::corb_tests::brought_up;

#[test]
fn both_dma_engines_are_left_running_with_the_response_interrupt_armed() {
    let bar = brought_up();
    assert_eq!(bar.wrote8(CORBCTL as usize), CORBCTL_RUN, "the command engine is stopped");
    assert_eq!(bar.wrote8(RIRBCTL as usize), RIRBCTL_DMAEN | RIRBCTL_RINTCTL);
    assert_eq!(bar.wrote16(RINTCNT as usize), RINTCNT_ONE, "responses must interrupt singly");
}

#[test]
fn the_command_read_pointer_is_left_with_its_reset_request_withdrawn() {
    /*
     * The reset is a request bit, not a state: leaving it asserted holds the
     * controller's read pointer at zero, so it never consumes a command.
     */
    let bar = brought_up();
    assert_eq!(bar.wrote16(CORBRP as usize) & CORBRP_RST, 0);
    assert_eq!(bar.wrote16(CORBWP as usize), 0, "the write pointer must start at slot zero");
}

#[test]
fn the_response_write_pointer_is_reset_by_asserting_its_request_bit() {
    /*
     * RIRBWP is the mirror image of CORBRP: it is cleared by writing the reset
     * bit, and the controller drops the bit itself. Writing a plain zero here
     * looks equivalent and leaves the pointer wherever firmware left it.
     */
    let bar = brought_up();
    assert_eq!(bar.wrote16(RIRBWP as usize), RIRBWP_RST);
}
