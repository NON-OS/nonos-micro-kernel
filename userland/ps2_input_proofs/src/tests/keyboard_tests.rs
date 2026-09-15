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

//! The keyboard side: the port enabled, scanning started, the answer eaten.

use super::controller::{Keyboard, MouseKind, CONFIG_KBD_DISABLE};
use super::shared::machine;
use crate::constants::{CTL_ENABLE_KBD, KBD_ENABLE_SCANNING};
use crate::init::{enable_keyboard, flush_output};

#[test]
fn the_first_port_is_enabled_then_scanning_started_and_the_ack_consumed() {
    let (ctl, _port) = machine(Keyboard::SLOW, MouseKind::Absent);
    enable_keyboard(7).expect("keyboard up");
    let c = ctl.borrow();
    assert_eq!(c.commands(), [CTL_ENABLE_KBD], "the port firmware left off is turned on");
    assert_eq!(c.data_writes(), [KBD_ENABLE_SCANNING]);
    assert_eq!(c.config & CONFIG_KBD_DISABLE, 0);
    assert!(c.output.is_empty(), "the late acknowledgement was waited for and eaten");
}

#[test]
fn a_keyboard_that_never_answers_is_tolerated() {
    let (ctl, _port) = machine(Keyboard::DEAD, MouseKind::Absent);
    enable_keyboard(7).expect("a dead keyboard is not a failed bring-up");
    assert_eq!(ctl.borrow().data_writes(), [KBD_ENABLE_SCANNING]);
}

#[test]
fn a_controller_whose_input_buffer_never_drains_is_given_up_on() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Absent);
    ctl.borrow_mut().input_stuck = true;
    assert_eq!(enable_keyboard(7).err(), Some("kbd input buffer busy"));
    assert!(ctl.borrow().writes.is_empty(), "nothing was written into a full buffer");
}

#[test]
fn without_a_port_behind_the_grant_the_first_read_is_the_error() {
    assert_eq!(enable_keyboard(7).err(), Some("kbd status read failed"));
}

#[test]
fn flushing_drains_stale_bytes_up_to_its_bound() {
    let (ctl, _port) = machine(Keyboard::PROMPT, MouseKind::Absent);
    for b in 0..5u8 {
        ctl.borrow_mut().output.push_back((b, false));
    }
    flush_output(7);
    assert!(ctl.borrow().output.is_empty());
    for b in 0..40u8 {
        ctl.borrow_mut().output.push_back((b, false));
    }
    flush_output(7);
    assert_eq!(ctl.borrow().output.len(), 24, "sixteen bytes a flush, then stop");
}
