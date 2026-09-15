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

//! What the device shows: writes into the window from the model's side.

use core::sync::atomic::Ordering;

use super::bar::FakeBar;

impl FakeBar {
    /// Present `value` at `offset`, as the device would.
    pub fn present8(&self, offset: usize, value: u8) {
        self.cell(offset).store(value, Ordering::Release);
    }

    pub fn present16(&self, offset: usize, value: u16) {
        for (i, b) in value.to_le_bytes().iter().enumerate() {
            self.present8(offset + i, *b);
        }
    }

    pub fn present32(&self, offset: usize, value: u32) {
        for (i, b) in value.to_le_bytes().iter().enumerate() {
            self.present8(offset + i, *b);
        }
    }
}
