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

//! The report descriptor of a one-finger precision touchpad. Input report 1
//! carries the finger; feature report 2 carries the input mode and the
//! surface and button switches a host sets to get raw touch.

use super::finger::FINGER;
use super::mode::MODE;

pub const TOUCH_REPORT_ID: u8 = 1;
pub const MODE_REPORT_ID: u8 = 2;
/// Report id plus seven bytes of body.
pub const TOUCH_REPORT_LEN: u16 = 8;
pub const X_MAX: u16 = 1200;
pub const Y_MAX: u16 = 800;

/// The application collection: Usage Page (Digitizer), Usage (Touch Pad),
/// Collection (Application), then the two reports, then End Collection.
pub fn report_descriptor() -> Vec<u8> {
    let head: &[&[u8]] = &[&[0x05, 0x0D], &[0x09, 0x05], &[0xA1, 0x01]];
    let tail: &[&[u8]] = &[&[0xC0]];
    [head, FINGER, MODE, tail].concat().concat()
}
