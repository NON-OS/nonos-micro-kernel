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

//! Port reset: power, connect, reset, and the change bits acknowledged.

use nonos_devmodel::run;

use super::model::{controller, op_base, PORT1};
use super::port_device::attached;
use crate::constants::{PORTSC_PED, PORTSC_PRC};
use crate::controller::reset_port;
use crate::error::XhciError;

#[test]
fn a_port_is_powered_reset_and_left_enabled_with_its_change_acknowledged() {
    let bar = controller(0);
    let _device = run(&bar, attached(true));
    reset_port(op_base(&bar), 1).expect("reset completes");
    /*
     * Not asserted on the value the driver returned: it reads PORTSC back in
     * the instruction after its own write, which in memory is that write and
     * on silicon is the register. What the specification fixes is what the
     * port shows once the device has taken the acknowledgement: enabled, and
     * the change bit down. The model clears it only on the driver's write of
     * one, so this is the driver's W1C, observed. A read that lands between
     * the driver's write and the model's answer sees the write itself, so
     * the wait is for the device-shown state, not for any value with the
     * change bit down.
     */
    let settled = (0..1_000_000)
        .map(|_| bar.wrote32(PORT1))
        .find(|v| v & (PORTSC_PRC | PORTSC_PED) == PORTSC_PED);
    assert!(settled.is_some(), "the device shows the port enabled with the change acknowledged");
}

#[test]
fn a_port_with_nothing_attached_is_refused() {
    let bar = controller(0);
    assert_eq!(reset_port(op_base(&bar), 1), Err(XhciError::NoDeviceOnPort));
}

#[test]
fn a_reset_the_port_never_completes_times_out() {
    let bar = controller(0);
    let _device = run(&bar, attached(false));
    assert_eq!(reset_port(op_base(&bar), 1), Err(XhciError::PortResetTimeout));
}
