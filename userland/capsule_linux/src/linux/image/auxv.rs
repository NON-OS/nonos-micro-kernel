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


//! The auxiliary vector: what the kernel tells a program about itself.
//!
//! A static binary can ignore most of this. A dynamic one cannot: the
//! interpreter finds the program's headers through AT_PHDR, learns where
//! it was itself placed from AT_BASE, and jumps to AT_ENTRY when it has
//! finished. AT_RANDOM is not optional either, because a C runtime takes
//! its stack guard from those sixteen bytes and reads them before it
//! runs anything.

use alloc::vec::Vec;

use super::loaded::Loaded;

pub const AT_NULL: u64 = 0;
pub const AT_PHDR: u64 = 3;
pub const AT_PHENT: u64 = 4;
pub const AT_PHNUM: u64 = 5;
pub const AT_PAGESZ: u64 = 6;
pub const AT_BASE: u64 = 7;
pub const AT_FLAGS: u64 = 8;
pub const AT_ENTRY: u64 = 9;
pub const AT_UID: u64 = 11;
pub const AT_EUID: u64 = 12;
pub const AT_GID: u64 = 13;
pub const AT_EGID: u64 = 14;
pub const AT_CLKTCK: u64 = 17;
pub const AT_SECURE: u64 = 23;
pub const AT_RANDOM: u64 = 25;
pub const AT_EXECFN: u64 = 31;

/// `interp_base` is zero for a static image, which is also what Linux
/// puts in AT_BASE when there is no interpreter.
pub fn pairs(image: &Loaded, interp_base: u64, random: u64, execfn: u64) -> Vec<u64> {
    let mut out = Vec::new();
    let mut put = |key: u64, value: u64| {
        out.push(key);
        out.push(value);
    };
    put(AT_PHDR, image.phdr);
    put(AT_PHENT, image.phentsize);
    put(AT_PHNUM, image.phnum);
    put(AT_PAGESZ, 4096);
    put(AT_BASE, interp_base);
    put(AT_FLAGS, 0);
    put(AT_ENTRY, image.entry);
    put(AT_UID, 0);
    put(AT_EUID, 0);
    put(AT_GID, 0);
    put(AT_EGID, 0);
    put(AT_CLKTCK, 100);
    put(AT_SECURE, 0);
    put(AT_RANDOM, random);
    put(AT_EXECFN, execfn);
    put(AT_NULL, 0);
    out
}
