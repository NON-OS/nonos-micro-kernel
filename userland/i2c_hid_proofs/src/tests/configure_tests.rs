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

//! Asking the pad for raw touch. A precision touchpad boots reporting as a
//! mouse; the host has to set input mode 3 and the surface and button
//! switches in the feature report, read-modify-write, or every frame is a
//! relative delta and the gesture engine never sees a finger.

use nonos_i2cmodel::touchpad::MODE_REPORT_ID;
use nonos_i2cmodel::Command;

use super::fixture::{pad, rig, rig_with, HID_DESC_REG, PAD};
use crate::setup;

const FEATURE: u8 = 3;
const INPUT_MODE_TOUCHPAD: u8 = 3;
const SURFACE_AND_BUTTON_SWITCHES: u8 = 0b11;
const WANTED: [u8; 3] = [MODE_REPORT_ID, INPUT_MODE_TOUCHPAD, SURFACE_AND_BUTTON_SWITCHES];

#[test]
fn setup_writes_touchpad_mode_and_both_switches_into_the_feature_report() {
    let r = rig();
    setup::run().expect("setup");
    let pad = r.pad.lock();
    let left = pad.feature(MODE_REPORT_ID);
    assert_eq!(left, Some(&WANTED[..]), "feature report left as {left:?}");
}

#[test]
fn the_report_is_read_before_it_is_written() {
    /*
     * Fields this driver does not know about live in the same report. A blind
     * write zeroes them; the read first is what keeps a vendor's defaults.
     */
    let r = rig();
    setup::run().expect("setup");
    let pad = r.pad.lock();
    let get = pad
        .commands()
        .iter()
        .position(|c| matches!(c, Command::GetReport { ty: FEATURE, id: MODE_REPORT_ID }));
    let set = pad
        .commands()
        .iter()
        .position(|c| matches!(c, Command::SetReport { ty: FEATURE, id: MODE_REPORT_ID, .. }));
    match (get, set) {
        (Some(g), Some(s)) => assert!(g < s, "SET_REPORT before GET_REPORT"),
        _ => panic!("GET or SET missing from {:?}", pad.commands()),
    }
}

#[test]
fn a_pad_already_in_touchpad_mode_is_left_alone() {
    let mut device = pad();
    device.set_feature(MODE_REPORT_ID, &WANTED);
    let r = rig_with(device, Some((PAD, HID_DESC_REG)));
    setup::run().expect("setup");
    let sets =
        r.pad.lock().commands().iter().filter(|c| matches!(c, Command::SetReport { .. })).count();
    assert_eq!(sets, 0, "rewrote a feature report that already held the wanted values");
}
