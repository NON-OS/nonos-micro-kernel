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


//! Bringing up a program and, when it asks for one, its interpreter.
//!
//! A dynamically linked executable does not start at its own entry: the
//! kernel of a real Linux system loads the interpreter named in
//! `PT_INTERP` beside it and starts there instead, and the interpreter
//! then maps the libraries and jumps to the program. This does the same,
//! with the store standing in for the filesystem.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::read_file;
use nonos_libc::mk_getpid;

use super::elf::{Elf, ET_DYN};
use super::load::load_at;
use super::loaded::{LoadError, Loaded};

/// Where a position independent executable goes, and where its
/// interpreter goes. Far apart so neither can grow into the other, and
/// both clear of the heap and the mapping area the guest gets.
const EXEC_BASE: u64 = 0x0000_4000_0000;
const INTERP_BASE: u64 = 0x0000_5000_0000;

/// The largest image that will be read out of the store.
const MAX_IMAGE: u32 = 64 << 20;

/// Load `bytes` as the program, and its interpreter if it names one.
/// Returns the program's own record and the address to start at, which is
/// the interpreter's entry whenever there is an interpreter.
pub fn program(
    guest: &crate::linux::guest::Guest,
    bytes: &[u8],
) -> Result<(Loaded, u64, u64), LoadError> {
    /*
     * A shared object is position independent and has to be told where
     * it went; an executable carries absolute addresses and must not be
     * moved. The file says which it is, so nothing upstream has to know
     * or can get it wrong.
     */
    let kind = Elf::parse(bytes).ok_or(LoadError::NotElf)?.kind;
    let bias = if kind == ET_DYN { EXEC_BASE } else { 0 };
    let image = load_at(guest, bytes, bias)?;
    let Some(path) = image.interp.clone() else {
        return Ok((image, image.entry, 0));
    };
    let raw = fetch(&path).ok_or(LoadError::Interp)?;
    let ld = load_at(guest, &raw, INTERP_BASE).map_err(|_| LoadError::Interp)?;
    Ok((image, ld.entry, INTERP_BASE))
}

fn fetch(path: &[u8]) -> Option<Vec<u8>> {
    read_file(mk_getpid() as u32, path, MAX_IMAGE).ok()
}
