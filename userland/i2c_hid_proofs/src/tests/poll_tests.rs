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

//! From a frame in the pad's input register to a posted pointer event, the
//! whole way through the shipping poll: one finger, and no finger.

use nonos_i2cmodel::touchpad::Touch;
use nonos_libc::{
    take_events, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_REL,
};

use super::driven::{bound, frame};
use super::fixture::of_kind;
use crate::input::poll;

#[test]
fn a_finger_sliding_right_posts_motion_to_the_right_and_nothing_else() {
    let (r, mut s) = bound();
    frame(&r, &mut s, Touch::finger(100, 100));
    frame(&r, &mut s, Touch::finger(140, 100));
    let events = take_events();
    let motion = of_kind(&events, INPUT_KIND_POINTER_REL);
    assert_eq!(motion.len(), 1, "{events:?}");
    assert!(motion[0].delta_x > 0 && motion[0].delta_y == 0, "{:?}", motion[0]);
    assert!(of_kind(&events, INPUT_KIND_BUTTON_DOWN).is_empty(), "a slide clicked");
}

#[test]
fn a_touch_and_lift_is_a_click_on_button_one() {
    let (r, mut s) = bound();
    frame(&r, &mut s, Touch::finger(300, 300));
    frame(&r, &mut s, Touch::lifted());
    let events = take_events();
    let kinds: Vec<u16> = events.iter().map(|e| e.kind).filter(|k| *k < 0x100).collect();
    assert_eq!(kinds, [INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP], "{events:?}");
    assert!(events.iter().filter(|e| e.kind < 0x100).all(|e| e.code == 1));
}

#[test]
fn an_empty_input_register_is_a_quiet_poll() {
    let (_r, mut s) = bound();
    poll(&mut s);
    assert!(take_events().is_empty(), "events from a pad with nothing to say");
    assert_eq!(s.input_polls, 1);
}
