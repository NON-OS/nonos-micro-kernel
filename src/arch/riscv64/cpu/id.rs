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

use core::arch::asm;

use crate::arch::riscv64::sbi::base as sbi_base;
use crate::arch::riscv64::sbi::SbiError;

pub fn cpu_id() -> usize {
    hart_id()
}

pub fn hart_id() -> usize {
    let id: usize;
    unsafe {
        asm!("mv {}, tp", out(reg) id, options(nomem, nostack, preserves_flags));
    }
    id
}

pub fn mvendorid() -> Result<usize, SbiError> {
    sbi_base::mvendorid()
}

pub fn marchid() -> Result<usize, SbiError> {
    sbi_base::marchid()
}

pub fn mimpid() -> Result<usize, SbiError> {
    sbi_base::mimpid()
}

#[derive(Debug, Clone, Copy)]
pub struct HartInfo {
    pub hart_id: usize,
    pub mvendorid: usize,
    pub marchid: usize,
    pub mimpid: usize,
}

impl HartInfo {
    pub fn current() -> Result<Self, SbiError> {
        Ok(Self {
            hart_id: hart_id(),
            mvendorid: mvendorid()?,
            marchid: marchid()?,
            mimpid: mimpid()?,
        })
    }

    pub fn vendor_name(&self) -> &'static str {
        match self.mvendorid {
            0x489 => "SiFive",
            0x5B7 => "Andes",
            0x61F => "T-Head",
            0x710 => "StarFive",
            _ => "Unknown",
        }
    }
}
pub fn is_primary_hart() -> bool {
    hart_id() == 0
}
