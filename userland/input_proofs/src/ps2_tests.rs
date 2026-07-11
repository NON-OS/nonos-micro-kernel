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

//! Proofs for the PS/2 mouse packet decoder, including the overflow handling
//! that keeps a real controller from jumping the cursor on fast motion.

use crate::packet::{
    parse, BUTTON_LEFT, BUTTON_MIDDLE, BUTTON_RIGHT, FLAG_X_OVERFLOW, FLAG_Y_OVERFLOW,
};

// Bit 3 of byte 0 is always set on a real packet; without it we are out of sync.
#[test]
fn rejects_unsynced_first_byte() {
    assert!(parse(&[0x00, 10, 10]).is_none());
}

#[test]
fn decodes_each_button() {
    assert_eq!(parse(&[0x08 | 0x01, 0, 0]).unwrap().buttons, BUTTON_LEFT);
    assert_eq!(parse(&[0x08 | 0x02, 0, 0]).unwrap().buttons, BUTTON_RIGHT);
    assert_eq!(parse(&[0x08 | 0x04, 0, 0]).unwrap().buttons, BUTTON_MIDDLE);
}

#[test]
fn signed_x_movement() {
    assert_eq!(parse(&[0x08, 5, 0]).unwrap().dx, 5);
    // X sign bit (0x10) with 0xFB is -5 in the 9-bit encoding.
    assert_eq!(parse(&[0x08 | 0x10, 0xFB, 0]).unwrap().dx, -5);
}

// Screen Y grows downward while the mouse reports Y growing upward, so dy is
// inverted.
#[test]
fn y_axis_is_inverted() {
    assert_eq!(parse(&[0x08, 0, 5]).unwrap().dy, -5);
}

// The core hardening proof: an overflow must not surface the garbage low byte,
// it must cap to a bounded step in the reported direction.
#[test]
fn x_overflow_caps_instead_of_jumping() {
    let pos = parse(&[0x08 | 0x40, 0x7F, 0]).unwrap();
    assert_eq!(pos.dx, 255);
    assert_eq!(pos.flags & FLAG_X_OVERFLOW, FLAG_X_OVERFLOW);

    let neg = parse(&[0x08 | 0x40 | 0x10, 0x03, 0]).unwrap();
    assert_eq!(neg.dx, -255);
}

#[test]
fn y_overflow_caps_and_inverts() {
    let ev = parse(&[0x08 | 0x80, 0, 0x7F]).unwrap();
    assert_eq!(ev.dy, -255);
    assert_eq!(ev.flags & FLAG_Y_OVERFLOW, FLAG_Y_OVERFLOW);
}

// A short frame is never decoded, and a 4-byte IntelliMouse frame yields the
// scroll wheel step from the fourth byte.
#[test]
fn wheel_from_fourth_byte() {
    assert!(parse(&[0x08, 1]).is_none());
    assert_eq!(parse(&[0x08, 0, 0, 0xFF]).unwrap().dz, -1);
    assert_eq!(parse(&[0x08, 0, 0, 0x01]).unwrap().dz, 1);
    // Three bytes still decode with no wheel movement.
    assert_eq!(parse(&[0x08, 0, 0]).unwrap().dz, 0);
}
