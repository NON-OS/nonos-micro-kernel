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

use super::is_real_key::is_real_key;
use super::types::Keyboard;

impl Keyboard {
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
}
