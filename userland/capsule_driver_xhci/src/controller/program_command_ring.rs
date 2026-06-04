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
use crate::regs::op::crcr_program;
use crate::rings::command::CommandRing;
pub fn program_command_ring(op_base: u64, ring: &CommandRing) {
    crcr_program(op_base, 0, 0);
    let v = ring.crcr_value();
    let rcs = (v & 0x1) as u8;
    crcr_program(op_base, v & !0x3F, rcs);
}
