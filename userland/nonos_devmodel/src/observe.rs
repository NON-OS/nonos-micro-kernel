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

//! Reading back what the driver put in the window.
//!
//! A register a device defines as write-only still lands here, so a test can
//! assert on values real hardware would consume and never show back. That is
//! most of what a bring-up sequence is: a queue address, a page number, a
//! status byte the device acts on.

use core::sync::atomic::Ordering;

use super::bar::FakeBar;

impl FakeBar {
    /// What the driver last wrote at `offset`.
    ///
    /// Safe to call while a device model is running against the same window:
    /// the read is atomic, so it observes some byte the driver wrote rather
    /// than a tear. It is not a snapshot of the whole window, and a multi-byte
    /// read below can straddle a concurrent write, which is the same thing a
    /// real device sees.
    pub fn wrote8(&self, offset: usize) -> u8 {
        self.cell(offset).load(Ordering::Acquire)
    }

    pub fn wrote16(&self, offset: usize) -> u16 {
        u16::from_le_bytes([self.wrote8(offset), self.wrote8(offset + 1)])
    }

    pub fn wrote32(&self, offset: usize) -> u32 {
        u32::from_le_bytes([
            self.wrote8(offset),
            self.wrote8(offset + 1),
            self.wrote8(offset + 2),
            self.wrote8(offset + 3),
        ])
    }
}
