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

use super::super::{key_event::KeyEvent, keymap, post_key};
use super::constants::CAP;
use super::types::Keyboard;

impl Keyboard {
    pub(in crate::hid::keyboard) fn push_key(&mut self, scancode: u8, pressed: bool) {
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
