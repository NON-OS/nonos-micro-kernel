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

//! The input events the HID driver posts, kept for the test to read back.
//! The record layout and the kind numbers are the ones in
//! userland/libc/src/surface_registry/types.rs.

use std::cell::RefCell;

use crate::raw;

pub const INPUT_KIND_POINTER_REL: u16 = 2;
pub const INPUT_KIND_WHEEL: u16 = 4;
pub const INPUT_KIND_BUTTON_DOWN: u16 = 5;
pub const INPUT_KIND_BUTTON_UP: u16 = 6;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InputEvent {
    pub kind: u16,
    pub flags: u16,
    pub code: u32,
    pub x: i32,
    pub y: i32,
    pub delta_x: i32,
    pub delta_y: i32,
    pub timestamp_ns: u64,
}

thread_local! {
    static EVENTS: RefCell<Vec<InputEvent>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_input_event_post(ev: *const InputEvent) -> i64 {
    let event = raw::load(ev);
    EVENTS.with(|e| e.borrow_mut().push(event));
    0
}

/// Everything posted so far, oldest first, leaving the record empty.
pub fn take_events() -> Vec<InputEvent> {
    EVENTS.with(|e| std::mem::take(&mut *e.borrow_mut()))
}
