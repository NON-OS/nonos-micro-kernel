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

//! Proofs for the touchpad gesture state machine: absolute move, tap-to-click,
//! drag (no accidental tap), two-finger scroll, and the clickpad button.

use crate::gesture::TouchGesture;

// One finger maps to an absolute cursor position scaled into the router's range.
#[test]
fn one_finger_moves_cursor() {
    let mut g = TouchGesture::default();
    let a = g.on_touch(500, 250, 1000, 1000, true, 1, false);
    assert_eq!(a.move_to, Some((16383, 8191)));
    assert!(!a.button_down && !a.button_up && a.wheel == 0);
}

// Finger down then up in place is a tap, delivered as a click.
#[test]
fn tap_is_a_click() {
    let mut g = TouchGesture::default();
    g.on_touch(500, 250, 1000, 1000, true, 1, false);
    let a = g.on_touch(500, 250, 1000, 1000, false, 0, false);
    assert!(a.button_down && a.button_up);
    assert_eq!(a.move_to, None);
}

// A finger that travels before lifting is a drag, not a tap.
#[test]
fn drag_does_not_click() {
    let mut g = TouchGesture::default();
    g.on_touch(100, 100, 1000, 1000, true, 1, false);
    g.on_touch(900, 100, 1000, 1000, true, 1, false);
    let a = g.on_touch(900, 100, 1000, 1000, false, 0, false);
    assert!(!a.button_down && !a.button_up);
}

// Two fingers scroll by the wheel and never move the cursor.
#[test]
fn two_fingers_scroll() {
    let mut g = TouchGesture::default();
    g.on_touch(500, 500, 1000, 1000, false, 2, false);
    let a = g.on_touch(500, 600, 1000, 1000, false, 2, false);
    assert_eq!(a.wheel, 6);
    assert_eq!(a.move_to, None);
}

// The physical clickpad press and release map to the left button.
#[test]
fn clickpad_button_maps_to_left() {
    let mut g = TouchGesture::default();
    assert!(g.on_touch(500, 250, 1000, 1000, true, 1, true).button_down);
    assert!(g.on_touch(500, 250, 1000, 1000, true, 1, false).button_up);
}
