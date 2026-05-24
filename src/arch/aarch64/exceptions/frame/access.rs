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

use super::state::ExceptionFrame;

impl ExceptionFrame {
    pub fn from_exception_level(&self) -> u8 {
        ((self.spsr >> 2) & 0x3) as u8
    }

    pub fn is_from_el0(&self) -> bool {
        self.from_exception_level() == 0
    }

    pub fn is_from_el1(&self) -> bool {
        self.from_exception_level() == 1
    }

    pub fn exception_class(&self) -> u8 {
        ((self.esr >> 26) & 0x3F) as u8
    }

    pub fn instruction_specific_syndrome(&self) -> u32 {
        (self.esr & 0x01FF_FFFF) as u32
    }

    pub fn instruction_length(&self) -> bool {
        (self.esr & (1 << 25)) != 0
    }

    pub fn return_address(&self) -> u64 {
        self.elr
    }

    pub fn faulting_address(&self) -> u64 {
        self.far
    }

    pub fn link_register(&self) -> u64 {
        self.x30
    }

    pub fn frame_pointer(&self) -> u64 {
        self.x29
    }

    pub fn stack_pointer(&self) -> u64 {
        self.sp
    }
}
