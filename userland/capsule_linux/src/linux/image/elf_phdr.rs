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
    pub fn phdr(&self, index: u16) -> Option<Phdr> {
        let at = (self.phoff + index as u64 * self.phentsize as u64) as usize;
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

    /// The address the program headers land on once the image is in
    /// place, which a dynamic linker needs and finds nowhere else. It is
    /// the load segment that happens to contain them; an image whose
    /// headers are in no segment reports nothing rather than an address
    /// the guest cannot read.
    pub fn phdr_addr(&self, bias: u64) -> Option<u64> {
        for i in 0..self.phnum {
            let ph = self.phdr(i)?;
            if ph.kind != super::elf::PT_LOAD {
                continue;
            }
            if self.phoff >= ph.offset && self.phoff < ph.offset + ph.filesz {
                return Some(bias + ph.vaddr + (self.phoff - ph.offset));
            }
        }
        None
    }
}
