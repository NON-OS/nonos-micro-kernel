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

//! One load segment into a guest, and the interpreter a header names.

use alloc::vec::Vec;

use super::elf::{PF_W, PF_X};
use super::loaded::LoadError;
use super::phdr::Phdr;
use crate::linux::guest::Guest;

/// The interpreter path, without its terminator. A `PT_INTERP` that runs
/// off the end of the file names nothing.
pub(super) fn name(bytes: &[u8], ph: &Phdr) -> Option<Vec<u8>> {
    let raw = bytes.get(ph.file_range()?)?;
    let end = raw.iter().position(|b| *b == 0).unwrap_or(raw.len());
    Some(raw[..end].to_vec())
}

/// One segment: pages first, then the file bytes into them.
pub(super) fn segment(
    guest: &mut Guest,
    bytes: &[u8],
    ph: &Phdr,
    bias: u64,
) -> Result<(), LoadError> {
    let at = ph.at(bias).ok_or(LoadError::NotElf)?;
    if guest.map(at, ph.memsz, ph.flags & PF_W != 0, ph.flags & PF_X != 0) < 0 {
        return Err(LoadError::Map);
    }
    if ph.filesz == 0 {
        return Ok(());
    }
    let body = bytes.get(ph.file_range().ok_or(LoadError::NotElf)?).ok_or(LoadError::NotElf)?;
    if guest.write(at, body) < 0 {
        return Err(LoadError::Copy);
    }
    Ok(())
}
