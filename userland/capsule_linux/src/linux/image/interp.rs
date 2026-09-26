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

use super::elf::{Elf, ET_DYN};
use super::load::load_at;
use super::loaded::{LoadError, Loaded};

pub use crate::linux::guest::{EXEC_BASE, INTERP_BASE};

/// Load `bytes` as the program, and its interpreter if it names one.
pub fn program(
    guest: &mut crate::linux::guest::Guest,
    bytes: &[u8],
) -> Result<(Loaded, u64, u64), LoadError> {
    /*
     * A shared object is position independent and has to be told where it
     * went; an executable carries absolute addresses and must not be moved.
     */
    let kind = Elf::parse(bytes).ok_or(LoadError::NotElf)?.kind;
    let bias = if kind == ET_DYN { EXEC_BASE } else { 0 };
    let image = load_at(guest, bytes, bias)?;
    let entry = image.entry;
    let Some(path) = image.interp.clone() else {
        return Ok((image, entry, 0));
    };
    let entry = super::interp_ld::place(guest, &path)?;
    Ok((image, entry, INTERP_BASE))
}
