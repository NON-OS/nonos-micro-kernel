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

pub const KERNEL_STACK_SIZE: usize = 32768;
pub const IRQ_STACK_SIZE: usize = 8192;

#[repr(C, align(16))]
pub struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

impl KernelStack {
    pub const fn new() -> Self {
        Self { data: [0; KERNEL_STACK_SIZE] }
    }

    pub fn base(&self) -> u64 {
        self.data.as_ptr() as u64
    }

    pub fn top(&self) -> u64 {
        self.base() + KERNEL_STACK_SIZE as u64
    }
}

#[repr(C, align(16))]
pub struct IrqStack {
    data: [u8; IRQ_STACK_SIZE],
}

impl IrqStack {
    pub const fn new() -> Self {
        Self { data: [0; IRQ_STACK_SIZE] }
    }

    pub fn top(&self) -> u64 {
        self.data.as_ptr() as u64 + IRQ_STACK_SIZE as u64
    }
}
