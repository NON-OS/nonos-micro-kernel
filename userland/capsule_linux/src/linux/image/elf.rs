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

//! Enough ELF to find the program headers of a 64-bit little-endian
//! x86_64 image. Parsing lives here and not in the kernel: a malformed
//! header is this capsule's problem and nobody else's.

use super::read::{u16v, u64v};

/// A shared object, which is also what a position independent
/// executable and every dynamic linker is.
pub const ET_DYN: u16 = 3;

pub const PT_LOAD: u32 = 1;
pub const PT_INTERP: u32 = 3;
pub const PF_X: u32 = 1;
pub const PF_W: u32 = 2;

pub struct Elf<'a> {
    pub bytes: &'a [u8],
    pub kind: u16,
    pub entry: u64,
    pub phoff: u64,
    pub phentsize: u16,
    pub(super) phnum: u16,
}

impl<'a> Elf<'a> {
    /// Refuses anything that is not a 64-bit little-endian x86_64
    /// executable, because this personality can host nothing else.
    pub fn parse(bytes: &'a [u8]) -> Option<Elf<'a>> {
        if bytes.len() < 64 || &bytes[0..4] != b"\x7fELF" {
            return None;
        }
        if bytes[4] != 2 || bytes[5] != 1 || u16v(bytes, 18)? != 0x3E {
            return None;
        }
        Some(Elf {
            bytes,
            kind: u16v(bytes, 16)?,
            entry: u64v(bytes, 24)?,
            phoff: u64v(bytes, 32)?,
            phentsize: u16v(bytes, 54)?,
            phnum: u16v(bytes, 56)?,
        })
    }

}
