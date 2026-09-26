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

//! Walking the program header table of a parsed image.

use super::elf::Elf;
use super::phdr::Phdr;
use super::read::{u32v, u64v};

impl Elf<'_> {
    /// The header at `index`, or nothing when the table it is in does not fit
    /// an address.
    pub fn phdr(&self, index: u16) -> Option<Phdr> {
        let step = (index as u64).checked_mul(self.phentsize as u64)?;
        let at = usize::try_from(self.phoff.checked_add(step)?).ok()?;
        Some(Phdr {
            kind: u32v(self.bytes, at)?,
            flags: u32v(self.bytes, at + 4)?,
            offset: u64v(self.bytes, at + 8)?,
            vaddr: u64v(self.bytes, at + 16)?,
            filesz: u64v(self.bytes, at + 32)?,
            memsz: u64v(self.bytes, at + 40)?,
        })
    }

    pub fn phnum(&self) -> u16 {
        self.phnum
    }

    /// The address the program headers land on once the image is in place,
    /// which a dynamic linker needs and finds nowhere else.
    pub fn phdr_addr(&self, bias: u64) -> Option<u64> {
        for i in 0..self.phnum {
            let ph = self.phdr(i)?;
            if ph.kind != super::elf::PT_LOAD {
                continue;
            }
            let end = ph.offset.checked_add(ph.filesz)?;
            if self.phoff < ph.offset || self.phoff >= end {
                continue;
            }
            return ph.at(bias)?.checked_add(self.phoff - ph.offset);
        }
        None
    }
}
