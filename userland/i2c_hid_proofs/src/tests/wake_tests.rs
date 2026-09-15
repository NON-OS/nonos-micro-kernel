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

//! Waking the pad: SET_POWER ON, then RESET, then the zero-length report
//! the specification says a reset device sends first and the host must read
//! before any input arrives.

use nonos_i2cmodel::Command;

use super::fixture::rig;
use crate::setup;

#[test]
fn wake_powers_the_pad_on_before_resetting_it() {
    /*
     * A RESET to a sleeping device is ignored by some parts and answered by
     * others with nothing, so the order is not a nicety: reversed, the driver
     * waits on an acknowledgement that never comes.
     */
    let r = rig();
    let state = setup::run().expect("setup");
    assert!(state.woke, "the driver did not report the pad awake");
    let cmds = r.pad.lock().take_commands();
    let power = cmds.iter().position(|c| *c == Command::SetPower { sleep: false });
    let reset = cmds.iter().position(|c| *c == Command::Reset);
    match (power, reset) {
        (Some(p), Some(q)) => assert!(p < q, "reset before power-on: {cmds:?}"),
        _ => panic!("power-on or reset missing from {cmds:?}"),
    }
}

#[test]
fn the_reset_acknowledgement_is_drained_so_the_first_poll_sees_a_real_frame() {
    let r = rig();
    setup::run().expect("setup");
    let pad = r.pad.lock();
    assert_eq!(pad.pending_inputs(), 0, "the zero-length reset report was left queued");
    let after_reset = pad.commands().iter().skip_while(|c| **c != Command::Reset).skip(1);
    assert!(
        after_reset.clone().any(|c| matches!(c, Command::InputRead | Command::RegisterRead(3))),
        "nothing read the input register after the reset: {:?}",
        pad.commands()
    );
}
