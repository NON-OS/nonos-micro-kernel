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

use super::call::syscall;
use super::numbers::N_INPUT_EVENT_DRAIN;

pub const INPUT_KIND_KEY_DOWN: u16 = 0;
pub const INPUT_KIND_KEY_UP: u16 = 1;
pub const INPUT_KIND_POINTER_REL: u16 = 2;
pub const INPUT_KIND_POINTER_ABS: u16 = 3;
pub const INPUT_KIND_WHEEL: u16 = 4;
pub const INPUT_KIND_BUTTON_DOWN: u16 = 5;
pub const INPUT_KIND_BUTTON_UP: u16 = 6;
pub const INPUT_KIND_TOUCH: u16 = 7;

#[repr(C)]
#[derive(Clone, Copy, Default)]
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

pub fn input_drain(out: &mut [InputEvent]) -> i64 {
    syscall(N_INPUT_EVENT_DRAIN, [out.as_mut_ptr() as u64, out.len() as u64, 0, 0, 0, 0])
}
