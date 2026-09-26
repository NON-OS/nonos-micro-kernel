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

//! Reading, proving and placing a program's interpreter.

use alloc::vec::Vec;

use crate::linux::file::{key, store_read, visible};
use crate::linux::guest::{Guest, INTERP_BASE};

use super::load::load_at;
use super::loaded::LoadError;

/// The largest image that will be read out of the store.
const MAX_IMAGE: u32 = 64 << 20;

/// Where the interpreter's entry point ended up.
pub(super) fn place(guest: &mut Guest, path: &[u8]) -> Result<u64, LoadError> {
    // Already confined: the path came out of the guest's own image.
    let at = visible(b"/", path);
    let raw: Vec<u8> = store_read(&key(&at), MAX_IMAGE).map_err(|_| LoadError::Interp)?;
    if crate::linux::attest::verify(&at, &raw).is_err() {
        return Err(LoadError::Unproven);
    }
    let ld = load_at(guest, &raw, INTERP_BASE).map_err(|_| LoadError::Interp)?;
    Ok(ld.entry)
}
