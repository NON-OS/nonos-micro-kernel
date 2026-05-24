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

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct ExceptionFrame {
    pub x0: u64,
    pub x1: u64,
    pub x2: u64,
    pub x3: u64,
    pub x4: u64,
    pub x5: u64,
    pub x6: u64,
    pub x7: u64,
    pub x8: u64,
    pub x9: u64,
    pub x10: u64,
    pub x11: u64,
    pub x12: u64,
    pub x13: u64,
    pub x14: u64,
    pub x15: u64,
    pub x16: u64,
    pub x17: u64,
    pub x18: u64,
    pub x19: u64,
    pub x20: u64,
    pub x21: u64,
    pub x22: u64,
    pub x23: u64,
    pub x24: u64,
    pub x25: u64,
    pub x26: u64,
    pub x27: u64,
    pub x28: u64,
    pub x29: u64,
    pub x30: u64,
    pub sp: u64,
    pub elr: u64,
    pub spsr: u64,
    pub esr: u64,
    pub far: u64,
}

impl ExceptionFrame {
    pub const fn new() -> Self {
        Self {
            x0: 0,
            x1: 0,
            x2: 0,
            x3: 0,
            x4: 0,
            x5: 0,
            x6: 0,
            x7: 0,
            x8: 0,
            x9: 0,
            x10: 0,
            x11: 0,
            x12: 0,
            x13: 0,
            x14: 0,
            x15: 0,
            x16: 0,
            x17: 0,
            x18: 0,
            x19: 0,
            x20: 0,
            x21: 0,
            x22: 0,
            x23: 0,
            x24: 0,
            x25: 0,
            x26: 0,
            x27: 0,
            x28: 0,
            x29: 0,
            x30: 0,
            sp: 0,
            elr: 0,
            spsr: 0,
            esr: 0,
            far: 0,
        }
    }

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

    pub fn set_return_value(&mut self, value: u64) {
        self.x0 = value;
    }

    pub fn syscall_number(&self) -> u64 {
        self.x8
    }

    pub fn syscall_arg(&self, n: usize) -> u64 {
        match n {
            0 => self.x0,
            1 => self.x1,
            2 => self.x2,
            3 => self.x3,
            4 => self.x4,
            5 => self.x5,
            _ => 0,
        }
    }

}

pub const FRAME_SIZE: usize = core::mem::size_of::<ExceptionFrame>();
