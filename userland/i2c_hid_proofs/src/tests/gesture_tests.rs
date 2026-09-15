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

//! One finger on the gesture engine, with no bus in the way: motion, the
//! discontinuity that is a new finger, and the tap.

use super::frames::{finger, LIFTED};
use crate::input::gesture::TouchActions;
use crate::input::TouchGesture;

#[test]
fn motion_takes_two_frames_and_scales_to_the_nominal_pad_width() {
    let mut g = TouchGesture::default();
    assert_eq!(g.on_touch(&finger(100, 100)), TouchActions::default(), "one frame is not motion");
    // Forty units on a 1200-unit pad, at the gain the speed selects.
    assert_eq!(g.on_touch(&finger(140, 100)).motion, Some((26, 0)));
}

#[test]
fn a_jump_across_the_pad_is_a_new_finger_not_a_swipe() {
    let mut g = TouchGesture::default();
    g.on_touch(&finger(100, 100));
    assert_eq!(g.on_touch(&finger(1000, 100)).motion, None, "a discontinuity moved the pointer");
}

#[test]
fn a_tap_is_a_press_and_a_release() {
    let mut g = TouchGesture::default();
    g.on_touch(&finger(300, 300));
    let tap = g.on_touch(&LIFTED);
    assert!(tap.button_down && tap.button_up, "{tap:?}");
}

#[test]
fn a_dragged_finger_lifting_is_not_a_tap() {
    let mut g = TouchGesture::default();
    for x in (300..500).step_by(20) {
        g.on_touch(&finger(x, 300));
    }
    let lift = g.on_touch(&LIFTED);
    assert!(!lift.button_down && !lift.button_up, "a drag clicked: {lift:?}");
}
