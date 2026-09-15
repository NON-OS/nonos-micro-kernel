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

//! Halting a running controller, against one that answers and one that does not.

use nonos_devmodel::run;

use super::devices::{dead_controller, host_controller};
use super::model::{controller, op_base, CAP_LEN};
use crate::constants::{USBCMD, USBCMD_RUN, USBSTS};
use crate::controller::halt;
use crate::error::XhciError;

#[test]
fn halt_clears_run_and_waits_for_the_controller_to_say_halted() {
    let bar = controller(0);
    bar.present32(CAP_LEN + USBCMD as usize, USBCMD_RUN);
    bar.present32(CAP_LEN + USBSTS as usize, 0);
    let _hc = run(&bar, host_controller);
    halt(op_base(&bar)).expect("a conforming controller halts");
    assert_eq!(bar.wrote32(CAP_LEN + USBCMD as usize) & USBCMD_RUN, 0);
}

#[test]
fn a_controller_that_never_halts_is_given_up_on() {
    let bar = controller(0);
    bar.present32(CAP_LEN + USBCMD as usize, USBCMD_RUN);
    bar.present32(CAP_LEN + USBSTS as usize, 0);
    let _dead = run(&bar, dead_controller);
    assert_eq!(halt(op_base(&bar)), Err(XhciError::HaltTimeout));
}
