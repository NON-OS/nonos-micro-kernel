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

//! The whole setup sequence, from the device list to the ready driver.

use nonos_libc::{acked, present, PIO_GRANT};

use super::controller::{Keyboard, MouseKind, CONFIG_KBD_DISABLE};
use super::shared::{machine, AUX, KBD};
use crate::setup::run;

#[test]
fn a_keyboard_and_a_wheel_mouse_come_up_together_and_both_lines_are_opened() {
    let (ctl, _port) = machine(Keyboard::SLOW, MouseKind::Present { wheel: true });
    present(&[KBD, AUX]);
    let driver = run().expect("ready");
    assert_eq!(
        (driver.pio_grant_id, driver.irq_grant_id, driver.aux_irq_grant_id),
        (PIO_GRANT, 101, 112)
    );
    assert!(driver.mouse_enabled && driver.mouse_wheel);
    assert_eq!(acked(), [101, 112]);
    assert_eq!(ctl.borrow().config & CONFIG_KBD_DISABLE, 0, "the keyboard is still clocked");
}

#[test]
fn a_late_keyboard_ack_is_not_taken_for_the_controller_configuration() {
    let (ctl, _port) = machine(Keyboard::SLOW, MouseKind::Present { wheel: false });
    present(&[KBD, AUX]);
    run().expect("ready");
    let c = ctl.borrow();
    assert_eq!(
        c.config & CONFIG_KBD_DISABLE,
        0,
        "an ACK written back as config would kill port one"
    );
    assert!(c.output.is_empty());
}
