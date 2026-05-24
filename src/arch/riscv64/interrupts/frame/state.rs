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

use crate::arch::riscv64::cpu::csr::SSTATUS_SPP;

use super::fields::TrapFrame;

pub const FRAME_SIZE: usize = core::mem::size_of::<TrapFrame>();

impl TrapFrame {
    pub fn is_interrupt(&self) -> bool {
        (self.scause >> 63) != 0
    }

    pub fn exception_code(&self) -> usize {
        self.scause & ((1 << 63) - 1)
    }

    pub fn is_from_user(&self) -> bool {
        (self.sstatus & SSTATUS_SPP) == 0
    }

    pub fn return_address(&self) -> usize {
        self.sepc
    }

    pub fn faulting_address(&self) -> usize {
        self.stval
    }

    pub fn set_return_value(&mut self, value: usize) {
        self.a0 = value;
    }

    pub fn syscall_number(&self) -> usize {
        self.a7
    }

    pub fn advance_pc(&mut self) {
        self.sepc += 4;
    }
}
