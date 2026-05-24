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

use super::{key_event::KeyEvent, keymap, post_key};

const CAP: usize = 64;

pub struct Keyboard {
    prev: [u8; 6],
    modifiers: u8,
    caps_lock: bool,
    events: VecDeque<KeyEvent>,
    post_failures: u64,
}

impl Keyboard {
    pub fn new() -> Self {
        Self { prev: [0; 6], modifiers: 0, caps_lock: false, events: VecDeque::new(), post_failures: 0 }
    }

    pub fn feed(&mut self, report: &[u8; 8]) {
        self.modifiers = report[0];
        let keys = [report[2], report[3], report[4], report[5], report[6], report[7]];
        for key in keys {
            if is_real_key(key) && !self.prev.contains(&key) {
                self.push_key(key, true);
            }
        }
        for key in self.prev {
            if is_real_key(key) && !keys.contains(&key) {
                self.push_key(key, false);
            }
        }
        self.prev = keys;
    }

    pub fn pop(&mut self) -> Option<KeyEvent> { self.events.pop_front() }

    pub fn pending(&self) -> u32 { self.events.len() as u32 }

    pub fn post_failures(&self) -> u64 { self.post_failures }

    fn push_key(&mut self, scancode: u8, pressed: bool) {
        if pressed && keymap::is_caps_lock(scancode) {
            self.caps_lock = !self.caps_lock;
        }
        let ascii =
            if pressed { keymap::ascii(scancode, self.modifiers, self.caps_lock) } else { 0 };
        let event = KeyEvent { scancode, ascii, modifiers: self.modifiers, pressed };
        if !post_key::publish(event) {
            self.post_failures = self.post_failures.wrapping_add(1);
        }
        if self.events.len() < CAP {
            self.events.push_back(event);
        }
    }
}

fn is_real_key(key: u8) -> bool { key > 1 }
