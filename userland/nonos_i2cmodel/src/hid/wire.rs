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

//! The device as the bus sees it.

use super::device::HidOverI2c;
use crate::bus::{Dir, Target};

impl Target for HidOverI2c {
    fn address(&self) -> u8 {
        self.addr
    }

    fn start(&mut self, dir: Dir, repeated: bool) {
        self.begin(dir, repeated);
    }

    fn write(&mut self, byte: u8) -> bool {
        self.take(byte)
    }

    /// Past the end of what was addressed the line floats high.
    fn read(&mut self, _last: bool) -> u8 {
        let Some(&byte) = self.serving.get(self.served) else {
            self.over_reads += 1;
            return 0xFF;
        };
        self.served += 1;
        byte
    }

    /// A STOP completes whatever the write phase asked and forgets the
    /// register, as a strict device may.
    fn stop(&mut self) {
        if !self.written.is_empty() {
            self.interpret();
        }
        self.pointer = None;
        self.serving.clear();
    }
}
