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

//! The command register (section 7.2 of the specification).

use super::device::HidOverI2c;
use super::record::Command;
use super::serve::framed;

const OPCODE_RESET: u8 = 0x1;
const OPCODE_GET_REPORT: u8 = 0x2;
const OPCODE_SET_REPORT: u8 = 0x3;
const OPCODE_SET_POWER: u8 = 0x8;

impl HidOverI2c {
    /// `w` is the whole write phase: two bytes of register, then byte 2 with
    /// the report id (3:0) and type (5:4), byte 3 with the opcode. GET and
    /// SET name the data register next; SET follows it with a two-byte
    /// length and the report, id first.
    pub(super) fn command(&mut self, w: &[u8]) {
        let (ty, id) = (w[2] >> 4, w[2] & 0x0F);
        match w[3] & 0x0F {
            OPCODE_RESET => {
                self.commands.push(Command::Reset);
                self.inputs.push_front(vec![0, 0]);
            }
            OPCODE_SET_POWER => {
                self.commands.push(Command::SetPower { sleep: w[2] & 0x03 != 0 });
            }
            OPCODE_GET_REPORT => {
                self.commands.push(Command::GetReport { ty, id });
                let report = self.features.get(&id).cloned().unwrap_or_else(|| vec![id]);
                self.serving = framed(&report);
            }
            OPCODE_SET_REPORT if w.len() >= 8 => {
                let data = w[8..].to_vec();
                self.commands.push(Command::SetReport { ty, id, data: data.clone() });
                self.features.insert(id, data);
            }
            _ => {}
        }
    }
}
