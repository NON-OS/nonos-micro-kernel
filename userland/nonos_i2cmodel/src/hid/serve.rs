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

//! What each register holds when the host turns round and reads it.

use super::device::HidOverI2c;
use super::record::Command;

impl HidOverI2c {
    /// Act on a completed write phase. Only the command register does
    /// anything on a write; a bare register address is served when read.
    pub(super) fn interpret(&mut self) {
        let written = std::mem::take(&mut self.written);
        if self.pointer == Some(self.regs.command) && written.len() >= 4 {
            self.command(&written);
        }
    }

    pub(super) fn register_contents(&mut self) -> Vec<u8> {
        let Some(reg) = self.pointer else { return Vec::new() };
        let known = [self.regs.hid_desc, self.regs.report_desc, self.regs.input];
        if !known.contains(&reg) {
            self.commands.push(Command::UnknownRegister(reg));
            return Vec::new();
        }
        self.commands.push(Command::RegisterRead(reg));
        match reg {
            r if r == self.regs.hid_desc => self.descriptor.to_vec(),
            r if r == self.regs.report_desc => self.report_desc.clone(),
            _ => self.next_input(),
        }
    }

    /// The next queued report, or a zero-length one when nothing is pending.
    pub(super) fn next_input(&mut self) -> Vec<u8> {
        self.inputs.pop_front().unwrap_or_else(|| vec![0, 0])
    }
}

/// A report with the two-byte length the input and data registers carry.
pub(super) fn framed(report: &[u8]) -> Vec<u8> {
    let len = (report.len() as u16 + 2).to_le_bytes();
    [&len[..], report].concat()
}
