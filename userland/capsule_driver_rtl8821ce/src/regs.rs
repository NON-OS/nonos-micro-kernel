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

//! Register access for the Realtek 8821CE. rtw88 works predominantly in 8-bit
//! MAC registers, so the byte accessors are primary; the power-on sequence and
//! everything above it is written against the `Mmio` trait rather than raw
//! volatile access, so the host proofs can drive the real code with a modeled
//! device and check the exact register program without a card.

#[derive(Clone, Copy)]
pub struct Regs {
    base: usize,
}

impl Regs {
    pub const fn new(base: u64) -> Self {
        Self { base: base as usize }
    }
}

/// The register surface the driver logic is written against. A real device
/// backs it with volatile MMIO; a proof backs it with a modeled register file.
pub trait Mmio {
    fn read8(&self, off: usize) -> u8;
    fn write8(&self, off: usize, val: u8);
    fn read16(&self, off: usize) -> u16;
    fn write16(&self, off: usize, val: u16);
    fn read32(&self, off: usize) -> u32;
    fn write32(&self, off: usize, val: u32);
}

impl Mmio for Regs {
    fn read8(&self, off: usize) -> u8 {
        unsafe { core::ptr::read_volatile((self.base + off) as *const u8) }
    }
    fn write8(&self, off: usize, val: u8) {
        unsafe { core::ptr::write_volatile((self.base + off) as *mut u8, val) }
    }
    fn read16(&self, off: usize) -> u16 {
        unsafe { core::ptr::read_volatile((self.base + off) as *const u16) }
    }
    fn write16(&self, off: usize, val: u16) {
        unsafe { core::ptr::write_volatile((self.base + off) as *mut u16, val) }
    }
    fn read32(&self, off: usize) -> u32 {
        unsafe { core::ptr::read_volatile((self.base + off) as *const u32) }
    }
    fn write32(&self, off: usize, val: u32) {
        unsafe { core::ptr::write_volatile((self.base + off) as *mut u32, val) }
    }
}
