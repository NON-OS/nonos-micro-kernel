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


//! Laying an image's segments into a guest's address space.
//!
//! The kernel maps pages on request and copies bytes on request; which
//! pages and which bytes is the personality's business, because the
//! format is the personality's knowledge.

use super::elf::{Elf, ET_DYN, PF_W, PF_X, PT_INTERP, PT_LOAD};
use super::loaded::{LoadError, Loaded};
use super::segment::{name, segment};
use crate::linux::guest::Guest;

/// Map every `PT_LOAD` at `bias` and report what was learned. A shared
/// object is expected to be biased and an executable is not, so a caller
/// passing a bias for an `ET_EXEC` image is refused rather than moving an
/// image that carries absolute addresses.
pub fn load_at(guest: &Guest, bytes: &[u8], bias: u64) -> Result<Loaded, LoadError> {
    let elf = Elf::parse(bytes).ok_or(LoadError::NotElf)?;
    if bias != 0 && elf.kind != ET_DYN {
        return Err(LoadError::NotElf);
    }
    let mut interp = None;
    for i in 0..elf.phnum() {
        let Some(ph) = elf.phdr(i) else { continue };
        match ph.kind {
            PT_INTERP => interp = name(bytes, &ph),
            PT_LOAD if ph.memsz > 0 => segment(guest, bytes, &ph, bias)?,
            _ => {}
        }
    }
    Ok(Loaded {
        entry: elf.entry + bias,
        phdr: elf.phdr_addr(bias).unwrap_or(0),
        phentsize: elf.phentsize as u64,
        phnum: elf.phnum() as u64,
        interp,
        bias,
    })
}
