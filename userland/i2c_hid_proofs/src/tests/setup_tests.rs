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

//! Finding the pad: by the firmware's word, by scanning, or not at all.

use nonos_i2cmodel::{touchpad, Command, HidOverI2c};

use super::fixture::{pad, rig, rig_with, HID_DESC_REG, MAX_INPUT, PAD};
use crate::setup;

#[test]
fn the_firmware_named_pad_is_bound_and_its_descriptor_read() {
    let r = rig();
    let state = setup::run().expect("setup");
    assert!(state.found(), "the pad was not bound");
    assert_eq!(state.addr, PAD);
    assert_eq!(&state.descriptor, r.pad.lock().descriptor());
    assert_eq!(state.input_register, 0x0003, "input register misread from the descriptor");
    assert_eq!(state.input_len, MAX_INPUT as usize, "max input length misread");
    assert_eq!(r.pad.lock().commands()[0], Command::RegisterRead(HID_DESC_REG));
}

#[test]
fn without_a_firmware_hint_the_bus_scan_finds_the_pad() {
    let _r = rig_with(pad(), None);
    let state = setup::run().expect("setup");
    assert_eq!((state.found(), state.addr), (true, PAD));
}

#[test]
fn a_wrong_firmware_hint_falls_back_to_the_scan() {
    /*
     * Multi-SKU firmware names pads that were never fitted. The hint is a
     * first guess, not a verdict, and the bus decides.
     */
    let _r = rig_with(pad(), Some((0x2C, HID_DESC_REG)));
    let state = setup::run().expect("setup");
    assert_eq!(state.addr, PAD);
}

#[test]
fn a_pad_at_an_address_the_scan_never_tries_is_absent_not_fatal() {
    let stray = HidOverI2c::new(0x3A, touchpad::report_descriptor(), MAX_INPUT, 0x04F3, 0x3028);
    let _r = rig_with(stray, None);
    let state = setup::run().expect("an empty bus is not a setup failure");
    assert!(!state.found(), "bound something on a bus with no reachable pad");
    assert_eq!(state.probes, 1);
}

#[test]
fn without_a_controller_service_setup_refuses() {
    assert!(setup::run().is_err(), "setup succeeded with no controller to talk to");
}
