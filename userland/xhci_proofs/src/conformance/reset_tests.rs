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

//! Reset, readiness and start, against a controller that answers and one that does not.

use nonos_devmodel::run;

use super::devices::{dead_controller, host_controller};
use super::model::{controller, op_base, CAP_LEN};
use crate::constants::{USBCMD, USBCMD_INTE, USBCMD_RUN, USBSTS, USBSTS_HSE};
use crate::controller::{reset, start, wait_cnr_clear, wait_hc_running};
use crate::error::XhciError;

#[test]
fn reset_completes_when_the_controller_drops_hcrst() {
    let bar = controller(0);
    let _hc = run(&bar, host_controller);
    reset(op_base(&bar)).expect("reset completes");
    wait_cnr_clear(op_base(&bar)).expect("ready after reset");
}

#[test]
fn a_reset_the_controller_never_finishes_times_out() {
    let bar = controller(0);
    let _dead = run(&bar, dead_controller);
    assert_eq!(reset(op_base(&bar)), Err(XhciError::ResetTimeout));
}

#[test]
fn start_acknowledges_hse_by_writing_one_and_sets_run_and_inte() {
    /*
     * A passive window: the write is what is under test, and a device would
     * overwrite the status register before it could be read back.
     */
    let bar = controller(0);
    bar.present32(CAP_LEN + USBSTS as usize, USBSTS_HSE);
    start(op_base(&bar));
    assert_ne!(bar.wrote32(CAP_LEN + USBSTS as usize) & USBSTS_HSE, 0, "HSE is RW1C");
    let cmd = bar.wrote32(CAP_LEN + USBCMD as usize);
    assert_eq!(cmd & (USBCMD_RUN | USBCMD_INTE), USBCMD_RUN | USBCMD_INTE);
}

#[test]
fn running_is_observed_when_the_controller_clears_hch() {
    let bar = controller(0);
    let _hc = run(&bar, host_controller);
    start(op_base(&bar));
    wait_hc_running(op_base(&bar)).expect("HCH drops once running");
}
