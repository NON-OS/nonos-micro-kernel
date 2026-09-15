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

//! Two fingers and a palm, through the shipping poll.

use nonos_i2cmodel::touchpad::Touch;
use nonos_libc::{take_events, INPUT_KIND_POINTER_REL, INPUT_KIND_WHEEL};

use super::driven::{bound, frame};
use super::fixture::of_kind;

#[test]
fn two_fingers_moving_down_scroll() {
    let (r, mut s) = bound();
    let two = |y| Touch { x: 300, y, tip: true, confidence: true, contacts: 2, button: false };
    frame(&r, &mut s, two(400));
    frame(&r, &mut s, two(440));
    let wheel = of_kind(&take_events(), INPUT_KIND_WHEEL);
    assert_eq!(wheel.len(), 1);
    assert_eq!(wheel[0].delta_y, 5, "40 units on an 800-unit pad is five steps");
}

#[test]
fn a_palm_moves_nothing() {
    let (r, mut s) = bound();
    let palm = |x| Touch { x, y: 400, tip: true, confidence: false, contacts: 1, button: false };
    frame(&r, &mut s, palm(300));
    frame(&r, &mut s, palm(340));
    let events = take_events();
    assert!(of_kind(&events, INPUT_KIND_POINTER_REL).is_empty(), "{events:?}");
}
