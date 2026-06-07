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

use alloc::collections::VecDeque;

use super::mouse_event::MouseEvent;
use super::post_mouse;

const CAP: usize = 64;

pub struct Mouse {
    buttons: u8,
    events: VecDeque<MouseEvent>,
    post_failures: u64,
}

impl Mouse {
    pub fn new() -> Self {
        Self { buttons: 0, events: VecDeque::new(), post_failures: 0 }
    }

    pub fn feed(&mut self, report: &[u8]) {
        if report.len() < 3 {
            return;
        }
        let previous_buttons = self.buttons;
        let buttons = report[0] & 0x1f;
        let dx = report[1] as i8 as i16;
        let dy = report[2] as i8 as i16;
        let dz = if report.len() > 3 { report[3] as i8 } else { 0 };
        let moved = dx != 0 || dy != 0;
        let changed = buttons != self.buttons;
        if moved || dz != 0 || changed {
            let flags = u8::from(moved) | (u8::from(changed) << 1) | (u8::from(dz != 0) << 2);
            self.push(MouseEvent { dx, dy, dz, buttons, flags }, previous_buttons);
        }
        self.buttons = buttons;
    }

    pub fn pop(&mut self) -> Option<MouseEvent> {
        self.events.pop_front()
    }

    pub fn pending(&self) -> u32 {
        self.events.len() as u32
    }

    pub fn post_failures(&self) -> u64 {
        self.post_failures
    }

    fn push(&mut self, event: MouseEvent, previous_buttons: u8) {
        if !post_mouse::publish(event, previous_buttons) {
            self.post_failures = self.post_failures.wrapping_add(1);
        }
        if self.events.len() < CAP {
            self.events.push_back(event);
        }
    }
}
