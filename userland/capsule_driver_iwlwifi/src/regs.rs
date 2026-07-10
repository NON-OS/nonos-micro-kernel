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

#[derive(Clone, Copy)]
pub struct Regs {
    base: usize,
}

impl Regs {
    pub const fn new(base: u64) -> Self {
        Self { base: base as usize }
    }
    pub fn read32(&self, off: usize) -> u32 {
        unsafe { core::ptr::read_volatile((self.base + off) as *const u32) }
    }
    pub fn write32(&self, off: usize, val: u32) {
        unsafe { core::ptr::write_volatile((self.base + off) as *mut u32, val) }
    }
    pub fn set_bits(&self, off: usize, bits: u32) {
        self.write32(off, self.read32(off) | bits);
    }
    pub fn poll_set(&self, off: usize, mask: u32, iters: usize) -> bool {
        for _ in 0..iters {
            if self.read32(off) & mask == mask {
                return true;
            }
            core::hint::spin_loop();
        }
        false
    }
}

/// Minimal 32-bit MMIO interface. The firmware-load path is written against
/// this instead of `Regs` directly so a host proof can drive it with a modeled
/// device (recording every write and simulating the FH completion) and check
/// the exact register sequence, with no hardware.
pub trait Mmio {
    fn read32(&self, off: usize) -> u32;
    fn write32(&self, off: usize, val: u32);
}

impl Mmio for Regs {
    fn read32(&self, off: usize) -> u32 {
        Regs::read32(self, off)
    }
    fn write32(&self, off: usize, val: u32) {
        Regs::write32(self, off, val)
    }
}
