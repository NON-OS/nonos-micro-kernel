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

//! The setup sequence on machines missing a piece: the keyboard alone is a
//! working machine, and nothing at all is not.

use nonos_libc::{acked, present};

use super::controller::{Keyboard, MouseKind, CONFIG_KBD_DISABLE};
use super::shared::{machine, AUX, KBD};
use crate::constants::{CTL_DISABLE_AUX, CTL_ENABLE_AUX, CTL_WRITE_AUX, KBD_ENABLE_SCANNING};
use crate::setup::run;

#[test]
fn an_echoing_aux_port_ends_switched_off_with_the_keyboard_still_scanning() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Echoing);
    present(&[KBD, AUX]);
    let driver = run().expect("the keyboard alone is a working machine");
    assert!(!driver.mouse_enabled);
    let c = ctl.borrow();
    assert_eq!(c.commands().last(), Some(&CTL_DISABLE_AUX), "the aux clock was turned back off");
    assert!(c.data_writes().contains(&KBD_ENABLE_SCANNING));
    assert_eq!(c.config & CONFIG_KBD_DISABLE, 0);
}

#[test]
fn a_machine_without_an_aux_device_never_touches_the_aux_port() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Present { wheel: true });
    present(&[KBD]);
    let driver = run().expect("ready");
    assert!(!driver.mouse_enabled && driver.aux_irq_grant_id == 0);
    let cmds = ctl.borrow().commands();
    assert!(!cmds.contains(&CTL_ENABLE_AUX) && !cmds.contains(&CTL_WRITE_AUX));
    assert_eq!(acked(), [101]);
}

#[test]
fn no_keyboard_in_the_list_is_refused_before_any_port_is_touched() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Absent);
    present(&[AUX]);
    assert_eq!(run().err(), Some("ps2 keyboard not present in device list"));
    assert!(ctl.borrow().writes.is_empty());
}
