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

//! The mouse side: the aux clock on, the configuration written back with
//! both interrupts, the wheel knock, and the ports that are not a mouse.

use super::controller::{Keyboard, MouseKind};
use super::shared::{machine, FIRMWARE_CONFIG};
use crate::constants::{
    CONFIG_AUX_DISABLE, CONFIG_IRQ1, CONFIG_IRQ12, CTL_ENABLE_AUX, CTL_READ_CONFIG, CTL_WRITE_AUX,
    CTL_WRITE_CONFIG, MOUSE_ENABLE_REPORTING, MOUSE_GET_DEVICE_ID, MOUSE_SET_DEFAULTS,
    MOUSE_SET_SAMPLE_RATE,
};
use crate::init::enable_mouse;

const KNOCK: [u8; 9] = [
    MOUSE_SET_DEFAULTS,
    MOUSE_SET_SAMPLE_RATE,
    200,
    MOUSE_SET_SAMPLE_RATE,
    100,
    MOUSE_SET_SAMPLE_RATE,
    80,
    MOUSE_GET_DEVICE_ID,
    MOUSE_ENABLE_REPORTING,
];

#[test]
fn a_wheel_mouse_is_found_by_the_knock_and_reporting_is_enabled_last() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Present { wheel: true });
    assert_eq!(enable_mouse(7), Ok(true));
    let c = ctl.borrow();
    assert_eq!(&c.commands()[..3], [CTL_ENABLE_AUX, CTL_READ_CONFIG, CTL_WRITE_CONFIG]);
    assert!(c.commands()[3..].iter().all(|&cmd| cmd == CTL_WRITE_AUX), "then only mouse writes");
    let expected = (FIRMWARE_CONFIG | CONFIG_IRQ1 | CONFIG_IRQ12) & !CONFIG_AUX_DISABLE;
    assert_eq!(c.config, expected, "both interrupts on, aux clock on, port one untouched");
    assert_eq!(c.data_writes()[1..], KNOCK, "the configuration byte, then the mouse dialogue");
    assert!(c.mouse.reporting);
}

#[test]
fn a_plain_mouse_answers_the_knock_with_id_zero_and_is_still_enabled() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Present { wheel: false });
    assert_eq!(enable_mouse(7), Ok(false));
    assert!(ctl.borrow().mouse.reporting);
}

#[test]
fn an_aux_port_with_nothing_behind_it_fails_at_the_first_mouse_command() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Absent);
    assert_eq!(enable_mouse(7).err(), Some("ps2 output buffer empty"));
    assert_eq!(ctl.borrow().data_writes().last(), Some(&MOUSE_SET_DEFAULTS));
}

#[test]
fn an_aux_port_that_echoes_is_not_taken_for_a_mouse() {
    let (_ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Echoing);
    assert_eq!(enable_mouse(7).err(), Some("mouse command not acknowledged"));
}
