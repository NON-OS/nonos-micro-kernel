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

use nonos_libc::{InputEvent, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL, INPUT_KIND_TOUCH};

const ABS_RANGE_MAX: i64 = 0x7FFF;

pub struct CursorState {
    pub x: i32,
    pub y: i32,
    max_x: i32,
    max_y: i32,
    pub configured: bool,
    pub mult_x2: i32,
}

impl CursorState {
    pub const fn new() -> Self {
        Self { x: 512, y: 384, max_x: 1023, max_y: 767, configured: false, mult_x2: 2 }
    }

    pub fn configure(&mut self, width: u32, height: u32) {
        self.max_x = width.saturating_sub(1).min(i32::MAX as u32) as i32;
        self.max_y = height.saturating_sub(1).min(i32::MAX as u32) as i32;
        self.x = self.max_x / 2;
        self.y = self.max_y / 2;
        self.configured = true;
    }

    pub fn apply(&mut self, ev: &InputEvent) -> (u32, u32) {
        if ev.kind == INPUT_KIND_POINTER_REL {
            self.x = self.x.saturating_add(ev.delta_x.saturating_mul(self.mult_x2) / 2);
            self.y = self.y.saturating_add(ev.delta_y.saturating_mul(self.mult_x2) / 2);
        }
        if ev.kind == INPUT_KIND_POINTER_ABS || ev.kind == INPUT_KIND_TOUCH {
            self.x = (ev.x as i64 * self.max_x as i64 / ABS_RANGE_MAX) as i32;
            self.y = (ev.y as i64 * self.max_y as i64 / ABS_RANGE_MAX) as i32;
        }
        self.clamp();
        (self.x as u32, self.y as u32)
    }

    fn clamp(&mut self) {
        self.x = self.x.clamp(0, self.max_x);
        self.y = self.y.clamp(0, self.max_y);
    }
}
