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

//! More than one finger, a palm, and the click button on the gesture engine.

use super::frames::{button, finger, palm, two, LIFTED};
use crate::input::TouchGesture;

#[test]
fn two_fingers_scroll_and_the_pointer_stays_put_until_both_lift() {
    let mut g = TouchGesture::default();
    assert_eq!(g.on_touch(&two(300, 400)).wheel, 0, "the first frame only anchors the scroll");
    assert_eq!(g.on_touch(&two(300, 440)).wheel, 5, "40 units on an 800-unit pad is five steps");
    assert_eq!(
        g.on_touch(&finger(300, 440)).motion,
        None,
        "the remaining finger moved the pointer"
    );
    g.on_touch(&LIFTED);
    g.on_touch(&finger(300, 300));
    assert!(g.on_touch(&finger(340, 300)).motion.is_some(), "the pointer stayed frozen after lift");
}

#[test]
fn a_palm_freezes_the_pointer_until_it_lifts() {
    let mut g = TouchGesture::default();
    g.on_touch(&palm(300, 300));
    assert_eq!(g.on_touch(&palm(340, 300)).motion, None);
    assert_eq!(
        g.on_touch(&finger(380, 300)).motion,
        None,
        "a palm became a finger without lifting"
    );
    g.on_touch(&LIFTED);
    g.on_touch(&finger(300, 300));
    assert!(g.on_touch(&finger(340, 300)).motion.is_some());
}

#[test]
fn the_click_button_is_debounced_over_two_frames() {
    let mut g = TouchGesture::default();
    assert!(!g.on_touch(&button(true)).button_down, "one frame clicked");
    assert!(g.on_touch(&button(true)).button_down);
    g.on_touch(&button(false));
    assert!(g.on_touch(&button(false)).button_up);
}
