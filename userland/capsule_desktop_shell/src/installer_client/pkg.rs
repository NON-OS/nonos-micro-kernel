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

//! Query and commit a signed .nonos package through the installer. Only the
//! path travels, never the bytes; the installer reads, parses and verifies the
//! package itself. The commit re-verifies against the digest the query
//! returned, so consent binds to the exact bytes the user was shown.

use alloc::vec::Vec;

use super::constants::{HDR_LEN, OP_PKG_COMMIT, OP_PKG_QUERY};
use super::pkg_decode::{decode_summary, PkgSummary};
use super::pkg_wire::{call, len_prefixed, ERR_INVALID, ERR_NOT_READY};

/// Longest path the installer's own `parse_path` accepts.
const MAX_PATH: usize = 255;

/// Widest summary the installer emits is 43 fixed bytes plus a 64 byte name and
/// a 64 byte namespace, so a whole reply always fits and never truncates into a
/// decode failure.
const QUERY_RX: usize = 256;

pub fn pkg_query(path: &[u8]) -> Result<PkgSummary, i32> {
    if path.is_empty() || path.len() > MAX_PATH {
        return Err(ERR_INVALID);
    }
    let mut rx = [0u8; QUERY_RX];
    let n = call(OP_PKG_QUERY, &len_prefixed(path), &mut rx)?;
    decode_summary(&rx[HDR_LEN..n]).ok_or(ERR_NOT_READY)
}

pub fn pkg_commit(path: &[u8], digest: &[u8; 32]) -> Result<(), i32> {
    if path.is_empty() || path.len() > MAX_PATH {
        return Err(ERR_INVALID);
    }
    let mut body = Vec::with_capacity(34 + path.len());
    body.extend_from_slice(digest);
    body.extend_from_slice(&len_prefixed(path));
    let mut rx = [0u8; 32];
    call(OP_PKG_COMMIT, &body, &mut rx).map(|_| ())
}
