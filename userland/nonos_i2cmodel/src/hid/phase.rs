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

//! The address phase and the write phase, from the device's side.

use super::device::HidOverI2c;
use super::record::Command;
use crate::bus::Dir;

impl HidOverI2c {
    /// A fresh START drops any register from the last transaction. A read
    /// with no register addressed is a read of the input register, which is
    /// how a host collects the report the device signalled with its
    /// interrupt line. A repeated START into a read serves the register the
    /// write phase named, or the report a GET_REPORT asked for.
    pub(super) fn begin(&mut self, dir: Dir, repeated: bool) {
        if !repeated {
            self.pointer = None;
            self.written.clear();
        }
        if dir != Dir::Read {
            return;
        }
        if repeated && !self.written.is_empty() {
            self.interpret();
        }
        if self.pointer.is_none() {
            self.commands.push(Command::InputRead);
            self.serving = self.next_input();
        } else if self.pointer != Some(self.regs.command) {
            self.serving = self.register_contents();
        }
        self.served = 0;
    }

    /// Two bytes name the register; whatever follows is the command or data.
    pub(super) fn take(&mut self, byte: u8) -> bool {
        if !self.acks_data {
            return false;
        }
        self.written.push(byte);
        if self.written.len() == 2 {
            self.pointer = Some(u16::from_le_bytes([self.written[0], self.written[1]]));
        }
        true
    }
}
